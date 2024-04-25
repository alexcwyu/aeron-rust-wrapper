


use std::os::raw::{c_int, c_void};


#[derive(Debug, Default, Clone, PartialEq)]
pub struct Counter {
    total: i32,
    calls: usize,
}

impl Counter {
    fn add_result(&mut self, result: i32) {
        self.total += result;
        self.calls += 1;
    }

}

//pub type AddCallback = unsafe extern "C" fn(c_int);
// type AddCallback2 = unsafe extern "C" fn(c_int);
#[cxx::bridge(namespace = "closures_ffi")]
pub mod ffi {

    extern "Rust" {
        type Counter;
        fn add_result(&mut self, result: i32);
    }
    unsafe extern "C++" {
        include!("aeron-rust-wrapper/cxx_wrapper/closures_ffi.h");
        type c_void;

        fn simple_add_two_numbers1(a : i32, b: i32, cb: fn(i32) -> ()) ;

        fn better_add_two_numbers(a : i32, b: i32, cd : fn(i32, Pin<&mut Counter>)->(), counter: Pin<&mut Counter>);

        // fn better_add_two_numbers2(a : i32, b: i32, cd : fn(i32, Pin<&mut Counter>)->(), counter: Pin<&mut Counter>);

        unsafe fn void_ptr(test: *mut c_void);
        unsafe fn void_ptr2(test: *mut c_void);
    }
}