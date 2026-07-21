use k4rust::*;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

#[link(name = "kdb_client", kind = "static")]
unsafe extern "C" {}

#[test]
fn test_live_ipc() {
    // 1. Spawn a local q server on port 50005
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let qhome = manifest_dir.join("../..").join("kx");
    let mut child = Command::new("q")
        .env("QHOME", qhome)
        .arg("-p")
        .arg("50005")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start q process");

    // Wait a brief moment for the q process to bind to the port
    sleep(Duration::from_millis(500));

    // 2. Connect to localhost:50005 using the real khp function
    let handle = khp("127.0.0.1", 50005);
    
    // Ensure we connected successfully
    if handle <= 0 {
        let _ = child.kill();
        panic!("Failed to connect to q on port 50005, handle={}", handle);
    }

    // 3. Send query and assert result
    let res = k0(handle, "2+2");
    assert_eq!(res.t(), -KJ, "Result type should be long (-7)");
    assert_eq!(res.j(), 4, "Result value should be 4");

    // 4. Verify okx behaves correctly (returns 0 or 1 without crash)
    let _ok_val = okx(&res);

    // 5. Close socket connection
    kclose(handle);

    // 6. Test connection failure case
    let fail_conn = IpcClient::connect("127.0.0.1", 50099);
    assert!(fail_conn.is_err(), "Connection to closed port should return Err");

    // 7. Test safe IpcClient wrapper connection variants
    {
        // Test connect_with_creds and k2 (2 parameters)
        let client = IpcClient::connect_with_creds("127.0.0.1", 50005, "username:password")
            .expect("IpcClient::connect_with_creds failed");
        let res2 = client.k2("{x+y}", ki(10), ki(20));
        assert_eq!(res2.t(), -KI, "Result type of int addition should be int (-6)");
        assert_eq!(res2.i(), 30, "Result value of 10+20 should be 30");
        assert!(!client.okx(&res2));
    }

    {
        // Test connect_with_timeout and k3 (3 parameters)
        let client = IpcClient::connect_with_timeout("127.0.0.1", 50005, "username:password", 2000)
            .expect("IpcClient::connect_with_timeout failed");
        let res3 = client.k3("{x+y+z}", ki(1), ki(2), ki(3));
        assert_eq!(res3.t(), -KI, "Result type of int addition should be int (-6)");
        assert_eq!(res3.i(), 6, "Result value of 1+2+3 should be 6");
    }

    {
        // Test connect_with_capability and k1 (1 parameter)
        let client = IpcClient::connect_with_capability("127.0.0.1", 50005, "username:password", 2000, 1)
            .expect("IpcClient::connect_with_capability failed");
        let res4 = client.k1("{(x)}", ki(100));
        assert_eq!(res4.t(), -KI, "Result type of identity should be int (-6)");
        assert_eq!(res4.i(), 100, "Result value should be 100");
    }

    {
        // Test k_eval_dynamic (dynamic arity query with 4 parameters)
        let client = IpcClient::connect("127.0.0.1", 50005)
            .expect("IpcClient::connect failed");
        let args = vec![ki(10), ki(20), ki(30), ki(40)];
        let res5 = client.k_eval_dynamic("{[a;b;c;d] a+b+c+d}", args);
        assert_eq!(res5.t(), -KI, "Result type of dynamic arity query should be int (-6)");
        assert_eq!(res5.i(), 100, "Result value should be 100");

        // Test K.r() method (reference count tracking)
        assert_eq!(res5.r(), 0, "Initial reference count should be 0");
        {
            let cloned = res5.clone();
            assert_eq!(res5.r(), 1, "Reference count after clone should be 1");
            assert_eq!(cloned.r(), 1, "Reference count of clone should be 1");
        } // cloned dropped here (decrementing reference count)
        assert_eq!(res5.r(), 0, "Reference count after dropping the clone should be 0");
    }

    // 8. Terminate q process
    let _ = child.kill();
}
