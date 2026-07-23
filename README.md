# k4rust

`k4rust` is a lightweight, zero-overhead Rust FFI wrapper for writing high-performance kdb+ extensions and standalone clients. It bridges standard C-style `k.h` syntax with Rust's compile-time safety and automated RAII memory management.

---

## Intention and Philosophy

The core goal of `k4rust` is **minimal cognitive friction**. 

Writing kdb+ shared library extensions in C using `k.h` has a distinct functional style (e.g. `kJ(x)`, checking `x->t`, returning `krr("type")`). `k4rust` mirrors this coding style in Rust as closely as possible. 

By keeping the syntax and conventions nearly identical to the native C API, developers can trivially port existing C libraries to Rust, gaining memory safety guarantees, automatic ref-counting, and panic isolation without having to adopt a completely foreign coding style.

For full details on native C API functions, macros, and header definitions, refer to the official [KX C API Reference](https://code.kx.com/q/interfaces/c-client-for-q/#c-api-functions).

---

## Code Style Comparison: C vs Rust

Here is a side-by-side comparison of a vector addition function implemented in native C (using `k.h`) and in Rust (using `k4rust`):

### 1. In C (Using `k.h`)
```c
#include "k.h"

K add_vectors(K x, K y) {
    // 1. Type check
    if (x->t != KJ || y->t != KJ) return krr("type");
        
    // 2. Length check
    if (x->n != y->n) return krr("length"); 
        
    K r = ktn(KJ, x->n); // Allocate result vector
    
    // 3. Vector addition
    for (int i = 0; i < x->n; ++i) kJ(r)[i] = kJ(x)[i] + kJ(y)[i];
    
    return r;
}
```

### 2. In Rust (Using `k4rust`)
```rust
use k4rust::*;

k4rust_api! {
    pub fn add_vectors(x: K, y: K) -> K {
        // 1. Type check (mirrors C style)
        if x.t() != KJ || y.t() != KJ { return krr("type"); }
        
        // 2. Length check (mirrors C style)
        if x.n() != y.n() { return krr("length"); }
        
        let r = ktn(KJ, x.n()); // Allocate result vector
        
        // 3. Vector addition (zero-overhead safe slice matching)
        let (xs, ys, rs) = (x.kJ(), y.kJ(), r.kJ());
        
        for i in 0..rs.len() { rs[i] = xs[i] + ys[i]; }
        
        r // Hand ownership back to kdb+
    }
}
```

### Key Differences & Style Enhancements
* **Null Safety**: Accessing `.t()`, `.n()`, and `.len()` checks for null pointers automatically, preventing segmentation faults.
* **Early Exit Leak Prevention**: In C, exiting early with `krr` requires manually freeing any intermediate allocations. In `k4rust`, any local `K` objects in scope are automatically freed by Rust's RAII drop mechanism upon return.
* **Safe Slicing**: Slices like `x.kJ()` return standard Rust slices (`&[i64]`), giving you safe, bounds-checked element access at zero runtime cost.
* **Pure Rust Compilation**: `k4rust` implements the struct definitions and function bindings internally in Rust, removing the need for `bindgen`, a local C compiler, or a Clang toolchain build step.
* **Safe IPC Connection Handling**: The RAII-based `IpcClient` wrapper automatically manages client socket descriptors, ensuring connections are safely closed (`kclose`) when dropped and preventing socket/resource leaks.

---

## Memory Management & Reference Counting

While `k4rust` handles most memory management automatically, understanding how it maps to KDB+'s raw FFI is essential:

### 1. Automatic Memory Management (`r0`)
Under the hood, `K` implements the `Drop` trait. When a local `K` object (such as a temporary list or vector) goes out of scope, Rust automatically calls `r0` to decrement its reference count and release the memory. This guarantees that early exits (e.g., returning `krr("type")`) never leak memory.

### 2. Compile-Time Safe Reference Counting (`r1`)
When KDB+ calls your FFI function, it retains ownership of the input parameters (`x`, `y`). To ensure safety, `k4rust_api!` maps FFI inputs as read-only borrowed references (`&K`) instead of owned values.

If your function needs to return one of the input parameters directly, or store it in another container (like a dictionary or list), you must increment its reference count. In `k4rust`, it is idiomatic to call `r1(y)` (which wraps `y.clone()`):

```rust
pub fn return_input_if_valid(x: &K, y: &K) -> K {
    if x.n() <= 0 { return krr("length"); }
    r1(y) // Increments KDB reference count via r1, returning an owned K
}
```

If you attempt to return `y` directly without calling `r1(y)` or cloning, the compiler will catch it at compile-time (`Expected struct K, found &K`), preventing use-after-free or double-free runtime segfaults that are common in C extensions.

---

## Safe IPC Client (`IpcClient`)

In addition to writing library extensions loaded *inside* a `q` process, `k4rust` supports standalone client executables connecting *to* a remote `q` server. The `IpcClient` struct provides a type-safe, RAII-based connection manager that automatically closes socket descriptors when dropped:

```rust
use k4rust::{IpcClient, K, ki};

fn query_kdb() -> Result<(), String> {
    // Establish connection to local q process (port 50005)
    let client = IpcClient::connect("127.0.0.1", 50005)?;

    // Evaluate queries with 0-3 parameters
    let result: K = client.k2("{x+y}", ki(10), ki(20));
    println!("Result Type: {}, Value: {}", result.t(), result.i());

    // Evaluate queries with dynamic arity (e.g. 4 parameters known only at runtime)
    let args = vec![ki(10), ki(20), ki(30), ki(40)];
    let res: K = client.query("{[a;b;c;d] a+b+c+d}", args);
    println!("Dynamic Result: {}", res.i());

    Ok(())
} // client goes out of scope here; kclose(handle) is automatically called
```

### Dynamic Evaluation

Because Rust does not natively support safe C-style variadic arguments, `k4rust` provides explicit functions/methods for evaluations with up to 3 arguments (`k0` through `k3`). For queries that require 4 or more arguments, or where the number of arguments is determined dynamically at runtime, `IpcClient` provides `query(query, args)`. 

Under the hood, `query` packages the arguments into a mixed list (type `0`) and applies them to the query on the remote `q` process using the KDB+ dot (`.`) application operator.

---

## FFI Coverage: `k4rust` vs `k.h`

The table below outlines how native `k.h` macros and functions map to `k4rust` code style equivalents:

| Category | C Macro / Function (`k.h`) | `k4rust` Method Syntax | `k4rust` Free Function | Status / Notes |
|---|---|---|---|---|
| **Header Access** | `x->t` / `xt` | `x.t()` | N/A | Supported |
| | x->n / xn | x.n() / x.len() | N/A | Supported |
| | x->r / xr | x.r() | N/A | Supported |
| **Vector Slices** | `kB(x)` | `x.kB()` | `kB(x)` | Supported (`&[u8]`) |
| | `kG(x)` / `kC(x)` | `x.kG()` / `x.kC()` | `kG(x)` / `kC(x)` | Supported |
| | `kH(x)` | `x.kH()` | `kH(x)` | Supported (`&[i16]`) |
| | `kI(x)` | `x.kI()` | `kI(x)` | Supported (`&[i32]`) |
| | `kJ(x)` | `x.kJ()` | `kJ(x)` | Supported (`&[i64]`) |
| | `kE(x)` | `x.kE()` | `kE(x)` | Supported (`&[f32]`) |
| | `kF(x)` | `x.kF()` | `kF(x)` | Supported (`&[f64]`) |
| | `kS(x)` | `x.kS()` | `kS(x)` | Supported (`&[*mut c_char]`) |
| | `kK(x)` | `x.kK()` | `kK(x)` | Supported (`&[K]`) |
| | `kU(x)` | `x.kU()` | `kU(x)` | Supported GUID array |
| **Scalar Extractors** | `xg` | `x.g()` | `xg(x)` | Supported |
| | `xh` | `x.h()` | `xh(x)` | Supported |
| | `xi` | `x.i()` | `xi(x)` | Supported |
| | `xj` | `x.j()` | `xj(x)` | Supported |
| | `xe` | `x.e()` | `xe(x)` | Supported |
| | `xf` | `x.f()` | `xf(x)` | Supported |
| | `xs` | `x.s()` | `xs(x)` | Supported (`*mut c_char`) |
| **List Indexing** | `xx` / `xy` | `x.xx()` / `x.xy()` | `xx(x)` / `xy(x)` | Supported |
| **Constructors** | `kb(x)` / `kg(x)` / `kh(x)` | N/A | `kb(x)` / `kg(x)` / `kh(x)` | Supported |
| | `ki(x)` / `kj(x)` / `ke(x)` / `kf(x)` | N/A | `ki(x)` / `kj(x)` / `ke(x)` / `kf(x)` | Supported |
| | `kp(s)` / `kpn(s, n)` | N/A | `kp(s)` | Supported (takes standard Rust `&str`) |
| | `ks(s)` | N/A | `ks(s)` | Supported (takes raw `S`); use `ss("sym")` to intern |
| | `kd(x)` / `kz(x)` / `kt(x)` / `ktj(x)` | N/A | `kd(x)` / `kz(x)` / `kt(x)` / `ktj(t, x)` | Supported temporal constructors |
| | `ku(uuid)` | N/A | `ku(uuid)` | Supported (GUID/UUID) |
| | `ktn(type, len)` | N/A | `ktn(type, len)` | Supported |
| | `krr(error)` | N/A | `krr(error)` | Supported (thread-safe error cache) |
| **Memory Management** | `r0(x)` | Handled via drop | N/A | Supported (automatic `Drop`) |
| | `r1(x)` | N/A | `r1(x)` | Supported (safe wrapper, calls `x.clone()`) |
| **List Appenders** | `ja(x, ptr)` / `js(x, s)` | N/A | `ja(x, ptr)` / `js(x, s)` | Supported (appends element in-place) |
| | `jk(x, y)` / `jv(x, y)` | N/A | `jk(x, y)` / `jv(x, y)` | Supported (appends list/value in-place) |
| **Event Loop** | `sd1(d, f)` | N/A | `sd1(d, f)` | Supported |
| | `sd0(d)` / `sd0x(d, f)` | N/A | `sd0(d)` / `sd0x(d, f)` | Supported |
| **IPC / Connection**| `khp` / `khpu` / `khpun` / `khpunc`| `IpcClient::connect(h, p)` / `IpcClient::builder(h, p)` | `khp` / `khpu` / `khpun` / `khpunc` | Supported |
| | `kclose` | N/A | `kclose` | Supported |
| | `okx` | N/A | `okx` | Supported |
| **Tables / Dicts** | `xD(x, y)` | N/A | `xD(x, y)` | Supported Dict constructor |
| | `xT(x)` | N/A | `xT(x)` | Supported Table constructor |
| | `ktd(x)` | N/A | `ktd(x)` | Supported Table-to-Dict converter |
| **Evaluations** | `k(h, q, ...)` | `client.k0(q)` to `client.k3(q, x, y, z)` / `client.query(q, args)` | `k0(h, q)` to `k3(h, q, x, y, z)` | Supported (arity 0-3 / dynamic arity via list packaging) |

---

## The `k4rust_api!` Macro

The `k4rust_api!` declarative macro is a **required key concept** for all functions exported to kdb+. It handles the translation layer between Rust's safe abstractions and kdb+'s raw C FFI boundary:

1. **FFI Signature Generation**: Adds `#[unsafe(no_mangle)]` and converts signatures to `pub unsafe extern "C" fn` using raw pointers (`*mut ffi::k0`).
2. **Automatic Parameter Wrapping**: Wraps input arguments inside `std::mem::ManuallyDrop` to prevent Rust's compiler from dropping them prematurely when the function finishes (kdb+ retains ownership of parameters).
3. **Panic Catching**: Catches any Rust panic that occurs inside your function body and returns a clean `krr("panic")` to KDB+, preventing a Rust panic from crashing the entire host `q` database process.

---

## FFI Ownership Gotchas

### Scenario 1: Mixed List Packaging
When creating a mixed list (type 0) containing input arguments, the developer must remember to clone the inputs to increment their reference counts:
```rust
k4rust_api! {
    pub fn package_results(x: K, y: K) -> K {
        let res = ktn(0, 2);
        let l = res.kK();
        l[0] = r1(x); // Use safe r1 wrapper (or x.clone())
        l[1] = r1(y); // Use safe r1 wrapper (or y.clone())
        res
    }
}
```
* **Compile-Time Safety & C Idioms**: Since `K`'s inner pointer is private, a developer cannot write `K(x.0)` or bypass the reference counting. The compiler guarantees that assignment to a list element requires either `r1(x)` or `x.clone()`, preventing double-free segmentation faults at compile-time.

### Scenario 2: Mutating Shared Vectors In-Place
In kdb+, lists are shared reference-counted objects. If you clone a handle `let mut cloned_x = x.clone();`, you have duplicated the handle (reference count is now `> 1`), but the underlying data is still shared.

If you then modify it:
```rust
let slice = cloned_x.kJ();
slice[0] = 999; 
```
* **Safe Copy-On-Write**: To prevent mutating shared data, a developer can call `cloned_x.make_mut()` before retrieving the slice. `make_mut()` automatically checks if the reference count is `> 1`, duplicating the underlying vector data in place if it is shared, fully maintaining KDB+'s value semantics.

---

## Documentation & References

* [KX C API Reference](https://code.kx.com/q/interfaces/c-client-for-q/#c-api-functions)
* [KX Data Types Reference](https://code.kx.com/q/basics/datatypes/)

