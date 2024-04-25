#![no_main]

use anyhow::Result;
use cxx::ExternType;

pub trait MyData {
    fn traitfn(&self);
}

unsafe impl ExternType for Box<dyn MyData> {
    type Id = cxx::type_id!("traits_demo::BoxDynMyData");
    type Kind = cxx::kind::Trivial;
}

#[repr(transparent)]
pub struct PtrBoxDynMyData(*mut Box<dyn MyData>);
unsafe impl ExternType for PtrBoxDynMyData {
    type Id = cxx::type_id!("traits_demo::PtrBoxDynMyData");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "traits_demo")]
pub mod ffi {
    unsafe extern "C++" {
        include!("aeron-rust-wrapper/cxx_demo_include/mydata.h");

        fn do_work() -> ();
        type BoxDynMyData = Box<dyn crate::demo::mydata::MyData>;
        type PtrBoxDynMyData = crate::demo::mydata::PtrBoxDynMyData;
    }

    extern "Rust" {
        fn dyn_mydata_traitfn(mydata: &BoxDynMyData);
        unsafe fn dyn_mydata_drop_in_place(ptr: PtrBoxDynMyData);

        fn read_data() -> Result<BoxDynMyData>;
    }
}

fn dyn_mydata_traitfn(mydata: &Box<dyn MyData>) {
    (**mydata).traitfn();
}

unsafe fn dyn_mydata_drop_in_place(ptr: PtrBoxDynMyData) {
    std::ptr::drop_in_place(ptr.0);
}

fn read_data() -> Result<Box<dyn MyData>> {
    struct Implementation(usize);
    impl MyData for Implementation {
        fn traitfn(&self) {
            println!("it worked! {}", self.0);
        }
    }
    Ok(Box::new(Implementation(9)))
}