
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Aeron.h");
        include!("aeron-rust-wrapper/cxx_wrapper/Aeron.cpp");

        type Aeron;
        fn say_hello_aeron();

        #[rust_name = "is_closed"]
        fn isClosed(self: Pin<&mut Aeron>) ->bool;
        // fn connect(self: Pin<&mut Aeron>);
    }
}