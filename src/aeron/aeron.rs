
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Aeron.h");

        type Aeron;

        #[rust_name = "is_closed"]
        fn isClosed(self: Pin<&mut Aeron>) ->bool;
        // fn connect(self: Pin<&mut Aeron>);


        include!("aeron-rust-wrapper/cxx_wrapper/Aeron.cpp");
        #[namespace = "aeron::aeron"]
        #[rust_name = "say_hello"]
        fn sayHello();
    }
}