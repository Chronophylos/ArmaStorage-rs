mod extension;
mod value;
pub use value::Value;

use std::os::raw::{c_char, c_int};

#[cfg(target_arch = "x86_64")]
#[no_mangle]
extern "C" fn RVExtensionVersion(output: *mut c_char, output_size: c_int) {
    extension::get_version(output, output_size);
}

#[cfg(target_arch = "x86")]
#[warn(non_snake_case)]
#[export_name = "_RVExtensionVersion@8"]
extern "C" fn RVExtensionVersion(output: *mut c_char, output_size: c_int) {
    extension::get_version(output, output_size);
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
extern "C" fn RVExtensionArgs(
    output: *mut c_char,
    output_size: c_int,
    function: *const c_char,
    argv: *const *const c_char,
    argc: c_int,
) {
    extension::exec_with_args(output, output_size, function, argv, argc)
}

#[cfg(target_arch = "x86")]
#[warn(non_snake_case)]
#[export_name = "_RVExtensionVersion@8"]
extern "C" fn RVExtensionArgs(
    output: *mut c_char,
    output_size: c_int,
    function: *const c_char,
    argv: *const *const c_char,
    argc: c_int,
) {
    extension::exec_with_args(output, output_size, function, argv, argc)
}
