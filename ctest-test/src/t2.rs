use std::ffi::{c_char, c_int};

pub type T2Foo = u32;
pub type T2Bar = u32;

pub type T2TypedefFoo = T2Foo;
pub type T2TypedefInt = c_int;

macro_rules! i {
    ($i:item) => {
        $i
    };
}

#[repr(C)]
#[derive(Debug)]
pub struct T2Baz {
    pub a: i64,
    pub b: u32,
}

#[repr(C)]
pub union T2Union {
    pub a: u32,
    pub b: i64,
}

pub const T2C: i32 = 5;

i! {
    pub const T2S: *const c_char = b"b\0".as_ptr().cast();
}

// FIXME(ctest): Cannot be uncommented until tests for functions are implemented in ctest-next.
// extern "C" {
//     pub fn T2a();
// }
