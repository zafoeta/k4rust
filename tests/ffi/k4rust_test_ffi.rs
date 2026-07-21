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
    // Return NULL to unregister if connection closed/failed, otherwise return kb(0) to keep monitored
    if n <= 0 {
        std::ptr::null_mut()
    } else {
        kb(0).into_raw()
    }
}


// --- Helper Functions ---

#[inline(always)]
fn to_f64_vec(k: &K) -> Vec<f64> {
    let n = k.n() as usize;
    let mut out = vec![0.0f64; n];
    match k.t() {
        1 => { let s = k.kB(); for i in 0..n { out[i] = s[i] as f64; } }
        4 => { let s = k.kG(); for i in 0..n { out[i] = s[i] as f64; } }
        5 => { let s = k.kH(); for i in 0..n { out[i] = if s[i] == i16::MIN { f64::NAN } else { s[i] as f64 }; } }
        6 => { let s = k.kI(); for i in 0..n { out[i] = if s[i] == i32::MIN { f64::NAN } else { s[i] as f64 }; } }
        7 => { let s = k.kJ(); for i in 0..n { out[i] = if s[i] == i64::MIN { f64::NAN } else { s[i] as f64 }; } }
        8 => { let s = k.kE(); for i in 0..n { out[i] = s[i] as f64; } }
        9 => { let s = k.kF(); for i in 0..n { out[i] = s[i]; } }
        _ => {}
    }
    out
}

#[inline(always)]
fn to_f32_vec(k: &K) -> Vec<f32> {
    let n = k.n() as usize;
    let mut out = vec![0.0f32; n];
    match k.t() {
        1 => { let s = k.kB(); for i in 0..n { out[i] = s[i] as f32; } }
        4 => { let s = k.kG(); for i in 0..n { out[i] = s[i] as f32; } }
        5 => { let s = k.kH(); for i in 0..n { out[i] = if s[i] == i16::MIN { f32::NAN } else { s[i] as f32 }; } }
        6 => { let s = k.kI(); for i in 0..n { out[i] = if s[i] == i32::MIN { f32::NAN } else { s[i] as f32 }; } }
        7 => { let s = k.kJ(); for i in 0..n { out[i] = if s[i] == i64::MIN { f32::NAN } else { s[i] as f32 }; } }
        8 => { let s = k.kE(); for i in 0..n { out[i] = s[i]; } }
        9 => { let s = k.kF(); for i in 0..n { out[i] = s[i] as f32; } }
        _ => {}
    }
    out
}

#[inline(always)]
fn to_i64_vec(k: &K) -> Vec<i64> {
    let n = k.n() as usize;
    let mut out = vec![0i64; n];
    match k.t() {
        1 => { let s = k.kB(); for i in 0..n { out[i] = s[i] as i64; } }
        4 => { let s = k.kG(); for i in 0..n { out[i] = s[i] as i64; } }
        5 => { let s = k.kH(); for i in 0..n { out[i] = if s[i] == i16::MIN { i64::MIN } else { s[i] as i64 }; } }
        6 => { let s = k.kI(); for i in 0..n { out[i] = if s[i] == i32::MIN { i64::MIN } else { s[i] as i64 }; } }
        7 => { let s = k.kJ(); for i in 0..n { out[i] = s[i]; } }
        _ => {}
    }
    out
}

#[inline(always)]
fn to_i32_vec(k: &K) -> Vec<i32> {
    let n = k.n() as usize;
    let mut out = vec![0i32; n];
    match k.t() {
        1 => { let s = k.kB(); for i in 0..n { out[i] = s[i] as i32; } }
        4 => { let s = k.kG(); for i in 0..n { out[i] = s[i] as i32; } }
        5 => { let s = k.kH(); for i in 0..n { out[i] = if s[i] == i16::MIN { i32::MIN } else { s[i] as i32 }; } }
        6 => { let s = k.kI(); for i in 0..n { out[i] = s[i]; } }
        _ => {}
    }
    out
}

#[inline(always)]
fn to_i16_vec(k: &K) -> Vec<i16> {
    let n = k.n() as usize;
    let mut out = vec![0i16; n];
    match k.t() {
        1 => { let s = k.kB(); for i in 0..n { out[i] = s[i] as i16; } }
        4 => { let s = k.kG(); for i in 0..n { out[i] = s[i] as i16; } }
        5 => { let s = k.kH(); for i in 0..n { out[i] = s[i]; } }
        _ => {}
    }
    out
}

fn add_mixed_floats(x: &K, y: &K) -> K {
    let res = ktn(KF, x.n());
    let res_floats = res.kF();
    if x.t() == 7 && y.t() == 9 {
        let (xv, yv) = (x.kJ(), y.kF());
        for i in 0..res_floats.len() {
            let xi = xv[i];
            let yi = yv[i];
            res_floats[i] = if xi == i64::MIN || yi.is_nan() { f64::NAN } else { (xi as f64) + yi };
        }
    } else if x.t() == 9 && y.t() == 7 {
        let (xv, yv) = (x.kF(), y.kJ());
        for i in 0..res_floats.len() {
            let xi = xv[i];
            let yi = yv[i];
            res_floats[i] = if xi.is_nan() || yi == i64::MIN { f64::NAN } else { xi + (yi as f64) };
        }
    } else {
        let xv = to_f64_vec(x);
        let yv = to_f64_vec(y);
        for i in 0..res_floats.len() {
            res_floats[i] = xv[i] + yv[i];
        }
    }
    res
}

fn add_mixed_reals(x: &K, y: &K) -> K {
    let xv = to_f32_vec(x);
    let yv = to_f32_vec(y);
    let res = ktn(KE, x.n());
    let res_reals = res.kE();
    for i in 0..res_reals.len() {
        res_reals[i] = xv[i] + yv[i];
    }
    res
}

fn add_mixed_longs(x: &K, y: &K) -> K {
    let res = ktn(KJ, x.n());
    let res_longs = res.kJ();
    if x.t() == 6 && y.t() == 7 {
        let (xv, yv) = (x.kI(), y.kJ());
        for i in 0..res_longs.len() {
            let xi = xv[i];
            let yi = yv[i];
            res_longs[i] = if xi == i32::MIN || yi == i64::MIN { i64::MIN } else { (xi as i64).wrapping_add(yi) };
        }
    } else if x.t() == 7 && y.t() == 6 {
        let (xv, yv) = (x.kJ(), y.kI());
        for i in 0..res_longs.len() {
            let xi = xv[i];
            let yi = yv[i];
            res_longs[i] = if xi == i64::MIN || yi == i32::MIN { i64::MIN } else { xi.wrapping_add(yi as i64) };
        }
    } else {
        let xv = to_i64_vec(x);
        let yv = to_i64_vec(y);
        for i in 0..res_longs.len() {
            let xi = xv[i];
            let yi = yv[i];
            res_longs[i] = if xi == i64::MIN || yi == i64::MIN { i64::MIN } else { xi.wrapping_add(yi) };
        }
    }
    res
}

fn add_mixed_ints(x: &K, y: &K) -> K {
    let xv = to_i32_vec(x);
    let yv = to_i32_vec(y);
    let res = ktn(KI, x.n());
    let res_ints = res.kI();
    for i in 0..res_ints.len() {
        let xi = xv[i];
        let yi = yv[i];
        res_ints[i] = if xi == i32::MIN || yi == i32::MIN { i32::MIN } else { xi.wrapping_add(yi) };
    }
    res
}

fn add_mixed_shorts(x: &K, y: &K) -> K {
    let res = ktn(KH, x.n());
    let res_shorts = res.kH();
    if x.t() == 5 && y.t() == 4 {
        let (xv, yv) = (x.kH(), y.kG());
        for i in 0..res_shorts.len() {
            let xi = xv[i];
            let yi = yv[i];
            res_shorts[i] = if xi == i16::MIN { i16::MIN } else { xi.wrapping_add(yi as i16) };
        }
    } else if x.t() == 4 && y.t() == 5 {
        let (xv, yv) = (x.kG(), y.kH());
        for i in 0..res_shorts.len() {
            let xi = xv[i];
            let yi = yv[i];
            res_shorts[i] = if yi == i16::MIN { i16::MIN } else { (xi as i16).wrapping_add(yi) };
        }
    } else {
        let xv = to_i16_vec(x);
        let yv = to_i16_vec(y);
        for i in 0..res_shorts.len() {
            let xi = xv[i];
            let yi = yv[i];
            res_shorts[i] = if xi == i16::MIN || yi == i16::MIN { i16::MIN } else { xi.wrapping_add(yi) };
        }
    }
    res
}

fn add_mixed_bytes(x: &K, y: &K) -> K {
    let res = ktn(KG, x.n());
    let res_bytes = res.kG();
    let (xb, yb) = (x.kB(), y.kB());
    for i in 0..res_bytes.len() {
        res_bytes[i] = (xb[i].wrapping_add(yb[i])) as u8;
    }
    res
}

fn add_mixed_bools(x: &K, y: &K) -> K {
    let res = ktn(KB, x.n());
    let res_bools = res.kB();
    let (xb, yb) = (x.kB(), y.kB());
    for i in 0..res_bools.len() {
        res_bools[i] = if (xb[i] != 0) ^ (yb[i] != 0) { 1 } else { 0 };
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

        let (x_longs, y_longs, res_longs) = (x.kJ(), y.kJ(), res.kJ());

        const NJ: i64 = i64::MIN;
        for i in 0..res_longs.len() {
            let xi = x_longs[i];
            let yi = y_longs[i];
            res_longs[i] = if xi == NJ || yi == NJ { NJ } else { xi.wrapping_add(yi) };
        }

        res
    }

    // 2. Scaled float vector multiplication (KF / Type 9 * -KF)
    pub fn scale_floats(x: K, y: K) -> K {
        if x.t() != KF || y.t() != -KF { return krr("type"); }

        let factor = xf(y);
        let res = ktn(KF, x.n());
        let (x_floats, res_floats) = (x.kF(), res.kF());

        for i in 0..res_floats.len() {
            res_floats[i] = x_floats[i] * factor;
        }

        res
    }

    // 3. Filter int vector > y (KI / Type 6)
    pub fn filter_greater(x: K, y: K) -> K {
        if x.t() != KI || y.t() != -KI { return krr("type"); }

        let x_ints = x.kI();
        let limit = xi(y);

        let mut count = 0i64;
        for i in 0..x_ints.len() {
            if x_ints[i] > limit {
                count += 1;
            }
        }

        let res = ktn(KI, count);
        let res_ints = res.kI();
        let mut idx = 0usize;
        for i in 0..x_ints.len() {
            let val = x_ints[i];
            if val > limit {
                res_ints[idx] = val;
                idx += 1;
            }
        }

        res
    }

    // 4. Count character occurrences in string (KC / Type 10)
    pub fn count_char(s: K, c: K) -> K {
        if s.t() != KC || c.t() != -KC { return krr("type"); }

        let s_chars = s.kC();
        let char_to_find = xg(c);

        let mut count = 0i32;
        for i in 0..s_chars.len() {
            if s_chars[i] == char_to_find {
                count += 1;
            }
        }

        ki(count)
    }

    // 5. Retrieve a vector from a mixed list and sum it (Mixed / Type 0)
    pub fn sum_mixed(x: K, y: K) -> K {
        if x.t() != 0 || y.t() != -KI { return krr("type"); }

        let index = xi(y) as usize;
        if index >= x.n() as usize { return krr("index"); }

        let x_mixed = kK(x);
        let sub_vector = &x_mixed[index];

        if sub_vector.t() != KJ { return krr("type"); }

        let sub_slice = sub_vector.kJ();
        let mut total = 0i64;
        let sub_len = sub_slice.len();
        for i in 0..sub_len {
            total = total.wrapping_add(sub_slice[i]);
        }

        kj(total)
    }

    // 6. Vector dot product (KF * KF -> KF)
    pub fn dot_product(x: K, y: K) -> K {
        if x.t() != KF || y.t() != x.t() { return krr("type"); }
        if x.n() != y.n() { return krr("length"); }

        let (x_floats, y_floats) = (x.kF(), y.kF());

        let mut total = 0.0;
        for i in 0..x_floats.len() {
            total += x_floats[i] * y_floats[i];
        }

        kf(total)
    }

    // 7. Boolean mask filtering (KJ where KB -> KJ)
    pub fn filter_mask(x: K, mask: K) -> K {
        if x.t() != KJ || mask.t() != KB { return krr("type"); }
        if x.n() != mask.n() { return krr("length"); }

        let (x_longs, mask_bytes) = (x.kJ(), mask.kB());

        let mut count = 0i64;
        for i in 0..x_longs.len() {
            if mask_bytes[i] != 0 {
                count += 1;
            }
        }

        let res = ktn(KJ, count);
        let res_longs = res.kJ();
        let mut idx = 0usize;
        for i in 0..x_longs.len() {
            if mask_bytes[i] != 0 {
                res_longs[idx] = x_longs[i];
                idx += 1;
            }
        }

        res
    }

    // 8. Count occurrences of target symbol (KS where KS -> KI)
    pub fn count_symbol(x: K, target: K) -> K {
        if x.t() != KS || target.t() != -KS { return krr("type"); }

        let x_syms = x.kS();
        let target_sym = xs(target);

        let mut count = 0i32;
        for i in 0..x_syms.len() {
            if x_syms[i] == target_sym {
                count += 1;
            }
        }

        ki(count)
    }

    // 9. Add mixed numeric vectors with upcasting promotion
    pub fn add_mixed(x: K, y: K) -> K {
        let xtype = x.t();
        let ytype = y.t();
        let xlen = x.n();
        if xlen != y.n() { return krr("length"); }

        let common_type = match (xtype, ytype) {
            (1..=9, 1..=9) => {
                if xtype == 2 || xtype == 3 || ytype == 2 || ytype == 3 {
                    return krr("type");
                }
                std::cmp::max(xtype, ytype)
            }
            _ => return krr("type"),
        };

        match common_type {
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
        let size = if t == -6 {
            xi(n) as i64
        } else if t == -7 {
            xj(n)
        } else {
            return krr("type");
        };

        if size < 0 { return krr("length"); }

        let res = ktn(KJ, size);
        let res_longs = res.kJ();
        for i in 0..res_longs.len() {
            res_longs[i] = i as i64;
        }

        res
    }

    // 11. Package results in a mixed list
    pub fn package_results(x: K, y: K) -> K {
        let res = ktn(0, 2);
        let res_k = res.kK();
        res_k[0] = x.clone();
        res_k[1] = y.clone();
        res
    }

    // 12. Test early exit error leak prevention
    pub fn test_leak_krr(x: K) -> K {
        let size = xj(x);
        let _res = ktn(KJ, size);
        krr("test_krr_error")
    }

    // 13. Trigger standard rust panic to test FFI boundary wrapping
    pub fn trigger_panic(_x: K) -> K {
        panic!("intentional rust panic to test catch_unwind in k4rust");
    }

    // 14. Verify new API signatures and constructors compile
    pub fn test_new_apis(x: K) -> K {
        // GUID
        let guid = ku([0; 16]);
        let _guids = kU(&guid);
        let _guid_free = kU(&guid);

        // Shorthand list indexing
        let mixed_list = ktn(0, 2);
        let _first = xx(&mixed_list);
        let _second = xy(&mixed_list);

        // Temporal constructors
        let _date = kd(20000101);
        let _datetime = kz(1.5);
        let _time = kt(120000);
        let _timestamp = ktj(KP, 1234567890);

        // Symbol constructor & interning
        let symbol_name = ss("test_symbol");
        let _sym = ks(symbol_name);

        // Dict / Table constructors
        let keys = ktn(KS, 1);
        let vals = ktn(KI, 1);
        let dict = xD(keys, vals);
        let _tbl = xT(dict.clone());
        let _simple_tbl = ktd(dict);

        // Serialization
        let _serialized = b9(1, &x);
        let _deserialized = d9(&x);

        // Temporal helpers
        let _encoded = ymd(2020, 1, 1);
        let _decoded = dj(_encoded);

        // List Appenders
        let mut list_i = ktn(KI, 0);
        ja(&mut list_i, &123 as *const i32 as *mut _);
        let mut list_s = ktn(KS, 0);
        js(&mut list_s, symbol_name);
        let mut list_mixed = ktn(0, 0);
        jk(&mut list_mixed, ki(456));
        let mut list_i2 = ktn(KI, 0);
        jv(&mut list_i2, ktn(KI, 2));

        // Evaluation
        let _eval0 = k0(0, "1+1");
        let _eval1 = k1(0, "{(x)}", ki(5));
        let _eval2 = k2(0, "{x+y}", ki(1), ki(2));
        let _eval3 = k3(0, "{x+y+z}", ki(1), ki(2), ki(3));

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
        let (sock1, sock2) = UnixStream::pair().unwrap();
        sock1.set_nonblocking(true).unwrap();
        sock2.set_nonblocking(true).unwrap();
        let fd1 = sock1.as_raw_fd();
        let fd2 = sock2.as_raw_fd();

        println!("Rust: fd1={}, fd2={}", fd1, fd2);

        // Store them globally so they are not dropped
        *SOCKET_1.lock().unwrap() = Some(sock1);
        *SOCKET_2.lock().unwrap() = Some(sock2);

        // Reset the counters
        *CALLED_COUNT.lock().unwrap() = 0;
        LAST_READ_DATA.lock().unwrap().clear();

        // Register with sd1
        let res_sd = sd1(fd1, Some(socket_callback));
        println!("Rust: sd1 returned = {:?}", res_sd.as_raw());

        // Return (fd1; fd2)
        let res = ktn(KI, 2);
        let res_ints = res.kI();
        res_ints[0] = fd1;
        res_ints[1] = fd2;
        res
    }

    // FFI entry point: Test sd write
    pub fn test_sd_write(fd: K, data: K) -> K {
        let fd_val = xi(fd);
        if data.t() != KC { return krr("type"); }
        let bytes = kC(data);
        let written = unsafe { write(fd_val, bytes.as_ptr() as *const _, bytes.len()) };
        kb(if written > 0 { 1 } else { 0 })
    }

    // FFI entry point: Test sd status retrieval
    pub fn test_sd_status(_x: K) -> K {
        let count = *CALLED_COUNT.lock().unwrap();
        let data = LAST_READ_DATA.lock().unwrap().clone();
        
        let res = ktn(0, 2);
        let res_k = res.kK();
        res_k[0] = ki(count);
        
        // Convert Vec<u8> to a KC string object
        let str_len = data.len();
        let s_k = ktn(KC, str_len as i64);
        let s_bytes = s_k.kC();
        s_bytes[..str_len].copy_from_slice(&data);
        res_k[1] = s_k;
        res
    }

    // FFI entry point: Test sd0 and sd0x unregistration and close
    pub fn test_sd_close(fd: K, mode: K) -> K {
        let fd_val = xi(fd);
        let mode_val = xi(mode);
        if mode_val == 1 {
            sd0x(fd_val, 1);
            // Clear the global socket storage if closed by sd0x
            if fd_val == SOCKET_1.lock().unwrap().as_ref().map(|s| s.as_raw_fd()).unwrap_or(-1) {
                let mut s = SOCKET_1.lock().unwrap();
                let _ = s.take();
            } else if fd_val == SOCKET_2.lock().unwrap().as_ref().map(|s| s.as_raw_fd()).unwrap_or(-1) {
                let mut s = SOCKET_2.lock().unwrap();
                let _ = s.take();
            }
        } else {
            sd0(fd_val);
            unsafe { close(fd_val); }
            if fd_val == SOCKET_1.lock().unwrap().as_ref().map(|s| s.as_raw_fd()).unwrap_or(-1) {
                let mut s = SOCKET_1.lock().unwrap();
                let _ = s.take();
            } else if fd_val == SOCKET_2.lock().unwrap().as_ref().map(|s| s.as_raw_fd()).unwrap_or(-1) {
                let mut s = SOCKET_2.lock().unwrap();
                let _ = s.take();
            }
        }
        kb(1)
    }
}

