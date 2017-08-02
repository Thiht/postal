extern crate libc;

use std::ffi;
use std::os::raw;

#[link(name = "postal")]
extern {
    pub fn libpostal_setup() -> bool;
    pub fn libpostal_setup_parser() -> bool;
    pub fn libpostal_parse_address(address: *const raw::c_char, options: AddressParserOptions) -> *const AddressParserResponse;
}

#[repr(C)]
#[derive(Debug)]
pub struct AddressParserOptions {
    language: *const raw::c_char,
    country: *const raw::c_char,
}

impl Default for AddressParserOptions {
    fn default() -> AddressParserOptions {
        AddressParserOptions {
            language: ffi::CString::new("").unwrap().as_ptr(),
            country: ffi::CString::new("").unwrap().as_ptr(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct AddressParserResponse {
    pub num_components: libc::size_t,
    pub components: *const *const raw::c_char,
    pub labels: *const *const raw::c_char,
}
