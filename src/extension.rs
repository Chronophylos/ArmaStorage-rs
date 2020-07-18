use log::info;
use std::{
    ffi::CString,
    os::raw::{c_char, c_int},
    ptr::copy,
};

pub fn get_version(output: *mut c_char, output_size: c_int) {
    info!("Loading Storage Extension");

    let mut version = format!("Arma Storage {}", env!("CARGO_PKG_VERSION"));

    // ensure the string is never too long
    version.truncate(output_size as usize);

    let c_string = CString::new(version).unwrap();

    unsafe {
        copy(c_string.as_ptr(), output, output_size as usize);
    }
}

pub fn exec_with_args(
    output: *mut c_char,
    output_size: c_int,
    function: *const c_char,
    argv: *const *const c_char,
    argc: c_int,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        let vec_buffer: Vec<u8> = [0x64u8; 256].to_vec();
        let str_buffer = CString::new(vec_buffer).unwrap();
        let ptr = str_buffer.clone().into_raw();

        get_version(ptr, str_buffer.as_bytes().len() as i32);

        let version = CString::new(format!("Arma Storage {}", env!("CARGO_PKG_VERSION"))).unwrap();
        let new_buffer = unsafe { CString::from_raw(ptr) };

        assert_eq!(version, new_buffer)
    }

    #[test]
    fn version_too_small_buffer() {
        let vec_buffer: Vec<u8> = [0x64u8; 8].to_vec();
        let str_buffer = CString::new(vec_buffer).unwrap();
        let ptr = str_buffer.clone().into_raw();

        get_version(ptr, str_buffer.as_bytes().len() as i32);

        let mut version = format!("Arma Storage {}", env!("CARGO_PKG_VERSION"));
        version.truncate(8);
        let c_version = CString::new(version).unwrap();
        let new_buffer = unsafe { CString::from_raw(ptr) };

        assert_eq!(c_version, new_buffer)
    }
}
