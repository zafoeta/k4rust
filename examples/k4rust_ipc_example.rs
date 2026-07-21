use k4rust::{IpcClient, ki};

// We must link target kdb_client library statically.
#[link(name = "kdb_client", kind = "static")]
unsafe extern "C" {}

fn main() -> Result<(), String> {
    println!("Connecting to KDB+ process at 127.0.0.1:50005...");

    // 1. Establish connection to local KDB+ process.
    // Ensure you have a q process running on port 50005: `q -p 50005`
    let client = match IpcClient::connect("127.0.0.1", 50005) {
        Ok(c) => c,
        Err(e) => {
            println!("Connection failed: {e}. Please ensure `q -p 50005` is running.");
            return Err(e);
        }
    };
    println!("Successfully connected! (handle = {})", client.handle());

    // 2. Evaluate queries with 0 to 3 parameters.
    println!("\n--- Standard Evaluations ---");
    
    // Evaluation with 0 arguments (k0)
    let res0 = client.k0("2+2");
    println!("k0(\"2+2\"): {:?}", res0);

    // Evaluation with 1 argument (k1)
    let res1 = client.k1("{x * 2}", ki(21));
    println!("k1(\"{{x * 2}}\", 21): {:?}", res1);

    // Evaluation with 2 arguments (k2)
    let res2 = client.k2("{x + y}", ki(10), ki(20));
    println!("k2(\"{{x + y}}\", 10, 20): {:?}", res2);

    // Evaluation with 3 arguments (k3)
    let res3 = client.k3("{x + y + z}", ki(1), ki(2), ki(3));
    println!("k3(\"{{x + y + z}}\", 1, 2, 3): {:?}", res3);

    // 3. Evaluate queries with dynamic arity (k_eval_dynamic).
    // For queries with 4 or more arguments (or when arity is known only at runtime).
    println!("\n--- Dynamic Evaluations ---");
    let args = vec![ki(10), ki(20), ki(30), ki(40)];
    let res_dyn = client.k_eval_dynamic("{[a;b;c;d] a+b+c+d}", args);
    println!("k_eval_dynamic(sum 10 20 30 40): {:?}", res_dyn);

    // 4. Working with results
    println!("\n--- Extracting Values ---");
    if !res_dyn.is_null() && !res_dyn.is_error() {
        println!("Type code: {}", res_dyn.t());
        println!("Value: {}", res_dyn.i());
    }

    // 5. Handling Errors
    println!("\n--- Error Handling ---");
    let res_err = client.k0("invalid_variable_name");
    // Since k0 sets error, we can check it
    if res_err.is_null() {
        println!("Query failed as expected (null K returned).");
    } else {
        println!("Query result: {:?}", res_err);
    }

    println!("\nIPC example run complete.");
    Ok(())
}
