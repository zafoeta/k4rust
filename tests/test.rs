use k4rust::*;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

#[link(name = "kdb_client", kind = "static")]
unsafe extern "C" {}

#[test]
fn test_nulls_and_infinities() {
    assert_eq!(NH, i16::MIN);
    assert_eq!(WH, i16::MAX);
    assert_eq!(NI, i32::MIN);
    assert_eq!(WI, i32::MAX);
    assert_eq!(NJ, i64::MIN);
    assert_eq!(WJ, i64::MAX);
    assert!(NE.is_nan());
    assert!(NF.is_nan());
    assert!(WE.is_infinite());
    assert!(WF.is_infinite());
    assert_eq!(NC, b' ');

    assert_eq!(UU, 2);
    assert_eq!(KP, 12);
    assert_eq!(KM, 13);
    assert_eq!(KD, 14);
    assert_eq!(KZ, 15);
    assert_eq!(KN, 16);
    assert_eq!(KU, 17);
    assert_eq!(KV, 18);
    assert_eq!(KT, 19);
    assert_eq!(XT, 98);
    assert_eq!(XD, 99);
}

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
        // Test builder with credentials and k2 (2 parameters)
        let client = IpcClient::builder("127.0.0.1", 50005)
            .creds("username:password")
            .connect()
            .expect("IpcClient builder with credentials failed");
        let res2 = client.k2("{x+y}", ki(10), ki(20));
        assert_eq!(res2.t(), -KI, "Result type of int addition should be int (-6)");
        assert_eq!(res2.i(), 30, "Result value of 10+20 should be 30");
        assert!(!client.okx(&res2));
    }

    {
        // Test builder with credentials + timeout and k3 (3 parameters)
        let client = IpcClient::builder("127.0.0.1", 50005)
            .creds("username:password")
            .timeout(2000)
            .connect()
            .expect("IpcClient builder with credentials and timeout failed");
        let res3 = client.k3("{x+y+z}", ki(1), ki(2), ki(3));
        assert_eq!(res3.t(), -KI, "Result type of int addition should be int (-6)");
        assert_eq!(res3.i(), 6, "Result value of 1+2+3 should be 6");
    }

    {
        // Test builder with credentials + timeout + capability and k1 (1 parameter)
        let client = IpcClient::builder("127.0.0.1", 50005)
            .creds("username:password")
            .timeout(2000)
            .capability(1)
            .connect()
            .expect("IpcClient builder with credentials, timeout, and capability failed");
        let res4 = client.k1("{(x)}", ki(100));
        assert_eq!(res4.t(), -KI, "Result type of identity should be int (-6)");
        assert_eq!(res4.i(), 100, "Result value should be 100");
    }

    {
        // Test query (dynamic arity query with 4 parameters)
        let c = IpcClient::connect("127.0.0.1", 50005)
            .expect("IpcClient::connect failed");
        let a = vec![ki(10), ki(20), ki(30), ki(40)];
        let r = c.query("{[a;b;c;d] a+b+c+d}", a);
        assert_eq!(r.t(), -KI, "Result type of dynamic arity query should be int (-6)");
        assert_eq!(r.i(), 100, "Result value should be 100");

        // Test K.r() method (reference count tracking)
        assert_eq!(r.r(), 0, "Initial reference count should be 0");
        {
            let cloned = r.clone();
            assert_eq!(r.r(), 1, "Reference count after clone should be 1");
            assert_eq!(cloned.r(), 1, "Reference count of clone should be 1");
        } // cloned dropped here (decrementing reference count)
        assert_eq!(r.r(), 0, "Reference count after dropping the clone should be 0");
    }

    // 8. Terminate q process
    let _ = child.kill();
}

#[test]
fn test_scalar_constructors_and_extractors() {
    let b = kb(1);
    assert_eq!(b.t(), -KB);
    assert_eq!(b.g(), 1);
    assert_eq!(xg(&b), 1);

    let g = kg(2);
    assert_eq!(g.t(), -KG);
    assert_eq!(g.g(), 2);
    assert_eq!(xg(&g), 2);

    let h = kh(3);
    assert_eq!(h.t(), -KH);
    assert_eq!(h.h(), 3);
    assert_eq!(xh(&h), 3);

    let i = ki(4);
    assert_eq!(i.t(), -KI);
    assert_eq!(i.i(), 4);
    assert_eq!(xi(&i), 4);

    let j = kj(5);
    assert_eq!(j.t(), -KJ);
    assert_eq!(j.j(), 5);
    assert_eq!(xj(&j), 5);

    let e = ke(6.0);
    assert_eq!(e.t(), -KE);
    assert_eq!(e.e(), 6.0);
    assert_eq!(xe(&e), 6.0);

    let f = kf(7.0);
    assert_eq!(f.t(), -KF);
    assert_eq!(f.f(), 7.0);
    assert_eq!(xf(&f), 7.0);

    let p = kp("hello");
    assert_eq!(p.t(), KC);
    assert_eq!(p.len(), 5);
    assert_eq!(p.kC(), b"hello");

    let guid = ku([1; 16]);
    assert_eq!(guid.t(), -UU);
}

#[test]
fn test_vector_allocation_and_slicing() {
    let vec_b = ktn(KB, 3);
    assert_eq!(vec_b.t(), KB);
    assert_eq!(vec_b.len(), 3);
    vec_b.kB()[0] = 1;
    vec_b.kB()[1] = 0;
    vec_b.kB()[2] = 1;
    assert_eq!(vec_b.kB(), &[1, 0, 1]);

    let vec_g = ktn(KG, 2);
    assert_eq!(vec_g.t(), KG);
    vec_g.kG()[0] = 42;
    vec_g.kG()[1] = 99;
    assert_eq!(vec_g.kG(), &[42, 99]);

    let vec_h = ktn(KH, 2);
    assert_eq!(vec_h.t(), KH);
    vec_h.kH()[0] = 1000;
    vec_h.kH()[1] = NH;
    assert_eq!(vec_h.kH(), &[1000, NH]);

    let vec_i = ktn(KI, 2);
    assert_eq!(vec_i.t(), KI);
    vec_i.kI()[0] = 200000;
    vec_i.kI()[1] = NI;
    assert_eq!(vec_i.kI(), &[200000, NI]);

    let vec_j = ktn(KJ, 2);
    assert_eq!(vec_j.t(), KJ);
    vec_j.kJ()[0] = 5000000000;
    vec_j.kJ()[1] = NJ;
    assert_eq!(vec_j.kJ(), &[5000000000, NJ]);

    let vec_e = ktn(KE, 2);
    assert_eq!(vec_e.t(), KE);
    vec_e.kE()[0] = 1.5;
    vec_e.kE()[1] = 2.5;
    assert_eq!(vec_e.kE(), &[1.5, 2.5]);

    let vec_f = ktn(KF, 2);
    assert_eq!(vec_f.t(), KF);
    vec_f.kF()[0] = 10.5;
    vec_f.kF()[1] = 20.5;
    assert_eq!(vec_f.kF(), &[10.5, 20.5]);

    let vec_c = ktn(KC, 4);
    assert_eq!(vec_c.t(), KC);
    vec_c.kC()[0] = b't';
    vec_c.kC()[1] = b'e';
    vec_c.kC()[2] = b's';
    vec_c.kC()[3] = b't';
    assert_eq!(vec_c.kC(), b"test");

    let vec_s = ktn(KS, 2);
    assert_eq!(vec_s.t(), KS);
    let s1 = ss("sym1");
    let s2 = ss("sym2");
    vec_s.kS()[0] = s1;
    vec_s.kS()[1] = s2;
    assert_eq!(vec_s.kS()[0], s1);
    assert_eq!(vec_s.kS()[1], s2);

    let vec_k = ktn(0, 2);
    assert_eq!(vec_k.t(), 0);
    vec_k.kK()[0] = ki(10);
    vec_k.kK()[1] = kf(20.5);
    assert_eq!(vec_k.kK()[0].i(), 10);
    assert_eq!(vec_k.kK()[1].f(), 20.5);
}

#[test]
fn test_clone_and_drop_refcount() {
    let original = ki(1234);
    assert_eq!(original.r(), 0);

    {
        let cloned = original.clone();
        assert_eq!(original.r(), 1);
        assert_eq!(cloned.r(), 1);
    } // cloned dropped here

    assert_eq!(original.r(), 0);
}

#[test]
fn test_cow_semantics_via_make_mut() {
    let vec = ktn(KI, 3);
    vec.kI()[0] = 10;
    vec.kI()[1] = 20;
    vec.kI()[2] = 30;

    let mut cloned = vec.clone();
    assert_eq!(vec.r(), 1);
    assert_eq!(cloned.r(), 1);

    // Call make_mut on cloned, which should perform a deep copy
    cloned.make_mut();

    // Reference counts should go back to 0 as they are no longer shared
    assert_eq!(vec.r(), 0);
    assert_eq!(cloned.r(), 0);

    // Modify cloned, original should remain unchanged
    cloned.kI()[0] = 999;
    assert_eq!(vec.kI()[0], 10);
    assert_eq!(cloned.kI()[0], 999);
}

#[test]
fn test_dicts_and_tables() {
    let keys = ktn(KS, 2);
    let s_a = ss("a");
    let s_b = ss("b");
    keys.kS()[0] = s_a;
    keys.kS()[1] = s_b;

    let vals = ktn(KI, 2);
    vals.kI()[0] = 100;
    vals.kI()[1] = 200;

    let dict = xD(keys, vals);
    assert_eq!(dict.t(), XD);

    let table = xT(dict.clone());
    assert_eq!(table.t(), XT);

    let decoded_dict = ktd(table);
    assert_eq!(decoded_dict.t(), XT);
}

#[test]
fn test_list_appenders() {
    // 1. ja (append scalar to list)
    let mut list_i = ktn(KI, 1);
    list_i.kI()[0] = 10;
    let mut val = 20i32;
    ja(&mut list_i, &mut val as *mut i32 as *mut _);
    assert_eq!(list_i.len(), 2);
    assert_eq!(list_i.kI(), &[10, 20]);

    // 2. js (append symbol to KS list)
    let mut list_s = ktn(KS, 1);
    let sym_a = ss("a");
    let sym_b = ss("b");
    list_s.kS()[0] = sym_a;
    js(&mut list_s, sym_b);
    assert_eq!(list_s.len(), 2);
    assert_eq!(list_s.kS()[0], sym_a);
    assert_eq!(list_s.kS()[1], sym_b);

    // 3. jk (append K to mixed list)
    let mut list_m = ktn(0, 1);
    list_m.kK()[0] = ki(100);
    jk(&mut list_m, kf(200.5));
    assert_eq!(list_m.len(), 2);
    assert_eq!(list_m.kK()[0].i(), 100);
    assert_eq!(list_m.kK()[1].f(), 200.5);

    // 4. jv (append list to list)
    let mut list1 = ktn(KI, 2);
    list1.kI()[0] = 1;
    list1.kI()[1] = 2;
    let list2 = ktn(KI, 2);
    list2.kI()[0] = 3;
    list2.kI()[1] = 4;
    jv(&mut list1, list2);
    assert_eq!(list1.len(), 4);
    assert_eq!(list1.kI(), &[1, 2, 3, 4]);
}

#[test]
fn test_null_k_safety() {
    let null_k = K::null();

    // All accessors should return safe defaults, not segfault
    assert_eq!(null_k.t(), 0);
    assert_eq!(null_k.n(), 0);
    assert_eq!(null_k.r(), 0);
    assert_eq!(null_k.g(), 0);
    assert_eq!(null_k.h(), 0);
    assert_eq!(null_k.i(), 0);
    assert_eq!(null_k.j(), 0);
    assert_eq!(null_k.e(), 0.0);
    assert_eq!(null_k.f(), 0.0);
    assert!(null_k.s().is_null());
    assert_eq!(null_k.len(), 0);

    // Slice accessors should return empty slices
    assert!(null_k.kJ().is_empty());
    assert!(null_k.kI().is_empty());
    assert!(null_k.kF().is_empty());
    assert!(null_k.kC().is_empty());
    assert!(null_k.kG().is_empty());
    assert!(null_k.kH().is_empty());
    assert!(null_k.kE().is_empty());
    assert!(null_k.kS().is_empty());
    assert!(null_k.kB().is_empty());

    // is_null / is_error
    assert!(null_k.is_null());
    assert!(!null_k.is_error());
}

#[test]
fn test_is_null_and_is_error() {
    let normal = ki(42);
    assert!(!normal.is_null());
    assert!(!normal.is_error());

    let null = K::null();
    assert!(null.is_null());
    assert!(!null.is_error());

    let err = krr("test");
    assert!(err.is_null()); // krr returns a null-inner K
    assert!(!err.is_error()); // the null pointer means t() == 0, not -128
}

#[test]
fn test_debug_formatting() {
    let null_k = K::null();
    assert_eq!(format!("{:?}", null_k), "K(null)");

    let int_k = ki(42);
    assert_eq!(format!("{:?}", int_k), "K(int: 42)");

    let long_k = kj(100);
    assert_eq!(format!("{:?}", long_k), "K(long: 100)");

    let float_k = kf(3.14);
    assert_eq!(format!("{:?}", float_k), "K(float: 3.14)");

    let bool_k = kb(1);
    assert_eq!(format!("{:?}", bool_k), "K(bool: true)");

    let vec_k = ktn(KI, 3);
    assert_eq!(format!("{:?}", vec_k), "K(vector type=6 len=3)");

    let mixed_k = ktn(0, 2);
    assert_eq!(format!("{:?}", mixed_k), "K(mixed list len=2)");

    let dict = xD(ktn(KS, 0), ktn(KI, 0));
    assert_eq!(format!("{:?}", dict), "K(dict)");
}

#[test]
#[should_panic(expected = "n() called on an atom")]
fn test_atom_n_panic() {
    let f = kf(1.0);
    let _ = f.n();
}

#[test]
#[should_panic(expected = "Slicing method called on an atom")]
fn test_atom_slice_panic() {
    let f = kf(1.0);
    let _ = f.kF();
}

#[test]
#[should_panic(expected = "n() called on a table")]
fn test_table_n_panic() {
    let keys = ktn(KS, 1);
    keys.kS()[0] = ss("col1");
    let col = ktn(KI, 5);
    let vals = ktn(0, 1);
    vals.kK()[0] = col;
    let table = xT(xD(keys, vals));
    let _ = table.n();
}

#[test]
#[should_panic(expected = "Slicing method called on a table")]
fn test_table_slice_panic() {
    let keys = ktn(KS, 1);
    keys.kS()[0] = ss("col1");
    let col = ktn(KI, 5);
    let vals = ktn(0, 1);
    vals.kK()[0] = col;
    let table = xT(xD(keys, vals));
    let _ = table.kK();
}


#[test]
fn test_ffi_integration() {
    // 1. Compile the FFI example in release mode so it's ready to be loaded by q
    let build_status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--example")
        .arg("k4rust_test_ffi")
        .status()
        .expect("Failed to build k4rust_test_ffi");
    assert!(build_status.success(), "Failed to compile FFI test library");

    // 2. Spawn Q process running tests/test.q
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let qhome = manifest_dir.join("../..").join("kx");
    let test_q_path = manifest_dir.join("tests").join("test.q");

    let status = Command::new("q")
        .env("QHOME", qhome)
        .arg(test_q_path)
        .status()
        .expect("Failed to run Q test script");

    assert!(status.success(), "Q FFI integration tests failed");
}
