use k4rust::*;

// Expose FFI functions to KDB+ using the k4rust_api! macro
k4rust_api! {
    /// Adds two long vector lists together.
    pub fn add_vectors(x: K, y: K) -> K {
        if x.t() != KJ || y.t() != KJ { return krr("type"); }
        if x.n() != y.n() { return krr("length"); }

        let res = ktn(KJ, x.n());
        // NOTE: Hoisting slice references (x.kJ(), y.kJ(), res.kJ()) before loops
        // that write to a K buffer is critical for performance. Without this,
        // LLVM cannot prove that writes to `rs[i]` do not invalidate the raw
        // pointers wrapped inside `x` and `y`, disabling SIMD auto-vectorization
        // and causing a ~3-4x performance penalty.
        let (xs, ys, rs) = (x.kJ(), y.kJ(), res.kJ());
        const NJ: i64 = i64::MIN;
        for i in 0..rs.len() {
            let xv = xs[i];
            let yv = ys[i];
            rs[i] = if xv == NJ || yv == NJ { NJ } else { xv.wrapping_add(yv) };
        }
        res
    }

    /// Multiplies a float vector list by a scalar float factor.
    pub fn scale_floats(x: K, y: K) -> K {
        if x.t() != KF || y.t() != -KF { return krr("type"); }

        let factor = xf(y);
        let res = ktn(KF, x.n());
        // Hoist slice references before the write loop to ensure SIMD auto-vectorization
        let (xs, rs) = (x.kF(), res.kF());
        for i in 0..rs.len() { rs[i] = xs[i] * factor; }
        res
    }

    /// Counts occurrences of a target symbol in a symbol vector.
    /// Read-only loop — no hoisting needed (writes to local scalar only).
    pub fn count_symbol(x: K, target: K) -> K {
        if x.t() != KS || target.t() != -KS { return krr("type"); }

        let t = xs(target);
        let mut count = 0i32;
        for i in 0..x.n() as usize {
            if x.kS()[i] == t { count += 1; }
        }
        ki(count)
    }
}
