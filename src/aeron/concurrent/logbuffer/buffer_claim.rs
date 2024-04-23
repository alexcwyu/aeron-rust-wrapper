

#[cxx::bridge(namespace = "aeron::concurrent::logbuffer")]
pub mod ffi {
    // Shared structs with fields visible to both languages.

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/logbuffer/BufferClaim.h");

        type BufferClaim;

        fn wrap(self: Pin<&mut BufferClaim>, buffer : Pin<&mut AtomicBuffer>, offset: i32, length: i32);

        fn offset(self: &BufferClaim) -> i32;
        fn length(self: &BufferClaim) -> i32;
        fn flags(self: &BufferClaim) -> u8;
        #[rust_name = "header_type"]
        fn headerType(self: &BufferClaim) -> u16;
        #[rust_name = "reserved_value"]
        fn reservedValue(self: &BufferClaim) -> i64;
        fn commit(self: Pin<&mut BufferClaim>);
        fn abort(self: Pin<&mut BufferClaim>);

        //this_t& flags(const std::uint8_t flags)

        //this_t &reservedValue(const std::int64_t value)

        include!("aeron-rust-wrapper/cxx_wrapper/concurrent/logbuffer/BufferClaim.cpp");
        #[namespace = "aeron::concurrent::logbuffer::buffer_claim"]
        #[rust_name = "new_instance"]
        fn newInstance() -> UniquePtr<BufferClaim>;

        #[namespace = "aeron::concurrent::logbuffer::buffer_claim"]
        #[rust_name = "wrap_raw_buffer"]
        unsafe fn wrapRawBuffer(buffer_claim: Pin<&mut BufferClaim>, buffer: *mut u8, length: i32);

        #[namespace = "aeron::concurrent::logbuffer::buffer_claim"]
        fn buffer(buffer_claim: Pin<&mut BufferClaim>) -> UniquePtr<AtomicBuffer>;

        #[namespace = "aeron::concurrent::logbuffer::buffer_claim"]
        #[rust_name = "say_hello"]
        fn sayHello();
    }
}