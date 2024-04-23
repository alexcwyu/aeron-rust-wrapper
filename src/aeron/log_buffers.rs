
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;


        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/LogBuffers.h");
        include!("aeron-rust-wrapper/cxx_wrapper/LogBuffers.cpp");

        type LogBuffers;
        fn say_hello_log_buffers();

        fn get_buffer_from_log_buffers(log_buffers: Pin<&mut LogBuffers>, index: i32) -> UniquePtr<AtomicBuffer>;

    }
}