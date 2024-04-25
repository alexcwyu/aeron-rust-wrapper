
extern crate aeron_rust_wrapper;

use std::pin::{pin, Pin};
use aeron_rust_wrapper::closures_ffi;
use aeron_rust_wrapper::closures_ffi::Counter;
use aeron_rust_wrapper::closures_ffi::ffi::c_void;


fn callback(x: i32){
    println!("calling in Rust, result: {}", x);
}

fn callback2(x: i32, mut counter: Pin<& mut Counter>){
    counter.as_mut().get_mut().add_result(x);
    println!("calling in Rust, result: {}, counter: {:?}", x, &counter.as_ref().get_ref());
}

fn callback3(x: i32, user_data: *mut c_void){
    unsafe {
        let mut counter = &mut *(user_data as *mut Counter);
        counter.add_result(x);
        println!("calling in Rust void*, result: {}, counter: {:?}", x, counter);
    }
}


////////
//https://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/
pub type AddCallback4 = fn(i32, *mut c_void);


fn trampoline<F>(result: i32, user_data: *mut c_void)
    where
        F: FnMut(i32),
{
    unsafe {
        let user_data = &mut *(user_data as *mut F);
        user_data(result);
    }
}

pub fn get_trampoline<F>(_closure: &F) -> AddCallback4
    where
        F: FnMut(i32),
{
    trampoline::<F>
}

/////////////////////////

fn test1(){
    println!("######## test1");
    closures_ffi::ffi::simple_add_two_numbers1(1, 2, callback);
}

fn test2(){
    println!("######## test2");
    let mut counter = Counter::default();
    let mut pinned_counter = pin!(counter);
    closures_ffi::ffi::better_add_two_numbers(1, 2, callback2, pinned_counter.as_mut());
    closures_ffi::ffi::better_add_two_numbers(1, 2, callback2, pinned_counter.as_mut());
    println!("counter: {:?}", pinned_counter.as_ref().get_ref());
}

fn test3(){
    println!("######## test3");
    let mut counter = Counter::default();

    unsafe{
        let ptr = &mut counter as *mut Counter as *mut c_void;

        closures_ffi::ffi::better_add_two_numbers2(1, 2, callback3, ptr);

        closures_ffi::ffi::better_add_two_numbers2(1, 2, callback3, ptr);

    }
}

fn test4(){
    println!("######## test4");
    let name = "test4".to_string();
    let mut counter = Counter::default();
    let mut closure = move|result: i32| {
        counter.add_result(result);
        println!("calling in Rust test4, {name}, result: {}, counter: {:?}", result, counter);
    };
    do_test4(&mut closure);
    do_test4(&mut closure);


    println!("######## test4 part 2");
    let name = "test4 again".to_string();
    let mut counter2 = Counter::default();
    let mut closure2 = move|result: i32| {
        counter2.add_result(result);
        println!("calling in Rust test4 again, {name}, result: {}, counter: {:?}", result, counter2);
    };
    do_test4(&mut closure2);
    do_test4(&mut closure2);

}
fn do_test4<F>(on_result_calculated: F)where
    F: FnMut(i32), {
    {
        unsafe {
            let mut closure = on_result_calculated;
            let trampoline = get_trampoline(&closure);

            closures_ffi::ffi::better_add_two_numbers2(
                1,
                2,
                trampoline,
                &mut closure as *mut _ as *mut c_void,
            );

            closures_ffi::ffi::better_add_two_numbers2(
                1,
                2,
                trampoline,
                &mut closure as *mut _ as *mut c_void,
            );
        }
    }
}


fn main(){

    test1();
    test2();
    test3();
    test4();
}