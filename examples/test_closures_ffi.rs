
extern crate aeron_rust_wrapper;

use std::pin::{pin, Pin};
use aeron_rust_wrapper::closures_ffi;
use aeron_rust_wrapper::closures_ffi::Counter;


fn callback(x: i32){
    println!("calling in Rust, result: {}", x);
}

fn callback2(x: i32, counter: Pin<& mut Counter>){
    println!("calling in Rust, result: {}, counter: {:?}", x, counter);
}

fn main(){
    // let callback = |x| {
    //     println!("Result: {}", x);
    // };

    closures_ffi::ffi::simple_add_two_numbers1(1, 2, callback);

    let mut counter = Counter::default();
    let mut pinned_counter = pin!(counter);
    closures_ffi::ffi::better_add_two_numbers(1, 2, callback2, pinned_counter.as_mut());


    closures_ffi::ffi::better_add_two_numbers(1, 2, callback2, pinned_counter.as_mut());
    println!("counter: {:?}", pinned_counter.as_ref().get_ref());
}