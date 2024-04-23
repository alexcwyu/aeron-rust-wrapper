
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Context.h");
        include!("aeron-rust-wrapper/cxx_wrapper/Context.cpp");

        type Context;
        fn say_hello_context();
    }
}