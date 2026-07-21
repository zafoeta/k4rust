#![allow(non_snake_case)]

pub mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(unsafe_op_in_unsafe_fn)]

    pub type S = *mut std::os::raw::c_char;
    pub type C = std::os::raw::c_char;
    pub type G = u8;
    pub type H = i16;
    pub type I = i32;
    pub type J = i64;
    pub type E = f32;
    pub type F = f64;
    pub type V = std::os::raw::c_void;
    pub type UJ = u64;

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct U {
        pub g: [G; 16],
    }

    #[repr(C)]
    pub struct k0 {
        pub m: std::os::raw::c_schar,
        pub a: std::os::raw::c_schar,
        pub t: std::os::raw::c_schar,
        pub u: C,
        pub r: I,
        pub union_data: k0_union,
    }

    #[repr(C)]
    pub union k0_union {
        pub g: G,
        pub h: H,
        pub i: I,
        pub j: J,
        pub e: E,
        pub f: F,
        pub s: S,
        pub k: *mut k0,
        pub list: k0_list,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct k0_list {
        pub n: J,
        pub G0: [G; 0],
    }

    pub type K = *mut k0;

    unsafe extern "C" {
        pub fn ku(arg1: U) -> K;
        pub fn knt(arg1: J, arg2: K) -> K;
        pub fn ktn(arg1: I, arg2: J) -> K;
        pub fn kpn(arg1: S, arg2: J) -> K;
        pub fn setm(arg1: I) -> I;
        pub fn ver() -> I;
        pub fn m9();
        pub fn gc(j: J) -> J;
        pub fn khpunc(arg1: S, arg2: I, arg3: S, arg4: I, arg5: I) -> I;
        pub fn khpun(arg1: S, arg2: I, arg3: S, arg4: I) -> I;
        pub fn khpu(arg1: S, arg2: I, arg3: S) -> I;
        pub fn khp(arg1: S, arg2: I) -> I;
        pub fn okx(arg1: K) -> I;
        pub fn ymd(arg1: I, arg2: I, arg3: I) -> I;
        pub fn dj(arg1: I) -> I;
        pub fn r0(arg1: K);
        pub fn sd0(arg1: I);
        pub fn sd0x(d: I, f: I);
        pub fn kclose(arg1: I);
        pub fn sn(arg1: S, arg2: I) -> S;
        pub fn ss(arg1: S) -> S;
        pub fn ee(arg1: K) -> K;
        pub fn ktj(arg1: I, arg2: J) -> K;
        pub fn ka(arg1: I) -> K;
        pub fn kb(arg1: I) -> K;
        pub fn kg(arg1: I) -> K;
        pub fn kh(arg1: I) -> K;
        pub fn ki(arg1: I) -> K;
        pub fn kj(arg1: J) -> K;
        pub fn ke(arg1: F) -> K;
        pub fn kf(arg1: F) -> K;
        pub fn kc(arg1: I) -> K;
        pub fn ks(arg1: S) -> K;
        pub fn kd(arg1: I) -> K;
        pub fn kz(arg1: F) -> K;
        pub fn kt(arg1: I) -> K;
        pub fn sd1(arg1: I, arg2: ::std::option::Option<unsafe extern "C" fn(arg1: I) -> K>) -> K;
        pub fn dl(f: *mut V, arg1: J) -> K;
        pub fn m4(arg1: I) -> K;
        pub fn knk(arg1: I, ...) -> K;
        pub fn kp(arg1: S) -> K;
        pub fn ja(arg1: *mut K, arg2: *mut V) -> K;
        pub fn js(arg1: *mut K, arg2: S) -> K;
        pub fn jk(arg1: *mut K, arg2: K) -> K;
        pub fn jv(k: *mut K, arg1: K) -> K;
        pub fn k(arg1: I, arg2: S, ...) -> K;
        pub fn xT(arg1: K) -> K;
        pub fn xD(arg1: K, arg2: K) -> K;
        pub fn ktd(arg1: K) -> K;
        pub fn r1(arg1: K) -> K;
        pub fn krr(arg1: S) -> K;
        pub fn orr(arg1: S) -> K;
        pub fn dot(arg1: K, arg2: K) -> K;
        pub fn b9(arg1: I, arg2: K) -> K;
        pub fn d9(arg1: K) -> K;
        pub fn sslInfo(x: K) -> K;
        pub fn vaknk(arg1: I, arg2: *mut std::os::raw::c_char) -> K;
        pub fn vak(arg1: I, arg2: S, arg3: *mut std::os::raw::c_char) -> K;
        pub fn vi(arg1: K, arg2: UJ) -> K;
        pub fn vk(arg1: K) -> K;
    }
}

// Safe memory management wrapper (matching original K)
// K is !Send and !Sync by default due to the inner *mut ffi::k0.
// This is intentional — KDB+ objects are not thread-safe and must not cross thread boundaries.
#[repr(transparent)]
pub struct K(*mut ffi::k0);

impl Drop for K {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                ffi::r0(self.0);
            }
        }
    }
}
impl Clone for K {
    fn clone(&self) -> Self {
        if !self.0.is_null() {
            unsafe {
                ffi::r1(self.0);
            }
        }
        K(self.0)
    }
}

impl K {
    #[doc(hidden)]
    pub unsafe fn __from_raw(ptr: *mut ffi::k0) -> Self {
        Self(ptr)
    }

    pub fn as_raw(&self) -> *mut ffi::k0 {
        self.0
    }

    pub fn duplicate(&self) -> K {
        if self.0.is_null() {
            return K::null();
        }
        let t = self.t();
        let n = self.n();
        if t < 0 {
            self.clone()
        } else if t == 0 {
            let res = ktn(0, n);
            let src_slice = self.kK();
            let dest_slice = res.kK();
            for i in 0..(n as usize) {
                dest_slice[i] = src_slice[i].clone();
            }
            res
        } else if t == 99 {
            let keys = self.xx().duplicate();
            let vals = self.xy().duplicate();
            xD(keys, vals)
        } else if t == 98 {
            let dict_ptr = unsafe { (*self.0).union_data.k };
            let dict = std::mem::ManuallyDrop::new(K(dict_ptr));
            let new_dict = dict.duplicate();
            xT(new_dict)
        } else {
            let res = ktn(t, n);
            unsafe {
                let elem_size = match t {
                    1 | 4 | 10 => 1,
                    2 => 16,
                    5 => 2,
                    6 | 13 | 14 | 17 | 18 | 19 => 4,
                    7 | 12 | 15 | 16 => 8,
                    8 => 4,
                    9 => 8,
                    // Type 11 (KS): symbol pointers are interned by q's `ss()`,
                    // so a shallow pointer copy is correct — the intern table owns the strings.
                    11 => std::mem::size_of::<*mut std::os::raw::c_char>(),
                    _ => 1,
                };
                let src_ptr = (*self.0).union_data.list.G0.as_ptr();
                let dest_ptr = (*res.0).union_data.list.G0.as_ptr() as *mut u8;
                std::ptr::copy_nonoverlapping(src_ptr, dest_ptr, (n as usize) * elem_size);
            }
            res
        }
    }

    pub fn make_mut(&mut self) {
        if self.r() > 0 {
            let new_k = self.duplicate();
            *self = new_k;
        }
    }

    pub fn null() -> Self {
        Self(std::ptr::null_mut())
    }

    pub fn into_raw(mut self) -> *mut ffi::k0 {
        let ptr = self.0;
        self.0 = std::ptr::null_mut();
        ptr
    }

    pub fn type_code(&self) -> i8 {
        if self.0.is_null() { 0 } else { unsafe { (*self.0).t } }
    }

    pub fn len(&self) -> usize {
        if self.0.is_null() { 0 } else { unsafe { (*self.0).union_data.list.n as usize } }
    }

    #[inline(always)]
    pub fn t(&self) -> i8 {
        if self.0.is_null() { 0 } else { unsafe { (*self.0).t } }
    }

    #[inline(always)]
    pub fn n(&self) -> i64 {
        if self.0.is_null() { 0 } else { unsafe { (*self.0).union_data.list.n } }
    }

    #[inline(always)]
    pub fn r(&self) -> i32 {
        if self.0.is_null() { 0 } else { unsafe { (*self.0).r } }
    }


    #[inline(always)]
    pub fn g(&self) -> u8 { if self.0.is_null() { 0 } else { unsafe { (*self.0).union_data.g } } }

    #[inline(always)]
    pub fn h(&self) -> i16 { if self.0.is_null() { 0 } else { unsafe { (*self.0).union_data.h } } }

    #[inline(always)]
    pub fn i(&self) -> i32 { if self.0.is_null() { 0 } else { unsafe { (*self.0).union_data.i } } }

    #[inline(always)]
    pub fn j(&self) -> i64 { if self.0.is_null() { 0 } else { unsafe { (*self.0).union_data.j } } }

    #[inline(always)]
    pub fn e(&self) -> f32 { if self.0.is_null() { 0.0 } else { unsafe { (*self.0).union_data.e } } }

    #[inline(always)]
    pub fn f(&self) -> f64 { if self.0.is_null() { 0.0 } else { unsafe { (*self.0).union_data.f } } }

    #[inline(always)]
    pub fn s(&self) -> *mut std::os::raw::c_char { if self.0.is_null() { std::ptr::null_mut() } else { unsafe { (*self.0).union_data.s } } }

    #[inline(always)]
    unsafe fn as_slice_mut_unchecked<T>(&self) -> &mut [T] {
        if self.0.is_null() {
            return &mut [];
        }
        unsafe {
            let ptr = (*self.0).union_data.list.G0.as_ptr() as *mut T;
            let len = (*self.0).union_data.list.n as usize;
            std::slice::from_raw_parts_mut(ptr, len)
        }
    }

    // NOTE: These slice accessors take &self but return &mut [T]. This is a deliberate
    // design trade-off mirroring the C API style (kJ(x)[i] = val). It violates Rust's
    // aliasing model but is safe in practice because KDB+ data access is single-threaded.
    // Callers must not hold two mutable slices to the same K simultaneously.
    #[inline(always)] pub fn kB(&self) -> &mut [i8] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn kG(&self) -> &mut [u8] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn kH(&self) -> &mut [i16] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn kI(&self) -> &mut [i32] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn kJ(&self) -> &mut [i64] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn kE(&self) -> &mut [f32] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn kF(&self) -> &mut [f64] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn kC(&self) -> &mut [u8] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn kS(&self) -> &mut [*mut std::os::raw::c_char] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn kK(&self) -> &mut [K] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn kU(&self) -> &mut [ffi::U] { unsafe { self.as_slice_mut_unchecked() } }
    #[inline(always)] pub fn xx(&self) -> &K { &self.kK()[0] }
    #[inline(always)] pub fn xy(&self) -> &K { &self.kK()[1] }

    /// Returns `true` if the inner pointer is null.
    pub fn is_null(&self) -> bool { self.0.is_null() }

    /// Returns `true` if this K represents a KDB+ error (type -128).
    pub fn is_error(&self) -> bool { self.t() == -128 }
}

impl std::fmt::Debug for K {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_null() {
            return write!(f, "K(null)");
        }
        let t = self.t();
        match t {
            -128 => {
                let s = self.s();
                let msg = if s.is_null() { "<null>" } else { unsafe { std::ffi::CStr::from_ptr(s).to_str().unwrap_or("<invalid>") } };
                write!(f, "K(error: \"{}\")", msg)
            }
            t if t < 0 => {
                match t {
                    -1  => write!(f, "K(bool: {})", self.g() != 0),
                    -4  => write!(f, "K(byte: {})", self.g()),
                    -5  => write!(f, "K(short: {})", self.h()),
                    -6  => write!(f, "K(int: {})", self.i()),
                    -7  => write!(f, "K(long: {})", self.j()),
                    -8  => write!(f, "K(real: {})", self.e()),
                    -9  => write!(f, "K(float: {})", self.f()),
                    -10 => write!(f, "K(char: {:?})", self.g() as char),
                    -11 => {
                        let s = self.s();
                        let sym = if s.is_null() { "<null>" } else { unsafe { std::ffi::CStr::from_ptr(s).to_str().unwrap_or("<invalid>") } };
                        write!(f, "K(symbol: `{})", sym)
                    }
                    _ => write!(f, "K(atom type={})", t),
                }
            }
            0 => write!(f, "K(mixed list len={})", self.n()),
            98 => write!(f, "K(table)"),
            99 => write!(f, "K(dict)"),
            _ => write!(f, "K(vector type={} len={})", t, self.n()),
        }
    }
}



// KDB+ Type Codes
pub const KB: i8 = 1;  // Boolean
pub const UU: i8 = 2;  // Guid
pub const KG: i8 = 4;  // Byte
pub const KH: i8 = 5;  // Short
pub const KI: i8 = 6;  // Int
pub const KJ: i8 = 7;  // Long
pub const KE: i8 = 8;  // Real
pub const KF: i8 = 9;  // Float
pub const KC: i8 = 10; // Char
pub const KS: i8 = 11; // Symbol
pub const KP: i8 = 12; // Timestamp
pub const KM: i8 = 13; // Month
pub const KD: i8 = 14; // Date
pub const KZ: i8 = 15; // DateTime
pub const KN: i8 = 16; // Timespan
pub const KU: i8 = 17; // Minute
pub const KV: i8 = 18; // Second
pub const KT: i8 = 19; // Time
pub const XT: i8 = 98; // Table
pub const XD: i8 = 99; // Dictionary

// KDB+ Nulls
pub const NH: i16 = i16::MIN;
pub const NI: i32 = i32::MIN;
pub const NJ: i64 = i64::MIN;
pub const NE: f32 = f32::NAN;
pub const NF: f64 = f64::NAN;
pub const NC: u8 = b' ';

// KDB+ Infinities
pub const WH: i16 = i16::MAX;
pub const WI: i32 = i32::MAX;
pub const WJ: i64 = i64::MAX;
pub const WE: f32 = f32::INFINITY;
pub const WF: f64 = f64::INFINITY;

pub fn kb(val: i32) -> K { unsafe { K(ffi::kb(val)) } }
pub fn kg(val: i32) -> K { unsafe { K(ffi::kg(val)) } }
pub fn kh(val: i32) -> K { unsafe { K(ffi::kh(val)) } }
pub fn ki(val: i32) -> K { unsafe { K(ffi::ki(val)) } }
pub fn kj(val: i64) -> K { unsafe { K(ffi::kj(val)) } }
pub fn ke(val: f64) -> K { unsafe { K(ffi::ke(val)) } }
pub fn kf(val: f64) -> K { unsafe { K(ffi::kf(val)) } }

pub fn kp(s: &str) -> K {
    unsafe { K(ffi::kpn(s.as_ptr() as *mut _, s.len() as i64)) }
}

pub fn ku(val: [u8; 16]) -> K { unsafe { K(ffi::ku(ffi::U { g: val })) } }
pub fn ks(s: ffi::S) -> K { unsafe { K(ffi::ks(s)) } }
pub fn kd(x: i32) -> K { unsafe { K(ffi::kd(x)) } }
pub fn kz(x: f64) -> K { unsafe { K(ffi::kz(x)) } }
pub fn kt(x: i32) -> K { unsafe { K(ffi::kt(x)) } }
pub fn ktj(t: i8, x: i64) -> K { unsafe { K(ffi::ktj(t as i32, x)) } }

pub fn sn(s: &str) -> *mut ::std::os::raw::c_char {
    unsafe { ffi::sn(s.as_ptr() as *mut _, s.len() as i32) }
}

pub fn ss(s: &str) -> ffi::S {
    let c_str = std::ffi::CString::new(s).unwrap();
    unsafe { ffi::ss(c_str.as_ptr() as *mut _) }
}

pub fn ns() -> *mut ::std::os::raw::c_char {
    sn("")
}

pub fn xD(x: K, y: K) -> K { unsafe { K(ffi::xD(x.into_raw(), y.into_raw())) } }
pub fn xT(x: K) -> K { unsafe { K(ffi::xT(x.into_raw())) } }
pub fn ktd(x: K) -> K { unsafe { K(ffi::ktd(x.into_raw())) } }

static ERROR_CACHE: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, std::ffi::CString>>> = std::sync::OnceLock::new();

pub fn krr(err: &str) -> K {
    let cache = ERROR_CACHE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let mut map = cache.lock().unwrap();
    let c_str = map.entry(err.to_string()).or_insert_with(|| std::ffi::CString::new(err).unwrap());
    K(unsafe { ffi::krr(c_str.as_ptr() as *mut _) })
}

pub fn ktn(t: i8, n: i64) -> K {
    let ptr = unsafe { ffi::ktn(t as i32, n) };
    if ptr.is_null() {
        krr("wsfull")
    } else {
        K(ptr)
    }
}

pub fn ja(x: &mut K, val: *mut std::os::raw::c_void) {
    unsafe { ffi::ja(&mut x.0, val); }
}
pub fn js(x: &mut K, s: ffi::S) {
    unsafe { ffi::js(&mut x.0, s); }
}
pub fn jk(x: &mut K, y: K) {
    unsafe { ffi::jk(&mut x.0, y.into_raw()); }
}
pub fn jv(x: &mut K, y: K) {
    unsafe { ffi::jv(&mut x.0, y.into_raw()); }
}

#[inline(always)] pub fn kB(x: &K) -> &mut [i8] { x.kB() }
#[inline(always)] pub fn kG(x: &K) -> &mut [u8] { x.kG() }
#[inline(always)] pub fn kH(x: &K) -> &mut [i16] { x.kH() }
#[inline(always)] pub fn kI(x: &K) -> &mut [i32] { x.kI() }
#[inline(always)] pub fn kJ(x: &K) -> &mut [i64] { x.kJ() }
#[inline(always)] pub fn kE(x: &K) -> &mut [f32] { x.kE() }
#[inline(always)] pub fn kF(x: &K) -> &mut [f64] { x.kF() }
#[inline(always)] pub fn kC(x: &K) -> &mut [u8] { x.kC() }
#[inline(always)] pub fn kS(x: &K) -> &mut [*mut std::os::raw::c_char] { x.kS() }
#[inline(always)] pub fn kK(x: &K) -> &mut [K] { x.kK() }
#[inline(always)] pub fn kU(x: &K) -> &mut [ffi::U] { x.kU() }
#[inline(always)] pub fn xx(x: &K) -> &K { x.xx() }
#[inline(always)] pub fn xy(x: &K) -> &K { x.xy() }

#[inline(always)] pub fn xg(x: &K) -> u8 { x.g() }
#[inline(always)] pub fn xh(x: &K) -> i16 { x.h() }
#[inline(always)] pub fn xi(x: &K) -> i32 { x.i() }
#[inline(always)] pub fn xj(x: &K) -> i64 { x.j() }
#[inline(always)] pub fn xe(x: &K) -> f32 { x.e() }
#[inline(always)] pub fn xf(x: &K) -> f64 { x.f() }
#[inline(always)] pub fn xs(x: &K) -> *mut std::os::raw::c_char { x.s() }

pub fn b9(mode: i32, x: &K) -> K { unsafe { K(ffi::b9(mode, x.0)) } }
pub fn d9(x: &K) -> K { unsafe { K(ffi::d9(x.0)) } }
#[inline(always)] pub fn r1(x: &K) -> K { x.clone() }

pub fn ymd(y: i32, m: i32, d: i32) -> i32 { unsafe { ffi::ymd(y, m, d) } }
pub fn dj(d: i32) -> i32 { unsafe { ffi::dj(d) } }

pub fn k0(h: i32, q: &str) -> K {
    let c_str = std::ffi::CString::new(q).unwrap();
    unsafe { K(ffi::k(h, c_str.as_ptr() as *mut _, std::ptr::null_mut::<ffi::k0>())) }
}

pub fn k1(h: i32, q: &str, x: K) -> K {
    let c_str = std::ffi::CString::new(q).unwrap();
    unsafe { K(ffi::k(h, c_str.as_ptr() as *mut _, x.into_raw(), std::ptr::null_mut::<ffi::k0>())) }
}

pub fn k2(h: i32, q: &str, x: K, y: K) -> K {
    let c_str = std::ffi::CString::new(q).unwrap();
    unsafe { K(ffi::k(h, c_str.as_ptr() as *mut _, x.into_raw(), y.into_raw(), std::ptr::null_mut::<ffi::k0>())) }
}

pub fn k3(h: i32, q: &str, x: K, y: K, z: K) -> K {
    let c_str = std::ffi::CString::new(q).unwrap();
    unsafe { K(ffi::k(h, c_str.as_ptr() as *mut _, x.into_raw(), y.into_raw(), z.into_raw(), std::ptr::null_mut::<ffi::k0>())) }
}

pub fn khpunc(hostname: &str, port: i32, username_password: &str, timeout: i32, capability: i32) -> i32 {
    let hostname_c = std::ffi::CString::new(hostname).unwrap();
    let creds_c = std::ffi::CString::new(username_password).unwrap();
    unsafe { ffi::khpunc(hostname_c.as_ptr() as *mut _, port, creds_c.as_ptr() as *mut _, timeout, capability) }
}

pub fn khpun(hostname: &str, port: i32, username_password: &str, timeout: i32) -> i32 {
    let hostname_c = std::ffi::CString::new(hostname).unwrap();
    let creds_c = std::ffi::CString::new(username_password).unwrap();
    unsafe { ffi::khpun(hostname_c.as_ptr() as *mut _, port, creds_c.as_ptr() as *mut _, timeout) }
}

pub fn khpu(hostname: &str, port: i32, username_password: &str) -> i32 {
    let hostname_c = std::ffi::CString::new(hostname).unwrap();
    let creds_c = std::ffi::CString::new(username_password).unwrap();
    unsafe { ffi::khpu(hostname_c.as_ptr() as *mut _, port, creds_c.as_ptr() as *mut _) }
}

pub fn khp(hostname: &str, port: i32) -> i32 {
    let hostname_c = std::ffi::CString::new(hostname).unwrap();
    unsafe { ffi::khp(hostname_c.as_ptr() as *mut _, port) }
}

pub fn okx(x: &K) -> i32 {
    unsafe { ffi::okx(x.0) }
}

pub fn kclose(socket: i32) {
    unsafe { ffi::kclose(socket); }
}

pub fn sd1(d: i32, f: Option<unsafe extern "C" fn(i32) -> *mut ffi::k0>) -> K {
    unsafe { K(ffi::sd1(d, f)) }
}

pub fn sd0(d: i32) {
    unsafe { ffi::sd0(d); }
}

pub fn sd0x(d: i32, f: i32) {
    unsafe { ffi::sd0x(d, f); }
}

/// A safe, RAII-based wrapper for KDB+ IPC client connections.
/// Automatically closes the socket descriptor when dropped.
pub struct IpcClient {
    handle: i32,
}

impl IpcClient {
    /// Returns the underlying KDB+ connection handle.
    pub fn handle(&self) -> i32 { self.handle }

    /// Connects to a remote KDB+ process.
    pub fn connect(hostname: &str, port: i32) -> Result<Self, String> {
        let h = khp(hostname, port);
        if h <= 0 {
            Err(format!("Connection failed, code: {}", h))
        } else {
            Ok(Self { handle: h })
        }
    }

    /// Connects to a remote KDB+ process with a username and password.
    pub fn connect_with_creds(hostname: &str, port: i32, username_password: &str) -> Result<Self, String> {
        let h = khpu(hostname, port, username_password);
        if h <= 0 {
            Err(format!("Connection failed with credentials, code: {}", h))
        } else {
            Ok(Self { handle: h })
        }
    }

    /// Connects to a remote KDB+ process with credentials and a connection timeout (in milliseconds).
    pub fn connect_with_timeout(hostname: &str, port: i32, username_password: &str, timeout_ms: i32) -> Result<Self, String> {
        let h = khpun(hostname, port, username_password, timeout_ms);
        if h <= 0 {
            Err(format!("Connection failed with timeout, code: {}", h))
        } else {
            Ok(Self { handle: h })
        }
    }

    /// Connects to a remote KDB+ process with credentials, timeout, and a capability flag (e.g. TLS).
    pub fn connect_with_capability(hostname: &str, port: i32, username_password: &str, timeout_ms: i32, capability: i32) -> Result<Self, String> {
        let h = khpunc(hostname, port, username_password, timeout_ms, capability);
        if h <= 0 {
            Err(format!("Connection failed with capability settings, code: {}", h))
        } else {
            Ok(Self { handle: h })
        }
    }

    /// Evaluates a query on the remote KDB+ process with 0 arguments.
    pub fn k0(&self, query: &str) -> K {
        k0(self.handle, query)
    }

    /// Evaluates a query on the remote KDB+ process with 1 argument.
    pub fn k1(&self, query: &str, x: K) -> K {
        k1(self.handle, query, x)
    }

    /// Evaluates a query on the remote KDB+ process with 2 arguments.
    pub fn k2(&self, query: &str, x: K, y: K) -> K {
        k2(self.handle, query, x, y)
    }

    /// Evaluates a query on the remote KDB+ process with 3 arguments.
    pub fn k3(&self, query: &str, x: K, y: K, z: K) -> K {
        k3(self.handle, query, x, y, z)
    }

    /// Evaluates a query with a dynamic number of arguments by packaging them
    /// into a mixed list under the hood and applying them via the KDB+ dot (`.`) operator.
    pub fn query(&self, query: &str, args: Vec<K>) -> K {
        // Fast-path: use direct k0-k3 calls for small arities to avoid
        // the overhead of mixed-list packaging and the dot operator.
        match args.len() {
            0 => return k0(self.handle, query),
            1 => {
                let mut it = args.into_iter();
                return k1(self.handle, query, it.next().unwrap());
            }
            2 => {
                let mut it = args.into_iter();
                let a = it.next().unwrap();
                let b = it.next().unwrap();
                return k2(self.handle, query, a, b);
            }
            3 => {
                let mut it = args.into_iter();
                let a = it.next().unwrap();
                let b = it.next().unwrap();
                let c = it.next().unwrap();
                return k3(self.handle, query, a, b, c);
            }
            _ => {}
        }

        let n = args.len();
        let al = ktn(0, n as i64);
        let s = al.kK();
        for (i, arg) in args.into_iter().enumerate() {
            s[i] = arg;
        }
        
        let qk = kp(query);
        k2(self.handle, "{(value x) . y}", qk, al)
    }

    /// Verifies the handshake response.
    pub fn okx(&self, x: &K) -> bool {
        okx(x) != 0
    }
}

impl Drop for IpcClient {
    fn drop(&mut self) {
        if self.handle > 0 {
            kclose(self.handle);
        }
    }
}



// Macro for defining kdb+ FFI functions safely and ergonomically inside a single block.
#[macro_export]
macro_rules! k4rust_api {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis fn $name:ident ( $($arg:ident : K),* $(,)? ) -> K $body:block
        )*
    ) => {
        $(
            $(#[$meta])*
            #[unsafe(no_mangle)]
            $vis unsafe extern "C" fn $name( $($arg : *mut $crate::ffi::k0),* ) -> *mut $crate::ffi::k0 {
                $(
                    let $arg = std::mem::ManuallyDrop::new(unsafe { $crate::K::__from_raw($arg) });
                    let $arg: &$crate::K = &*$arg;
                )*
                let catch_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                    let res: $crate::K = (move || $body)();
                    res
                }));
                match catch_res {
                    Ok(res) => res.into_raw(),
                    Err(_) => {
                        $crate::ffi::krr(b"panic\0".as_ptr() as *mut std::os::raw::c_char)
                    }
                }
            }
        )*
    };
}



