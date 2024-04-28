
#[cxx::bridge(namespace = "aeron")]
pub(crate) mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;


        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/LogBuffers.h");
        type LogBuffers;


        include!("aeron-rust-wrapper/cxx_wrapper/LogBuffers.cpp");


        #[namespace = "aeron::logbuffers"]
        #[rust_name = "atomic_buffer"]
        fn atomicBuffer(log_buffers: Pin<&mut LogBuffers>, index: i32) -> UniquePtr<AtomicBuffer>;

    }

    impl SharedPtr<LogBuffers> {}
}