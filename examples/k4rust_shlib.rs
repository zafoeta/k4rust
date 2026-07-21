use k4rust::*;

// Expose FFI functions to KDB+ using the k4rust_api! macro
k4rust_api! {
    /// Adds two long vector lists together.
    pub fn add_vectors(x: K, y: K) -> K {
        if x.t() != KJ || y.t() != KJ { return krr("type"); }
        if x.n() != y.n() { return krr("length"); }

        let res = ktn(KJ, x.n());
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
        let (xs, rs) = (x.kF(), res.kF());
        for i in 0..rs.len() { rs[i] = xs[i] * factor; }
        res
    }
}
