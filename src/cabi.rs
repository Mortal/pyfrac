use std::slice;
use std::os::raw::c_char;
use std::ffi::CString;
use num_bigint::BigUint;
use bridge::*;
use err::Result;
use pyfrac::repeated;

#[no_mangle]
pub unsafe extern "C" fn pyfrac_init() {
    set_panic_hook();
}

#[no_mangle]
pub unsafe extern "C" fn pyfrac_free(buf: *mut c_char) {
    CString::from_raw(buf);
}

export!(pyfrac_repeated(num: *const u8, num_len: usize, den: *const u8, den_len: usize, base: usize, min_exp: usize) -> Result<*mut c_char> {
    let num = BigUint::from_bytes_be(slice::from_raw_parts(num as *const u8, num_len));
    let den = BigUint::from_bytes_be(slice::from_raw_parts(den as *const u8, den_len));
    let r = repeated(num, den, base, min_exp);
    let r_str = format!("{}", r);
    let r_cstring = CString::new(r_str.into_bytes()).unwrap();
    Ok(r_cstring.into_raw())
});
