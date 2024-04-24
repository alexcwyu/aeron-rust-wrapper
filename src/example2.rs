use cxx::CxxString;
#[cxx::bridge(namespace = "example2")]
pub mod ffi {
    unsafe extern "C++" {

        #[namespace = "example"]
        type Object = crate::example::ffi::Object;

        include!("aeron-rust-wrapper/cxx_wrapper/example2.h");
        type Object2;

        fn createShared(name: &CxxString) -> SharedPtr<Object2>;
        fn createUnique(name: &CxxString) -> UniquePtr<Object2>;
        fn sayHi(self: &Object2);
        fn getObj1(self: &Object2) -> SharedPtr<Object>;
        fn getObj1Mut(self: Pin<&mut Object2>) -> SharedPtr<Object>;
        fn getObj2(self: &Object2) -> UniquePtr<Object>;
        fn getObj2Mut(self: Pin<&mut Object2>) -> UniquePtr<Object>;
    }
}