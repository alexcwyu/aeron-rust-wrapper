
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Publication.h");
        include!("aeron-rust-wrapper/cxx_wrapper/Publication.cpp");

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

        //std::vector<std::string> localSocketAddresses() const

        //std::int64_t offer(const concurrent::AtomicBuffer &buffer, util::index_t offset, util::index_t length, const on_reserved_value_supplier_t &reservedValueSupplier)

        //std::int64_t offer(const concurrent::AtomicBuffer &buffer, util::index_t offset, util::index_t length)

        //std::int64_t offer(const concurrent::AtomicBuffer &buffer)

        //template<class BufferIterator> std::int64_t offer( BufferIterator startBuffer, BufferIterator lastBuffer, const on_reserved_value_supplier_t &reservedValueSupplier = DEFAULT_RESERVED_VALUE_SUPPLIER)

        //std::int64_t offer(const concurrent::AtomicBuffer buffers[], std::size_t length, const on_reserved_value_supplier_t &reservedValueSupplier = DEFAULT_RESERVED_VALUE_SUPPLIER)

        //template<std::size_t N> std::int64_t offer( const std::array<concurrent::AtomicBuffer, N> &buffers, const on_reserved_value_supplier_t &reservedValueSupplier = DEFAULT_RESERVED_VALUE_SUPPLIER)

        //std::int64_t tryClaim(util::index_t length, concurrent::logbuffer::BufferClaim &bufferClaim)


        fn say_hello_publication();
    }
}