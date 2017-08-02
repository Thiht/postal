mod ffi;

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::slice::from_raw_parts;
use std::sync::{Once, ONCE_INIT};

static C_LIB_INITIALIZED: Once = ONCE_INIT;

pub struct Postal {
    marker: PhantomData<()>,
}

impl Postal {
    // TODO: return an Option
    pub fn new() -> Self {
        C_LIB_INITIALIZED.call_once(|| {
            unsafe {
                ffi::libpostal_setup() && ffi::libpostal_setup_parser();
            }
        });
        Postal {
            marker: PhantomData,
        }
    }

    pub fn parse_address(&self, address: &str) -> Option<HashMap<&str, &str>> {
        let mut ret = HashMap::new();
        unsafe {
            // TODO: error handling
            let parsed_address = ffi::libpostal_parse_address(CString::new(address).unwrap().as_ptr(), Default::default());

            if parsed_address.is_null() {
                return None
            }

            let parsed_address = &*parsed_address;
            let num_components = parsed_address.num_components as usize;
            if num_components == 0 {
                return Some(ret)
            }

            let labels = from_raw_parts(parsed_address.labels, num_components);
            let components = from_raw_parts(parsed_address.components, num_components);
            for i in 0..num_components {
                // TODO: error handling
                let label = CStr::from_ptr(labels[i]).to_str().unwrap();
                let component = CStr::from_ptr(components[i]).to_str().unwrap();
                ret.insert(label, component);
            }
        }
        return Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let postal = Postal::new();
        let parsed_address = postal.parse_address("Rennes").expect("parse_address returned None");
        assert!(parsed_address.contains_key("city"));
        assert_eq!(parsed_address.get("city").unwrap(), "rennes");
    }
}
