
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/ExclusivePublication.h");


        type ExclusivePublication;

        fn channel(self: &ExclusivePublication) -> &CxxString;

        #[rust_name = "stream_id"]
        fn streamId(self: &ExclusivePublication) -> i32;

        #[rust_name = "session_id"]
        fn sessionId(self: &ExclusivePublication) -> i32;

        #[rust_name = "initial_term_id"]
        fn initialTermId(self: &ExclusivePublication) -> i32;

        #[rust_name = "term_id"]
        fn termId(self: &ExclusivePublication) -> i32;

        #[rust_name = "term_offset"]
        fn termOffset(self: &ExclusivePublication) -> i32;

        #[rust_name = "original_registration_id"]
        fn originalRegistrationId(self: &ExclusivePublication) -> i64;

        #[rust_name = "registration_id"]
        fn registrationId(self: &ExclusivePublication) -> i64;

        #[rust_name = "max_message_length"]
        fn maxMessageLength(self: &ExclusivePublication) -> i32;

        #[rust_name = "max_payload_length"]
        fn maxPayloadLength(self: &ExclusivePublication) -> i32;

        #[rust_name = "term_buffer_length"]
        fn termBufferLength(self: &ExclusivePublication) -> i32;

        #[rust_name = "position_bits_to_shift"]
        fn positionBitsToShift(self: &ExclusivePublication) -> i32;

        #[rust_name = "is_connected"]
        fn isConnected(self: &ExclusivePublication) -> bool;

        #[rust_name = "is_closed"]
        fn isClosed(self: &ExclusivePublication) -> bool;

        #[rust_name = "max_possible_position"]
        fn maxPossiblePosition(self: &ExclusivePublication) -> i64;
        fn position(self: &ExclusivePublication) -> i64;

        #[rust_name = "publication_limit"]
        fn publicationLimit(self: &ExclusivePublication) -> i64;

        #[rust_name = "publication_limit_id"]
        fn publicationLimitId(self: &ExclusivePublication) -> i32;

        #[rust_name = "available_window"]
        fn availableWindow(self: &ExclusivePublication) -> i64;

        #[rust_name = "channel_status_id"]
        fn channelStatusId(self: &ExclusivePublication) -> i32;

        #[rust_name = "channel_status"]
        fn channelStatus(self: &ExclusivePublication) -> i64;

        #[rust_name = "add_destination"]
        fn addDestination(self: Pin<&mut ExclusivePublication>, endpoint_channel: &CxxString) ->i64;

        #[rust_name = "remove_destination"]
        fn removeDestination(self: Pin<&mut ExclusivePublication>, endpoint_channel: &CxxString) ->i64;

        #[rust_name = "find_destination_response"]
        fn findDestinationResponse(self: Pin<&mut ExclusivePublication>, correlation_id: i64) -> bool;
        fn close(self: Pin<&mut ExclusivePublication>);

        //std::int64_t offer(const concurrent::AtomicBuffer &buffer, util::index_t offset, util::index_t length, const on_reserved_value_supplier_t &reservedValueSupplier)

        //std::int64_t offer(const concurrent::AtomicBuffer &buffer, util::index_t offset, util::index_t length)

        //std::int64_t offer(const concurrent::AtomicBuffer &buffer)

        //template<class BufferIterator> std::int64_t offer(BufferIterator startBuffer, BufferIterator lastBuffer, const on_reserved_value_supplier_t &reservedValueSupplier = DEFAULT_RESERVED_VALUE_SUPPLIER)

        //std::int64_t offer(const concurrent::AtomicBuffer buffers[], std::size_t length, const on_reserved_value_supplier_t &reservedValueSupplier = DEFAULT_RESERVED_VALUE_SUPPLIER)

        //template<std::size_t N> std::int64_t offer(const std::array<concurrent::AtomicBuffer, N> &buffers, const on_reserved_value_supplier_t &reservedValueSupplier = DEFAULT_RESERVED_VALUE_SUPPLIER)

        //std::int64_t tryClaim(util::index_t length, concurrent::logbuffer::BufferClaim &bufferClaim)

        include!("aeron-rust-wrapper/cxx_wrapper/ExclusivePublication.cpp");
        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "say_hello"]
        fn sayHello();
    }

    impl SharedPtr<ExclusivePublication> {}
}