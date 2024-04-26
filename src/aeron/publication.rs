
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {

        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;


        #[namespace = "aeron::concurrent::logbuffer"]
        type BufferClaim = crate::aeron::concurrent::logbuffer::buffer_claim::ffi::BufferClaim;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Publication.h");

        type Publication;

        fn channel(self: &Publication) -> &CxxString;

        #[rust_name = "stream_id"]
        fn streamId(self: &Publication) -> i32;

        #[rust_name = "session_id"]
        fn sessionId(self: &Publication) -> i32;

        #[rust_name = "initial_term_id"]
        fn initialTermId(self: &Publication) -> i32;

        #[rust_name = "original_registration_id"]
        fn originalRegistrationId(self: &Publication) -> i64;

        #[rust_name = "registration_id"]
        fn registrationId(self: &Publication) -> i64;

        #[rust_name = "is_original"]
        fn isOriginal(self: &Publication) -> bool;

        #[rust_name = "max_message_length"]
        fn maxMessageLength(self: &Publication) -> i32;

        #[rust_name = "max_payload_length"]
        fn maxPayloadLength(self: &Publication) -> i32;

        #[rust_name = "term_buffer_length"]
        fn termBufferLength(self: &Publication) -> i32;

        #[rust_name = "position_bits_to_shift"]
        fn positionBitsToShift(self: &Publication) -> i32;

        #[rust_name = "is_connected"]
        fn isConnected(self: &Publication) -> bool;

        #[rust_name = "is_closed"]
        fn isClosed(self: &Publication) -> bool;

        #[rust_name = "max_possible_position"]
        fn maxPossiblePosition(self: &Publication) -> i64;
        fn position(self: &Publication) -> i64;

        #[rust_name = "publication_limit"]
        fn publicationLimit(self: &Publication) -> i64;

        #[rust_name = "publication_limit_id"]
        fn publicationLimitId(self: &Publication) -> i32;

        #[rust_name = "available_window"]
        fn availableWindow(self: &Publication) -> i64;

        #[rust_name = "channel_status_id"]
        fn channelStatusId(self: &Publication) -> i32;

        #[rust_name = "channel_status"]
        fn channelStatus(self: &Publication) -> i64;

        #[rust_name = "add_destination"]
        fn addDestination(self: Pin<&mut Publication>, endpoint_channel: &CxxString) ->i64;
        #[rust_name = "remove_destination"]
        fn removeDestination(self: Pin<&mut Publication>, endpoint_channel: &CxxString) ->i64;

        #[rust_name = "find_destination_response"]
        fn findDestinationResponse(self: Pin<&mut Publication>, correlation_id: i64) -> bool;
        fn close(self: Pin<&mut Publication>);

        #[rust_name = "offer_part"]
        fn offer(self: Pin<&mut Publication>, buffer: &AtomicBuffer, offset: i32, length: i32) -> i64;

        #[rust_name = "offer"]
        fn offer(self: Pin<&mut Publication>, buffer: &AtomicBuffer) -> i64;

        #[rust_name = "try_claim"]
        fn tryClaim(self: Pin<&mut Publication>, length: i32, bufferClaim : Pin<&mut BufferClaim>) ->i64;


        //std::vector<std::string> localSocketAddresses() const


        include!("aeron-rust-wrapper/cxx_wrapper/Publication.cpp");
        #[namespace = "aeron::publication"]
        #[rust_name = "offer_opt"]
        fn offer(publication: Pin<&mut Publication>, buffer: &AtomicBuffer, offset: i32, length: i32, reservedValueSupplier: fn(buffer: Pin<&mut AtomicBuffer>, offset: i32, length: i32) -> i64) -> i64;

        #[namespace = "aeron::publication"]
        #[rust_name = "offer_bulk"]
        fn offer(publication: Pin<&mut Publication>, buffer: &CxxVector<AtomicBuffer>, reservedValueSupplier: fn(buffer: Pin<&mut AtomicBuffer>, offset: i32, length: i32) -> i64) -> i64;

        #[namespace = "aeron::publication"]
        #[rust_name = "say_hello"]
        fn sayHello();
    }

    impl SharedPtr<Publication> {}
}
