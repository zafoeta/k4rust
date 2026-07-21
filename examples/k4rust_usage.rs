use k4rust::*;

// Expose FFI functions to KDB+ using the k4rust_api! macro
k4rust_api! {
    /// Adds two long vector lists together, demonstrating type checking,
    /// length validation, allocation, and slice manipulation.
    pub fn add_vectors_example(x: K, y: K) -> K {
        // 1. Type validation
        if x.t() != KJ || y.t() != KJ { return krr("type"); }
        
        // 2. Length validation
        if x.n() != y.n() { return krr("length"); }

        // 3. Allocate a new KDB+ long vector of size x.n()
        let res = ktn(KJ, x.n());

        // 4. Retrieve mutable slices for zero-overhead element access
        let (xs, ys, rs) = (x.kJ(), y.kJ(), res.kJ());

        // 5. Perform the vectorized addition
        const NJ: i64 = i64::MIN; // KDB+ long null value
        for i in 0..rs.len() {
            let xv = xs[i];
            let yv = ys[i];
            
            // Check for nulls and wrap on overflow
            if xv == NJ || yv == NJ {
                rs[i] = NJ;
            } else {
                rs[i] = xv.wrapping_add(yv);
            }
        }

        // Return the owned result (k4rust_api! handles into_raw() automatically)
        res
    }

    /// Multiplies a float vector list by a scalar float factor.
    pub fn scale_floats_example(x: K, y: K) -> K {
        if x.t() != KF || y.t() != -KF { return krr("type"); }

        let factor = xf(y);
        let res = ktn(KF, x.n());

        let (xs, rs) = (x.kF(), res.kF());

        for i in 0..rs.len() { rs[i] = xs[i] * factor; }

        res
    }
}
