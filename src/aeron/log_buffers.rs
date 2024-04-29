
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type CxxAtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;


        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/LogBuffers.h");
        #[rust_name = "CxxLogBuffers"]
        type LogBuffers;


        include!("aeron-rust-wrapper/cxx_wrapper/LogBuffers.cpp");


        #[namespace = "aeron::logbuffers"]
        #[rust_name = "atomic_buffer"]
        fn atomicBuffer(log_buffers: Pin<&mut CxxLogBuffers>, index: i32) -> UniquePtr<CxxAtomicBuffer>;

    }

    impl SharedPtr<CxxLogBuffers> {}
}