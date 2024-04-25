
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;


        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Counter.h");
        type Counter;
        #[rust_name = "registration_id"]
        fn registrationId(self: &Counter) -> i64;
        fn state(self: &Counter) -> i32;
        
        // fn label(self: &Counter) -> &CxxString;
        #[rust_name = "is_closed"]
        fn isClosed(self: &Counter) -> bool;

        fn close(self: Pin<&mut Counter>);




        include!("aeron-rust-wrapper/cxx_wrapper/Counter.cpp");

        #[namespace = "aeron::counter"]
        #[rust_name = "say_hello"]
        fn sayHello();

    }

    impl SharedPtr<Counter> {}
}