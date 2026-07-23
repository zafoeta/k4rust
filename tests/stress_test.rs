use k4rust::*;

#[link(name = "kdb_client", kind = "static")]
unsafe extern "C" {}

k4rust_api! {
    fn panic_trigger_api(x: K) -> K {
        if x.i() > 0 {
            panic!("Intentional stress panic!");
        }
        ki(42)
    }
}

#[test]
fn stress_test_refcount_and_memory_leaks() {
    println!("[STRESS] Starting 1,000,000 iterations of allocation, cloning, dropping, and duplication...");
    
    for i in 0..1_000_000 {
        let k_list = ktn(KJ, 100);
        let slice = k_list.kJ();
        slice[0] = i;
        slice[99] = i * 2;
        
        // Initial refcount for new K object is 0
        assert_eq!(k_list.r(), 0);
        
        // Clone (r1 increments from 0 to 1)
        let k_clone = k_list.clone();
        assert_eq!(k_list.r(), 1);
        assert_eq!(k_clone.r(), 1);
        
        // Duplicate (deep copy, brand new object with r=0)
        let k_dup = k_list.duplicate();
        assert_eq!(k_dup.r(), 0);
        assert_eq!(k_dup.n(), 100);
        
        // Make mut (triggers copy-on-write because refcount > 0)
        let mut k_mut = k_list;
        k_mut.make_mut();
        assert_eq!(k_mut.r(), 0);
    }
    
    println!("[STRESS] Refcount and memory leak test passed cleanly!");
}

#[test]
fn stress_test_nested_dicts_and_tables() {
    println!("[STRESS] Starting 50,000 iterations of nested Table and Dict operations...");
    
    for _ in 0..50_000 {
        let sym_keys = ktn(KS, 3);
        js(&mut sym_keys.clone(), ss("time"));
        js(&mut sym_keys.clone(), ss("sym"));
        js(&mut sym_keys.clone(), ss("price"));
        
        let col1 = ktn(KP, 10);
        let col2 = ktn(KS, 10);
        let col3 = ktn(KF, 10);
        
        let values = ktn(0, 3);
        values.kK()[0] = col1;
        values.kK()[1] = col2;
        values.kK()[2] = col3;
        
        let dict = xD(sym_keys, values);
        let table = xT(dict);
        
        assert_eq!(table.t(), XT);
        
        let cloned_table = table.clone();
        assert_eq!(cloned_table.r(), 1);
        
        let duplicated_table = table.duplicate();
        assert_eq!(duplicated_table.r(), 0);
        assert_eq!(duplicated_table.t(), XT);
    }
    
    println!("[STRESS] Nested structures test passed cleanly!");
}

#[test]
fn stress_test_panic_safety_under_load() {
    println!("[STRESS] Starting 100,000 iterations of panic handling across k4rust_api! FFI boundary...");
    
    for i in 0..100_000 {
        let input = ki(if i % 2 == 0 { 1 } else { 0 });
        let raw_res = unsafe { panic_trigger_api(input.into_raw()) };
        let res = unsafe { K::__from_raw(raw_res) };
        
        if i % 2 == 0 {
            assert!(res.is_null() || res.is_error(), "Expected error/null result on panic");
        } else {
            assert!(!res.is_error());
            assert_eq!(res.i(), 42);
        }
    }
    
    println!("[STRESS] Panic safety stress test passed cleanly!");
}

#[test]
fn stress_test_large_vector_allocations() {
    println!("[STRESS] Allocating large vectors (10,000,000 elements)...");
    
    let n = 10_000_000;
    let big_vec = ktn(KF, n);
    assert_eq!(big_vec.n(), n);
    
    let slice = big_vec.kF();
    slice[0] = 3.14159;
    slice[n as usize - 1] = 2.71828;
    
    assert_eq!(slice[0], 3.14159);
    assert_eq!(slice[n as usize - 1], 2.71828);
    
    let dup = big_vec.duplicate();
    assert_eq!(dup.n(), n);
    assert_eq!(dup.kF()[0], 3.14159);
    
    println!("[STRESS] Large vector allocation test passed cleanly!");
}
