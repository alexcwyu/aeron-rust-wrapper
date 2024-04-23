use cxx::ExternType;

#[cxx::bridge(namespace = "aeron::concurrent")]
pub mod ffi{
    // Shared structs with fields visible to both languages.

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/AtomicBuffer.h");
        include!("aeron-rust-wrapper/cxx_wrapper/concurrent/AtomicBuffer.cpp");

        type AtomicBuffer;

        fn capacity(&self) -> i32;

        fn buffer(&self) -> *mut u8;

        #[rust_name = "put_i32"]
        fn putInt32(self: Pin<&mut AtomicBuffer>, offset: i32, value: i32);
        #[rust_name = "get_i32"]
        fn getInt32(&self, offset: i32) -> i32;

        #[rust_name = "put_i64"]
        fn putInt64(self: Pin<&mut AtomicBuffer>, offset: i32, value: i64);

        #[rust_name = "get_i64"]
        fn getInt64(&self, offset: i32) -> i64;

        //void capacity(std::size_t length)

        //template <typename struct_t> struct_t &overlayStruct()

        unsafe fn new_atomic_buffer(buffer: *mut u8, length: usize) -> UniquePtr<AtomicBuffer>;

        fn wrap_atomic_buffer(buffer : &AtomicBuffer) -> UniquePtr<AtomicBuffer>;
    }
}

