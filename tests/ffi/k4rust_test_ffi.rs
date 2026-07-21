use k4rust::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;
use std::sync::Mutex;

unsafe extern "C" {
    fn read(fd: i32, buf: *mut std::os::raw::c_void, count: usize) -> isize;
    fn write(fd: i32, buf: *const std::os::raw::c_void, count: usize) -> isize;
    fn close(fd: i32) -> i32;
}

static SOCKET_1: Mutex<Option<UnixStream>> = Mutex::new(None);
static SOCKET_2: Mutex<Option<UnixStream>> = Mutex::new(None);
static CALLED_COUNT: Mutex<i32> = Mutex::new(0);
static LAST_READ_DATA: Mutex<Vec<u8>> = Mutex::new(Vec::new());

unsafe extern "C" fn socket_callback(fd: i32) -> *mut ffi::k0 {
    println!("Rust: socket_callback triggered on fd={}", fd);
    let mut buf = [0u8; 1024];
    let n = unsafe { read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };
    println!("Rust: read returned={}", n);
    if n > 0 {
        let mut count = CALLED_COUNT.lock().unwrap();
        *count += 1;
        let mut data = LAST_READ_DATA.lock().unwrap();
        data.extend_from_slice(&buf[..n as usize]);
    }
    if n <= 0 {
        std::ptr::null_mut()
    } else {
        kb(0).into_raw()
    }
}


// --- Helper Functions ---
// Helpers write to Vec (not K), so inline K reads are safe — no aliasing.

#[inline(always)]
fn to_f64_vec(k: &K) -> Vec<f64> {
    let n = k.n() as usize;
    let mut out = vec![0.0f64; n];
    match k.t() {
        1 => for i in 0..n { out[i] = k.kB()[i] as f64; },
        4 => for i in 0..n { out[i] = k.kG()[i] as f64; },
        5 => for i in 0..n { out[i] = if k.kH()[i] == i16::MIN { f64::NAN } else { k.kH()[i] as f64 }; },
        6 => for i in 0..n { out[i] = if k.kI()[i] == i32::MIN { f64::NAN } else { k.kI()[i] as f64 }; },
        7 => for i in 0..n { out[i] = if k.kJ()[i] == i64::MIN { f64::NAN } else { k.kJ()[i] as f64 }; },
        8 => for i in 0..n { out[i] = k.kE()[i] as f64; },
        9 => for i in 0..n { out[i] = k.kF()[i]; },
        _ => {}
    }
    out
}

#[inline(always)]
fn to_f32_vec(k: &K) -> Vec<f32> {
    let n = k.n() as usize;
    let mut out = vec![0.0f32; n];
    match k.t() {
        1 => for i in 0..n { out[i] = k.kB()[i] as f32; },
        4 => for i in 0..n { out[i] = k.kG()[i] as f32; },
        5 => for i in 0..n { out[i] = if k.kH()[i] == i16::MIN { f32::NAN } else { k.kH()[i] as f32 }; },
        6 => for i in 0..n { out[i] = if k.kI()[i] == i32::MIN { f32::NAN } else { k.kI()[i] as f32 }; },
        7 => for i in 0..n { out[i] = if k.kJ()[i] == i64::MIN { f32::NAN } else { k.kJ()[i] as f32 }; },
        8 => for i in 0..n { out[i] = k.kE()[i]; },
        9 => for i in 0..n { out[i] = k.kF()[i] as f32; },
        _ => {}
    }
    out
}

#[inline(always)]
fn to_i64_vec(k: &K) -> Vec<i64> {
    let n = k.n() as usize;
    let mut out = vec![0i64; n];
    match k.t() {
        1 => for i in 0..n { out[i] = k.kB()[i] as i64; },
        4 => for i in 0..n { out[i] = k.kG()[i] as i64; },
        5 => for i in 0..n { out[i] = if k.kH()[i] == i16::MIN { i64::MIN } else { k.kH()[i] as i64 }; },
        6 => for i in 0..n { out[i] = if k.kI()[i] == i32::MIN { i64::MIN } else { k.kI()[i] as i64 }; },
        7 => for i in 0..n { out[i] = k.kJ()[i]; },
        _ => {}
    }
    out
}

#[inline(always)]
fn to_i32_vec(k: &K) -> Vec<i32> {
    let n = k.n() as usize;
    let mut out = vec![0i32; n];
    match k.t() {
        1 => for i in 0..n { out[i] = k.kB()[i] as i32; },
        4 => for i in 0..n { out[i] = k.kG()[i] as i32; },
        5 => for i in 0..n { out[i] = if k.kH()[i] == i16::MIN { i32::MIN } else { k.kH()[i] as i32 }; },
        6 => for i in 0..n { out[i] = k.kI()[i]; },
        _ => {}
    }
    out
}

#[inline(always)]
fn to_i16_vec(k: &K) -> Vec<i16> {
    let n = k.n() as usize;
    let mut out = vec![0i16; n];
    match k.t() {
        1 => for i in 0..n { out[i] = k.kB()[i] as i16; },
        4 => for i in 0..n { out[i] = k.kG()[i] as i16; },
        5 => for i in 0..n { out[i] = k.kH()[i]; },
        _ => {}
    }
    out
}

// --- Mixed-type addition helpers ---
// All write to K result buffers → hoist slices before loops.

fn add_mixed_floats(x: &K, y: &K) -> K {
    let res = ktn(KF, x.n());
    let rs = res.kF();
    if x.t() == 7 && y.t() == 9 {
        let (xv, yv) = (x.kJ(), y.kF());
        for i in 0..rs.len() {
            let xi = xv[i];
            let yi = yv[i];
            rs[i] = if xi == i64::MIN || yi.is_nan() { f64::NAN } else { (xi as f64) + yi };
        }
    } else if x.t() == 9 && y.t() == 7 {
        let (xv, yv) = (x.kF(), y.kJ());
        for i in 0..rs.len() {
            let xi = xv[i];
            let yi = yv[i];
            rs[i] = if xi.is_nan() || yi == i64::MIN { f64::NAN } else { xi + (yi as f64) };
        }
    } else {
        let xv = to_f64_vec(x);
        let yv = to_f64_vec(y);
        for i in 0..rs.len() {
            rs[i] = xv[i] + yv[i];
        }
    }
    res
}

fn add_mixed_reals(x: &K, y: &K) -> K {
    let xv = to_f32_vec(x);
    let yv = to_f32_vec(y);
    let res = ktn(KE, x.n());
    let rs = res.kE();
    for i in 0..rs.len() {
        rs[i] = xv[i] + yv[i];
    }
    res
}

fn add_mixed_longs(x: &K, y: &K) -> K {
    let res = ktn(KJ, x.n());
    let rs = res.kJ();
    if x.t() == 6 && y.t() == 7 {
        let (xv, yv) = (x.kI(), y.kJ());
        for i in 0..rs.len() {
            let xi = xv[i];
            let yi = yv[i];
            rs[i] = if xi == i32::MIN || yi == i64::MIN { i64::MIN } else { (xi as i64).wrapping_add(yi) };
        }
    } else if x.t() == 7 && y.t() == 6 {
        let (xv, yv) = (x.kJ(), y.kI());
        for i in 0..rs.len() {
            let xi = xv[i];
            let yi = yv[i];
            rs[i] = if xi == i64::MIN || yi == i32::MIN { i64::MIN } else { xi.wrapping_add(yi as i64) };
        }
    } else {
        let xv = to_i64_vec(x);
        let yv = to_i64_vec(y);
        for i in 0..rs.len() {
            let xi = xv[i];
            let yi = yv[i];
            rs[i] = if xi == i64::MIN || yi == i64::MIN { i64::MIN } else { xi.wrapping_add(yi) };
        }
    }
    res
}

fn add_mixed_ints(x: &K, y: &K) -> K {
    let xv = to_i32_vec(x);
    let yv = to_i32_vec(y);
    let res = ktn(KI, x.n());
    let rs = res.kI();
    for i in 0..rs.len() {
        let xi = xv[i];
        let yi = yv[i];
        rs[i] = if xi == i32::MIN || yi == i32::MIN { i32::MIN } else { xi.wrapping_add(yi) };
    }
    res
}

fn add_mixed_shorts(x: &K, y: &K) -> K {
    let res = ktn(KH, x.n());
    let rs = res.kH();
    if x.t() == 5 && y.t() == 4 {
        let (xv, yv) = (x.kH(), y.kG());
        for i in 0..rs.len() {
            let xi = xv[i];
            let yi = yv[i];
            rs[i] = if xi == i16::MIN { i16::MIN } else { xi.wrapping_add(yi as i16) };
        }
    } else if x.t() == 4 && y.t() == 5 {
        let (xv, yv) = (x.kG(), y.kH());
        for i in 0..rs.len() {
            let xi = xv[i];
            let yi = yv[i];
            rs[i] = if yi == i16::MIN { i16::MIN } else { (xi as i16).wrapping_add(yi) };
        }
    } else {
        let xv = to_i16_vec(x);
        let yv = to_i16_vec(y);
        for i in 0..rs.len() {
            let xi = xv[i];
            let yi = yv[i];
            rs[i] = if xi == i16::MIN || yi == i16::MIN { i16::MIN } else { xi.wrapping_add(yi) };
        }
    }
    res
}

fn add_mixed_bytes(x: &K, y: &K) -> K {
    let res = ktn(KG, x.n());
    let (xb, yb, rs) = (x.kB(), y.kB(), res.kG());
    for i in 0..rs.len() {
        rs[i] = (xb[i].wrapping_add(yb[i])) as u8;
    }
    res
}

fn add_mixed_bools(x: &K, y: &K) -> K {
    let res = ktn(KB, x.n());
    let (xb, yb, rs) = (x.kB(), y.kB(), res.kB());
    for i in 0..rs.len() {
        rs[i] = if (xb[i] != 0) ^ (yb[i] != 0) { 1 } else { 0 };
    }
    res
}

// --- FFI API Surface ---

k4rust_api! {
    // 1. Long Vector addition (KJ / Type 7)
    pub fn add_vectors(x: K, y: K) -> K {
        if x.t() != KJ || y.t() != KJ { return krr("type"); }
        if x.n() != y.n() { return krr("length"); }

        let res = ktn(KJ, x.n());
        let (xs, ys, rs) = (x.kJ(), y.kJ(), res.kJ());
        const NJ: i64 = i64::MIN;
        for i in 0..rs.len() {
            let xi = xs[i];
            let yi = ys[i];
            rs[i] = if xi == NJ || yi == NJ { NJ } else { xi.wrapping_add(yi) };
        }
        res
    }

    // 2. Scaled float vector multiplication (KF / Type 9 * -KF)
    pub fn scale_floats(x: K, y: K) -> K {
        if x.t() != KF || y.t() != -KF { return krr("type"); }

        let factor = xf(y);
        let res = ktn(KF, x.n());
        let (xs, rs) = (x.kF(), res.kF());
        for i in 0..rs.len() {
            rs[i] = xs[i] * factor;
        }
        res
    }

    // 3. Filter int vector > y (KI / Type 6)
    // Read-only count loop uses inline; write loop hoists slices.
    pub fn filter_greater(x: K, y: K) -> K {
        if x.t() != KI || y.t() != -KI { return krr("type"); }

        let xs = x.kI();
        let limit = xi(y);

        let mut count = 0i64;
        for i in 0..xs.len() {
            if xs[i] > limit { count += 1; }
        }

        let res = ktn(KI, count);
        let rs = res.kI();
        let mut idx = 0usize;
        for i in 0..xs.len() {
            if xs[i] > limit {
                rs[idx] = xs[i];
                idx += 1;
            }
        }
        res
    }

    // 4. Count character occurrences in string (KC / Type 10)
    // Read-only — inline is fine.
    pub fn count_char(s: K, c: K) -> K {
        if s.t() != KC || c.t() != -KC { return krr("type"); }

        let ch = xg(c);
        let mut count = 0i32;
        for i in 0..s.n() as usize {
            if s.kC()[i] == ch { count += 1; }
        }
        ki(count)
    }

    // 5. Retrieve a vector from a mixed list and sum it (Mixed / Type 0)
    // Read-only — inline is fine.
    pub fn sum_mixed(x: K, y: K) -> K {
        if x.t() != 0 || y.t() != -KI { return krr("type"); }

        let idx = xi(y) as usize;
        if idx >= x.n() as usize { return krr("index"); }

        let sub = &kK(x)[idx];
        if sub.t() != KJ { return krr("type"); }

        let mut total = 0i64;
        for i in 0..sub.n() as usize {
            total = total.wrapping_add(sub.kJ()[i]);
        }
        kj(total)
    }

    // 6. Vector dot product (KF * KF -> KF)
    // Read-only — inline is fine.
    pub fn dot_product(x: K, y: K) -> K {
        if x.t() != KF || y.t() != x.t() { return krr("type"); }
        if x.n() != y.n() { return krr("length"); }

        let mut total = 0.0;
        for i in 0..x.n() as usize {
            total += x.kF()[i] * y.kF()[i];
        }
        kf(total)
    }

    // 7. Boolean mask filtering (KJ where KB -> KJ)
    pub fn filter_mask(x: K, mask: K) -> K {
        if x.t() != KJ || mask.t() != KB { return krr("type"); }
        if x.n() != mask.n() { return krr("length"); }

        let (xs, ms) = (x.kJ(), mask.kB());

        let mut count = 0i64;
        for i in 0..xs.len() {
            if ms[i] != 0 { count += 1; }
        }

        let res = ktn(KJ, count);
        let rs = res.kJ();
        let mut idx = 0usize;
        for i in 0..xs.len() {
            if ms[i] != 0 {
                rs[idx] = xs[i];
                idx += 1;
            }
        }
        res
    }

    // 8. Count occurrences of target symbol (KS where KS -> KI)
    // Read-only — inline is fine.
    pub fn count_symbol(x: K, target: K) -> K {
        if x.t() != KS || target.t() != -KS { return krr("type"); }

        let t = xs(target);
        let mut count = 0i32;
        for i in 0..x.n() as usize {
            if x.kS()[i] == t { count += 1; }
        }
        ki(count)
    }

    // 9. Add mixed numeric vectors with upcasting promotion
    pub fn add_mixed(x: K, y: K) -> K {
        let xt = x.t();
        let yt = y.t();
        if x.n() != y.n() { return krr("length"); }

        let ct = match (xt, yt) {
            (1..=9, 1..=9) => {
                if xt == 2 || xt == 3 || yt == 2 || yt == 3 {
                    return krr("type");
                }
                std::cmp::max(xt, yt)
            }
            _ => return krr("type"),
        };

        match ct {
            9 => add_mixed_floats(x, y),
            8 => add_mixed_reals(x, y),
            7 => add_mixed_longs(x, y),
            6 => add_mixed_ints(x, y),
            5 => add_mixed_shorts(x, y),
            4 => add_mixed_bytes(x, y),
            1 => add_mixed_bools(x, y),
            _ => krr("type"),
        }
    }

    // 10. Generate a long vector of size n containing 0..n
    pub fn generate_vector(n: K) -> K {
        let t = n.t();
        let sz = if t == -6 {
            xi(n) as i64
        } else if t == -7 {
            xj(n)
        } else {
            return krr("type");
        };
        if sz < 0 { return krr("length"); }

        let res = ktn(KJ, sz);
        let rs = res.kJ();
        for i in 0..rs.len() {
            rs[i] = i as i64;
        }
        res
    }

    // 11. Package results in a mixed list
    // No loop — inline is fine.
    pub fn package_results(x: K, y: K) -> K {
        let res = ktn(0, 2);
        res.kK()[0] = x.clone();
        res.kK()[1] = y.clone();
        res
    }

    // 12. Test early exit error leak prevention
    pub fn test_leak_krr(x: K) -> K {
        let _res = ktn(KJ, xj(x));
        krr("test_krr_error")
    }

    // 13. Trigger standard rust panic to test FFI boundary wrapping
    pub fn trigger_panic(_x: K) -> K {
        panic!("intentional rust panic to test catch_unwind in k4rust");
    }

    // 14. Verify new API signatures and constructors compile
    pub fn test_new_apis(x: K) -> K {
        let guid = ku([0; 16]);
        let _guids = kU(&guid);
        let _guid_free = kU(&guid);

        let ml = ktn(0, 2);
        let _first = xx(&ml);
        let _second = xy(&ml);

        let _date = kd(20000101);
        let _datetime = kz(1.5);
        let _time = kt(120000);
        let _timestamp = ktj(KP, 1234567890);

        let sym = ss("test_symbol");
        let _s = ks(sym);

        let keys = ktn(KS, 1);
        let vals = ktn(KI, 1);
        let dict = xD(keys, vals);
        let _tbl = xT(dict.clone());
        let _simple_tbl = ktd(dict);

        let _ser = b9(1, &x);
        let _de = d9(&x);

        let _enc = ymd(2020, 1, 1);
        let _dec = dj(_enc);

        let mut li = ktn(KI, 0);
        ja(&mut li, &123 as *const i32 as *mut _);
        let mut ls = ktn(KS, 0);
        js(&mut ls, sym);
        let mut lm = ktn(0, 0);
        jk(&mut lm, ki(456));
        let mut li2 = ktn(KI, 0);
        jv(&mut li2, ktn(KI, 2));

        let _e0 = k0(0, "1+1");
        let _e1 = k1(0, "{(x)}", ki(5));
        let _e2 = k2(0, "{x+y}", ki(1), ki(2));
        let _e3 = k3(0, "{x+y+z}", ki(1), ki(2), ki(3));

        x.clone()
    }

    // FFI entry point: Test b9 serialization
    pub fn test_b9(mode: K, x: K) -> K {
        b9(xi(mode), &x)
    }

    // FFI entry point: Test d9 deserialization
    pub fn test_d9(x: K) -> K {
        d9(&x)
    }

    // FFI entry point: Test sd1 setup and descriptor registration
    pub fn test_sd_setup(_x: K) -> K {
        let (s1, s2) = UnixStream::pair().unwrap();
        s1.set_nonblocking(true).unwrap();
        s2.set_nonblocking(true).unwrap();
        let fd1 = s1.as_raw_fd();
        let fd2 = s2.as_raw_fd();

        println!("Rust: fd1={}, fd2={}", fd1, fd2);

        *SOCKET_1.lock().unwrap() = Some(s1);
        *SOCKET_2.lock().unwrap() = Some(s2);
        *CALLED_COUNT.lock().unwrap() = 0;
        LAST_READ_DATA.lock().unwrap().clear();

        let _r = sd1(fd1, Some(socket_callback));
        println!("Rust: sd1 returned = {:?}", _r.as_raw());

        // No loop — inline is fine.
        let res = ktn(KI, 2);
        res.kI()[0] = fd1;
        res.kI()[1] = fd2;
        res
    }

    // FFI entry point: Test sd write
    pub fn test_sd_write(fd: K, data: K) -> K {
        let fv = xi(fd);
        if data.t() != KC { return krr("type"); }
        let b = kC(data);
        let w = unsafe { write(fv, b.as_ptr() as *const _, b.len()) };
        kb(if w > 0 { 1 } else { 0 })
    }

    // FFI entry point: Test sd status retrieval
    pub fn test_sd_status(_x: K) -> K {
        let count = *CALLED_COUNT.lock().unwrap();
        let data = LAST_READ_DATA.lock().unwrap().clone();

        // No loop — inline is fine.
        let res = ktn(0, 2);
        res.kK()[0] = ki(count);

        let sk = ktn(KC, data.len() as i64);
        let sb = sk.kC();
        sb[..data.len()].copy_from_slice(&data);
        res.kK()[1] = sk;
        res
    }

    // FFI entry point: Test sd0 and sd0x unregistration and close
    pub fn test_sd_close(fd: K, mode: K) -> K {
        let fv = xi(fd);
        let mv = xi(mode);
        if mv == 1 {
            sd0x(fv, 1);
            if fv == SOCKET_1.lock().unwrap().as_ref().map(|s| s.as_raw_fd()).unwrap_or(-1) {
                let _ = SOCKET_1.lock().unwrap().take();
            } else if fv == SOCKET_2.lock().unwrap().as_ref().map(|s| s.as_raw_fd()).unwrap_or(-1) {
                let _ = SOCKET_2.lock().unwrap().take();
            }
        } else {
            sd0(fv);
            unsafe { close(fv); }
            if fv == SOCKET_1.lock().unwrap().as_ref().map(|s| s.as_raw_fd()).unwrap_or(-1) {
                let _ = SOCKET_1.lock().unwrap().take();
            } else if fv == SOCKET_2.lock().unwrap().as_ref().map(|s| s.as_raw_fd()).unwrap_or(-1) {
                let _ = SOCKET_2.lock().unwrap().take();
            }
        }
        kb(1)
    }
}
