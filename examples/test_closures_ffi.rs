extern crate aeron_rust_wrapper;

use std::pin::{pin, Pin};

use aeron_rust_wrapper::demo::closures_ffi;
use aeron_rust_wrapper::demo::closures_ffi::Counter;
use aeron_rust_wrapper::demo::closures_ffi::ffi::c_void;

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

fn test5(){
    println!("######## test5");
    let name = "test5".to_string();
    let mut counter = Counter::default();
    let mut closure = move|result: i32| {
        counter.add_result(result);
        println!("calling in Rust test5, {name}, result: {}, counter: {:?}", result, counter);
    };
    do_test5(&mut closure);
    do_test5(&mut closure);

}

fn do_test5<F>(on_result_calculated: F)where
    F: FnMut(i32), {
    {
        unsafe {
            let mut closure = on_result_calculated;

            let (mut state, mut callback) = ffi_helpers::split_closure(&mut closure);

            let callback2 : fn(*mut c_void, i32) ->() = std::mem::transmute(callback as *const ());
            let state2 = state as *mut _ as *mut c_void;

            closures_ffi::ffi::better_add_two_numbers3(
                1,
                2,
                callback2,
                state2,
            );

            closures_ffi::ffi::better_add_two_numbers3(
                1,
                2,
                callback2,
                state2,
            );
        }
    }
}

/////////////


pub trait ResultHandler {
    fn call(&mut self, input1: i32, input2: i32, result: i64);
    fn clone_box(&self) -> Box<dyn ResultHandler>;
}

// impl <T> FnMut<(i32, i32, i64)> for T where T: ResultHandler{
//     type Output = ();
//     fn call_mut(&mut self, input: (i32, i32, i64)) -> Self::Output {
//         self.call(input.0, input.1, input.2);
//     }
// }

impl<T> ResultHandler for T
    where
        T: FnMut(i32, i32, i64) + Clone + 'static,
{
    fn call(&mut self, input1: i32, input2: i32, result: i64) {
        self(input1, input2, result)
    }

    fn clone_box(&self) -> Box<dyn ResultHandler> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ResultHandler>{
    fn clone(&self) -> Box<dyn ResultHandler> {
        self.clone_box()
    }
}

fn default_result_handler(input1: i32, input2: i32, result: i64) {
    println!("default_result_handler: input1: {}, input2: {}, result3: {}", input1, input2, result);
}


#[derive(Clone, Debug)]
struct ResultCounter{
    name: String,
    input1: i32,
    input2: i32,
    result: i64,
    count: i64,
}

impl ResultCounter {
    pub fn new(name: &str) -> Self {
        ResultCounter {
            name: name.to_string(),
            input1: 0,
            input2: 0,
            result: 0,
            count: 0,
        }
    }

}

impl ResultHandler for ResultCounter {
    fn call(&mut self, input1: i32, input2: i32, result: i64) {
        self.input1 = input1;
        self.input2 = input2;
        self.result = result;
        self.count +=1;
        println!("ResultCounter: {:?}", self);
    }

    fn clone_box(&self) -> Box<dyn ResultHandler> {
        Box::new(self.clone())
    }
}



fn test6(){
    // println!("######## test6 part 1");
    //let mut counter  = ResultCounter::new("test6 object");
    // do_test6(&mut counter);
    // do_test6(&mut counter);

    println!("######## test6 part 2");
    let mut counter2 = ResultCounter::new("test6 closure");
    let mut closure = move|input1: i32, input2: i32, result: i64| {
        counter2.call(input1, input2, result);
        println!("calling from closure, counter: {:?}", counter2);
    };

    do_test6(&mut closure);
    do_test6(&mut closure);

    println!("######## test6 part 3");

    do_test6(&mut default_result_handler);
    do_test6(&mut default_result_handler);

}

fn do_test6<F>(result_handler : &mut F)where
    F: FnMut(i32, i32, i64) + Clone + 'static, {
    {
        unsafe {
            let mut closure = result_handler;

            let (mut state, mut callback) = ffi_helpers::split_closure(&mut closure);

            let callback2 : fn(*mut c_void, i32, i32, i64) ->() = std::mem::transmute(callback as *const ());
            let state2 = state as *mut _ as *mut c_void;

            closures_ffi::ffi::better_add_two_numbers4(
                1,
                2,
                callback2,
                state2,
            );

            closures_ffi::ffi::better_add_two_numbers4(
                1,
                2,
                callback2,
                state2,
            );
        }
    }
}


fn test7(){
    println!("######## test6 part 1");
    let mut counter  = ResultCounter::new("test6 object");
    do_test7(&mut counter);
    do_test7(&mut counter);

    println!("######## test6 part 2");
    let mut counter2 = ResultCounter::new("test6 closure");
    let mut closure = move|input1: i32, input2: i32, result: i64| {
        counter2.call(input1, input2, result);
        println!("calling from closure, counter: {:?}", counter2);
    };

    do_test7(&mut closure);
    do_test7(&mut closure);

    do_test7(&mut default_result_handler);
    do_test7(&mut default_result_handler);

}


fn do_test7(result_handler : &mut impl ResultHandler) {
    result_handler.call(1, 2, 3);
}

fn main(){
    // test1();
    // test2();
    // test3();
    // test4();
    // test5();
    //test6();
    test7();
}