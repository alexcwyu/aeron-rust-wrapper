

#[cxx::bridge(namespace = "aeron::concurrent::logbuffer")]
pub mod ffi {
    // Shared structs with fields visible to both languages.

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/logbuffer/Header.h");
        include!("aeron-rust-wrapper/cxx_wrapper/concurrent/logbuffer/Header.cpp");
        type Header;

        #[rust_name = "initial_term_id"]
        fn initialTermId(self: &Header) -> i32;
        fn offset(self: &Header) -> i32;

        #[rust_name = "frame_length"]
        fn frameLength(self: &Header) -> i32;

        #[rust_name = "session_id"]
        fn sessionId(self: &Header) -> i32;

        #[rust_name = "stream_id"]
        fn streamId(self: &Header) -> i32;

        #[rust_name = "term_id"]
        fn termId(self: &Header) -> i32;

        #[rust_name = "term_offset"]
        fn termOffset(self: &Header) -> i32;

        #[rust_name = "next_term_offset"]
        fn nextTermOffset(self: &Header) -> i32;
        fn flags(self: &Header) -> u8;
        fn position(self: &Header) -> i64;

        #[rust_name = "position_bits_to_shift"]
        fn positionBitsToShift(self: &Header) -> i32;

        #[rust_name = "reserved_value"]
        fn reservedValue(self: &Header) -> i64;

        //void fragmentedFrameLength(std::int32_t fragmentedFrameLength)

        //void initialTermId(std::int32_t initialTermId)

        //void offset(util::index_t offset)

        //void buffer(AtomicBuffer &buffer)

        //std::uint16_t type() const

        //void *context() const

        fn get_buffer_from_header(header: Pin<&mut Header>) -> UniquePtr<AtomicBuffer>;
        fn say_hello_header();
    }
}