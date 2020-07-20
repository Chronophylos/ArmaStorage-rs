//! # Usage in SQF
//! Arma stringifies every argument except the function name when using
//! [`callExtension`][callExtension]. We use
//! this to our advantage so we do not have to destringify the data send to the
//! extension. Also this is done to be compatible with [`FileXT`][FileXT] so you
//! can use this extension as a drop-in replacement for [`FileXT`][FileXT]
//!
//! This changes the syntax a bit in usage:
//! ```sqf
//! "arma_storage" callExtension [data, [function, arg1, arg2, ...]];
//! ```
//!
//! ## Commands
//! ### Get Error Codes
//! ```sqf
//! "arma_storage" callExtension ["", ["errorCodes"]];
//! ```
//!
//! ### Open Storage
//!
//! Open a storage. If the storage is already open an error is returned.
//!
//! | | |
//! | --- | --- |
//! | **Syntax** | `"arma_storage" callExtension ["", ["open", storage]]` |
//! | **Parameters** | **storage**: *String* - storage name |
//! | **Return Value** | *nothing* |
//!
//! #### Example
//! ```sqf
//! "arma_storage" callExtension ["", ["open", "spam"]];
//! ```
//!
//! ### Close Storage
//!
//! Close a storage file. If the storage was not written before all changes are lost.
//! If the storage is already closed an error is returned.
//!
//! | | |
//! | --- | --- |
//! | **Syntax** | `"arma_storage" callExtension ["", ["close", storage]]` |
//! | **Parameters** | **storage**: *String* - storage name |
//! | **Return Value** | *nothing* |
//!
//! #### Example
//! ```sqf
//! "arma_storage" callExtension ["", ["close", "spam"]];
//! ```
//!
//! ### Read Storage
//!
//! Read a storage file. If the storage is already in memory all possible changes will get overridden.
//! If the storage is not open an error is returned.
//!
//! | | |
//! | --- | --- |
//! | **Syntax** | `"arma_storage" callExtension ["", ["read", storage]]` |
//! | **Parameters** | **storage**: *String* - storage name |
//! | **Return Value** | *nothing* |
//!
//! #### Example
//! ```sqf
//! "arma_storage" callExtension ["", ["read", "spam"]];
//! ```
//!
//! ### Write Storage
//!
//! Write a storage file. This overrides the file.
//! If the storage is not open an error is returned.
//!
//! | | |
//! | --- | --- |
//! | **Syntax** | `"arma_storage" callExtension ["", ["write", storage]]` |
//! | **Parameters** | **storage**: *String* - storage name |
//! | **Return Value** | *nothing* |
//!
//! #### Example
//! ```sqf
//! "arma_storage" callExtension ["", ["write", "spam"]];
//! ```
//!
//!
//! ## Error Codes
//!
//! With some errors the result contains more information about what went wrong.
//!
//! | Code | Meaning | Result Value |
//! | :--: | :------ | ------------ |
//! | 0 | no error | nothing |
//! | 1 | A parameter is not valid UTF-8 | nothing |
//! | 2 | The function passed is unknown | The function name |
//! | 10 | Missing a required argument | The name of the argument |
//! | 11 | Argument is empty| The name of the argument |
//! | 20 | A error in the storage occured | The exact error with cause |
//!
//! [FileXT]: https://github.com/Vindicta-Team/FileXT
/// [callExtension]: https://community.bistudio.com/wiki/callExtension
mod error;
mod extension;
mod memory;
mod storage;
mod value;

pub use error::ErrorCodes;
pub use storage::Storage;
pub use value::Value;

use log::{error, info};
use memory::write_str_to_ptr;
use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
    slice,
};

/// This function gets called when loading an extension
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn RVExtensionVersion(response_ptr: *mut c_char, response_size: c_int) {
    env_logger::init();

    let version = env!("CARGO_PKG_VERSION");

    info!("Loading Arma Storage {}", version);

    write_str_to_ptr(
        &format!("Arma Storage {}", version),
        response_ptr,
        response_size as usize,
    );
}

/// This function gets called when using the standard syntax of [`callExtension`][callExtension]
///
/// [callExtension]: https://community.bistudio.com/wiki/callExtension
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn RVExtension(
    response_ptr: *mut c_char,
    response_size: c_int,
    function_name_ptr: *const c_char,
) -> c_int {
    let function = match CStr::from_ptr(function_name_ptr).to_str() {
        Ok(d) => d,
        Err(err) => {
            error!(
                "Data argument (function) is not a valid UTF-8 string: {:?}",
                err
            );
            return ErrorCodes::InvalidUtf8.into();
        }
    };

    let (error_code, result) = extension::ext(function);

    write_str_to_ptr(&result.as_sqf(), response_ptr, response_size as usize);

    error_code as c_int
}

/// This function gets called when using the alternative syntax of [`callExtension`][callExtension]
///
/// [callExtension]: https://community.bistudio.com/wiki/callExtension
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn RVExtensionArgs(
    response_ptr: *mut c_char,
    response_size: c_int,
    function_name_ptr: *const c_char,
    argv: *const *const c_char,
    argc: c_int,
) -> c_int {
    let argc = argc as usize;
    let raw_args = slice::from_raw_parts(argv, argc);
    let args: Vec<&str> = match raw_args
        .iter()
        .map(|&c_str| CStr::from_ptr(c_str).to_str())
        .collect()
    {
        Ok(vec) => vec,
        Err(err) => {
            error!("Argument in argv is not a valid UTF-8 string: {:?}", err);
            return ErrorCodes::InvalidUtf8.into();
        }
    };

    let function = match CStr::from_ptr(function_name_ptr).to_str() {
        Ok(d) => d,
        Err(err) => {
            error!(
                "Data argument (function) is not a valid UTF-8 string: {:?}",
                err
            );
            return ErrorCodes::InvalidUtf8.into();
        }
    };

    let (error_code, result) = extension::ext_args(function, args);

    write_str_to_ptr(&result.as_sqf(), response_ptr, response_size as usize);

    error_code as c_int
}
