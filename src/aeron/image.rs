use std::ops::Deref;

#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;
        #[namespace = "aeron::concurrent::logbuffer"]
        type Header = crate::aeron::concurrent::logbuffer::header::ffi::Header;
        #[namespace = "aeron"]
        type LogBuffers = crate::aeron::log_buffers::ffi::LogBuffers;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Image.h");

        type Image;

        type ControlledPollAction;

        #[rust_name = "term_buffer_length"]
        fn termBufferLength(self: &Image) -> i32;

        #[rust_name = "position_bits_to_shift"]
        fn positionBitsToShift(self: &Image) -> i32;

        #[rust_name = "session_id"]
        fn sessionId(self: &Image) -> i32;

        #[rust_name = "correlation_id"]
        fn correlationId(self: &Image) -> i64;

        #[rust_name = "subscription_registration_id"]
        fn subscriptionRegistrationId(self: &Image) -> i64;

        #[rust_name = "join_position"]
        fn joinPosition(self: &Image) -> i64;

        #[rust_name = "initial_term_id"]
        fn initialTermId(self: &Image) -> i32;

        #[rust_name = "is_closed"]
        fn isClosed(self: &Image) -> bool;
        fn position(self: &Image) -> i64;

        #[rust_name = "subscriber_position_id"]
        fn subscriberPositionId(self: &Image) -> i32;

        #[rust_name = "is_end_of_stream"]
        fn isEndOfStream(self: &Image) -> bool;

        #[rust_name = "end_of_stream_position"]
        fn endOfStreamPosition(self: &Image) -> i64;

        #[rust_name = "active_transport_count"]
        fn activeTransportCount(self: &Image) -> i32;
        fn close(self: Pin<&mut Image>);

        //std::string sourceIdentity() const
        //void position(std::int64_t newPosition)

        include!("aeron-rust-wrapper/cxx_wrapper/Image.cpp");

        #[namespace = "aeron::image"]
        fn poll(image: Pin<&mut Image>, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> (), fragmentLimit: i32) -> i32;

        #[namespace = "aeron::image"]
        #[rust_name = "bounded_poll"]
        fn boundedPoll(image: Pin<&mut Image>, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> (), limitPosition: i64, fragmentLimit: i32) -> i32;

        #[namespace = "aeron::image"]
        #[rust_name = "controlled_poll"]
        fn controlledPoll(image: Pin<&mut Image>, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> ControlledPollAction, fragmentLimit: i32) -> i32;

        #[namespace = "aeron::image"]
        #[rust_name = "bounded_controlled_poll"]
        fn boundedControlledPoll(image: Pin<&mut Image>, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> ControlledPollAction, limitPosition: i64, fragmentLimit: i32) -> i32;

        #[namespace = "aeron::image"]
        #[rust_name = "controlled_peek"]
        fn controlledPeek(image: Pin<&mut Image>, initialPosition: i64, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> ControlledPollAction, limitPosition: i64) -> i64;

        #[namespace = "aeron::image"]
        #[rust_name = "block_poll"]
        fn blockPoll(image: Pin<&mut Image>, blockHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, sessionId: i32, termId: i32) -> (), blockLengthLimit: i32) -> i32;

        #[namespace = "aeron::image"]
        #[rust_name = "say_hello"]
        fn sayHello();
    }
}