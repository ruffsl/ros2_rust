#[allow(dead_code)]

mod rcl_bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(clippy::all)]
    #![allow(improper_ctypes)]

    include!(concat!(env!("OUT_DIR"), "/rcl_bindings.rs"));
}

pub(crate) use self::rcl_bindings::*;