use std::ops::Deref;
use std::ptr;

use cxx::let_cxx_string;

use aeron_rust_wrapper::demo::example;
use aeron_rust_wrapper::demo::example2;

fn test1(){

    println!("########### test1");
    let_cxx_string!(name1 = "Tom (shared)");
    let_cxx_string!(name2 = "Mary (shared)");
    let mut ptr1 = example::ffi::createShared(&name1);
    let mut ptr2 = example2::ffi::createShared(&name2);

    {
        // Create a second shared_ptr holding shared ownership of the same
        // object. There is still only one Object but two SharedPtr<Object>.
        // Both pointers point to the same object on the heap.
        let ptr1_2 = ptr1.clone();
        assert!(ptr::eq(ptr1.deref(), ptr1_2.deref()));
        ptr1_2.sayHi();
        // ptr1_2 goes out of scope, but Object is not destroyed yet.
    }

    println!("##### 1");
    {
        let ptr2_2 = ptr2.clone();
        assert!(ptr::eq(ptr2.deref(), ptr2_2.deref()));
        ptr2_2.sayHi();
        // ptr2_2 goes out of scope, but Object2 is not destroyed yet.
    }

    println!("##### 2");
    ptr1.sayHi();
    ptr2.sayHi();

    println!("##### 3");
    ptr2.as_ref().unwrap().getObj1().sayHi();
    // ptr2.as_ref().unwrap().getObj1Mut().sayHi();
    ptr2.as_ref().unwrap().getObj2().sayHi();
    // ptr2.as_ref().unwrap().getObj2Mut().sayHi();
    // ptr1 goes out of scope and Object is destroyed.
    // ptr2 goes out of scope and Object2 is destroyed.
}

fn test2() {
    println!("########### test2");
    let_cxx_string!(name1 = "Peter (unique)");
    let_cxx_string!(name2 = "Jane (unique)");
    let mut ptr1 = example::ffi::createUnique(&name1);
    let mut ptr2 = example2::ffi::createUnique(&name2);


    ptr1.sayHi();
    ptr2.sayHi();
    println!("##### 2 ");
    ptr2.as_ref().unwrap().getObj1().sayHi();
    ptr2.as_mut().unwrap().getObj1Mut().sayHi();
    println!("##### 3");
    ptr2.as_ref().unwrap().getObj2().sayHi();
    ptr2.as_mut().unwrap().getObj2Mut().sayHi();

    // ptr1 goes out of scope and Object is destroyed.
    // ptr1 goes out of scope and Object is destroyed.
    // ptr2 goes out of scope and Object2 is destroyed.
}

fn main() {
    test1();
    test2();
}