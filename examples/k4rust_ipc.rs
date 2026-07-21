use k4rust::{IpcClient, ki};

// We must link target kdb_client library statically.
#[link(name = "kdb_client", kind = "static")]
unsafe extern "C" {}

fn main() -> Result<(), String> {
    println!("Connecting to KDB+ process at 127.0.0.1:50005...");

    // 1. Establish connection to local KDB+ process.
    // Ensure you have a q process running on port 50005: `q -p 50005`
    let c = match IpcClient::connect("127.0.0.1", 50005) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Connection failed: {e}. Please ensure `q -p 50005` is running.");
            return Err(e);
        }
    };
    println!("Successfully connected! (handle = {})", c.handle());

    // 2. Evaluate queries with 0 to 3 parameters.
    println!("\n--- Standard Evaluations ---");
    
    // Evaluation with 0 arguments (k0)
    let r0 = c.k0("2+2");
    println!("k0(\"2+2\"): {:?}", r0);

    // Evaluation with 1 argument (k1)
    let r1 = c.k1("{x * 2}", ki(21));
    println!("k1(\"{{x * 2}}\", 21): {:?}", r1);

    // Evaluation with 2 arguments (k2)
    let r2 = c.k2("{x + y}", ki(10), ki(20));
    println!("k2(\"{{x + y}}\", 10, 20): {:?}", r2);

    // Evaluation with 3 arguments (k3)
    let r3 = c.k3("{x + y + z}", ki(1), ki(2), ki(3));
    println!("k3(\"{{x + y + z}}\", 1, 2, 3): {:?}", r3);

    // 3. Evaluate queries with dynamic arity (query).
    // For queries with 4 or more arguments (or when arity is known only at runtime).
    println!("\n--- Dynamic Evaluations ---");
    let a = vec![ki(10), ki(20), ki(30), ki(40)];
    let rd = c.query("{[a;b;c;d] a+b+c+d}", a);
    println!("query(sum 10 20 30 40): {:?}", rd);

    // 4. Working with results
    println!("\n--- Extracting Values ---");
    if !rd.is_null() && !rd.is_error() {
        println!("Type code: {}", rd.t());
        println!("Value: {}", rd.i());
    }

    // 5. Handling Errors
    println!("\n--- Error Handling ---");
    let re = c.k0("invalid_variable_name");
    // Since k0 sets error, we can check it
    if re.is_null() {
        println!("Query failed as expected (null K returned).");
    } else {
        println!("Query result: {:?}", re);
    }

    println!("\nIPC example run complete.");
    Ok(())
}
