use cxx::CxxString;

#[cxx::bridge(namespace = "example")]
pub mod ffi {
    unsafe extern "C++" {
        include!("aeron-rust-wrapper/cxx_wrapper/example.h");

        type Object;

        fn createShared(name: &CxxString) -> SharedPtr<Object>;
        fn createUnique(name: &CxxString) -> UniquePtr<Object>;
        fn sayHi(self: &Object);
    }
}