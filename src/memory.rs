use log::error;
use std::{cmp::min, ffi::CString, os::raw::c_char};

/// Copies an ASCII rust string into a memory buffer as a C string.
/// Performs necessary validation, including:
/// * Ensuring the string is ASCII
/// * Ensuring the string has no null bytes except at the end
/// * Making sure string length doesn't exceed the buffer.
/// # Returns
/// :Option with the number of ASCII characters written - *excludes the C null terminator*
// taken from https://github.com/Spoffy/Rust-Arma-Extension-Example/blob/5fc61340a1572ddecd9f8caf5458fd4faaf28e20/src/lib.rs#L88-L113
pub unsafe fn write_str_to_ptr(string: &str, ptr: *mut c_char, buf_size: usize) -> Option<usize> {
    // We shouldn't encode non-ascii string as C strings, things will get weird. Better to abort, I think.
    if !string.is_ascii() {
        error!("Trying to encode non-asci string");
        return None;
    };

    // This should never fail, honestly - we'd have to have manually added null bytes or something.
    let cstr = CString::new(string).ok()?;
    let cstr_bytes = cstr.as_bytes();

    // C Strings end in null bytes. We want to make sure we always write a valid string.
    // So we want to be able to always write a null byte at the end.
    let amount_to_copy = min(cstr_bytes.len(), buf_size - 1);

    // We provide a guarantee to our unsafe code, that we'll never pass anything too large.
    // In reality, I can't see this ever happening.
    if amount_to_copy > isize::MAX as usize {
        error!("String is too large");
        return None;
    }

    // We'll never copy the whole string here - it will always be missing the null byte.
    ptr.copy_from(cstr.as_ptr(), amount_to_copy);
    // this is equal to strncpy(ptr, cstr.as_ptr(), amount_to_copy);

    // Add our null byte at the end
    ptr.add(amount_to_copy).write(0x00);

    Some(amount_to_copy)
}
