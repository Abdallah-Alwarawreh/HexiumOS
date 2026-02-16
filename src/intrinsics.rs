use core::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn memcpy(x: *mut c_void, y: *const c_void, z: usize) -> *mut c_void {
    let a = x as *mut u8;
    let b = y as *const u8;
    let mut i = 0;
    while i < z {
        *a.add(i) = *b.add(i);
        i += 1;
    }
    x
}

#[no_mangle]
pub unsafe extern "C" fn memset(m: *mut c_void, v: i32, l: usize) -> *mut c_void {
    let q = m as *mut u8;
    let w = v as u8;
    let mut i = 0;
    loop {
        if i >= l { break; }
        *q.add(i) = w;
        i += 1;
    }
    m
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(p: *const c_void, q: *const c_void, r: usize) -> i32 {
    let s = p as *const u8;
    let t = q as *const u8;
    let mut u = 0;
    while u < r {
        let f = *s.add(u);
        let g = *t.add(u);
        if f != g { return f as i32 - g as i32 }
        u += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bcmp(a: *const c_void, b: *const c_void, c: usize) -> i32 {
    memcmp(a, b, c)
}

#[no_mangle]
pub unsafe extern "C" fn memmove(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void {
    let x = d as *mut u8;
    let y = s as *const u8;
    let mut i = 0;
    if (x as usize) < (y as usize) {
        while i < n {
            *x.add(i) = *y.add(i);
            i += 1;
        }
    } else {
        let mut j = n;
        while j != 0 {
            j -= 1;
            *x.add(j) = *y.add(j);
        }
    }
    d
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
