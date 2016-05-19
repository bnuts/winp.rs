extern crate libc;

use libc::{c_int, c_schar, c_ulong, size_t, wchar_t};

#[repr(C)]
pub struct winp_t {
    pub output: *const c_schar,
    pub output_len: size_t,
    pub error: *const c_schar,
    pub error_len: size_t,
    pub return_code: c_ulong,
}

impl winp_t {
    pub fn new() -> Self {
        use std::ptr;

        winp_t {
            output: ptr::null(),
            output_len: 0,
            error: ptr::null(),
            error_len: 0,
            return_code: 0,
        }
    }
}

extern "system" {
    pub fn winp_run_w(
        winp: *mut winp_t,
        command: *const wchar_t,
        input: *const c_schar,
        input_len: size_t)
    -> c_int;

    pub fn winp_free(winp: *mut winp_t);
}
