
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Context.h");

        type Context;


        include!("aeron-rust-wrapper/cxx_wrapper/Context.cpp");
        #[namespace = "aeron::context"]
        #[rust_name = "say_hello"]
        fn sayHello();
    }
}