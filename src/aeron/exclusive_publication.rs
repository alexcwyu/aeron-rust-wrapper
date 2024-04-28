use std::pin::Pin;

use cxx::{CxxVector, SharedPtr};

use crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;
use crate::aeron::concurrent::logbuffer::buffer_claim::ffi::BufferClaim;

#[cxx::bridge(namespace = "aeron")]
pub(crate) mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

        #[namespace = "aeron::concurrent::logbuffer"]
        type BufferClaim = crate::aeron::concurrent::logbuffer::buffer_claim::ffi::BufferClaim;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/ExclusivePublication.h");

        type ExclusivePublication;

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

        //std::int64_t offer(const concurrent::AtomicBuffer &buffer, util::index_t offset, util::index_t length, const on_reserved_value_supplier_t &reservedValueSupplier)

        //std::int64_t offer(const concurrent::AtomicBuffer &buffer, util::index_t offset, util::index_t length)

        //std::int64_t offer(const concurrent::AtomicBuffer &buffer)

        //template<class BufferIterator> std::int64_t offer(BufferIterator startBuffer, BufferIterator lastBuffer, const on_reserved_value_supplier_t &reservedValueSupplier = DEFAULT_RESERVED_VALUE_SUPPLIER)

        //std::int64_t offer(const concurrent::AtomicBuffer buffers[], std::size_t length, const on_reserved_value_supplier_t &reservedValueSupplier = DEFAULT_RESERVED_VALUE_SUPPLIER)

        //template<std::size_t N> std::int64_t offer(const std::array<concurrent::AtomicBuffer, N> &buffers, const on_reserved_value_supplier_t &reservedValueSupplier = DEFAULT_RESERVED_VALUE_SUPPLIER)

        //std::int64_t tryClaim(util::index_t length, concurrent::logbuffer::BufferClaim &bufferClaim)

        include!("aeron-rust-wrapper/cxx_wrapper/ExclusivePublication.cpp");

        #[namespace = "aeron::exclusive_publication"]
        fn channel(publication: &SharedPtr<ExclusivePublication>) -> String;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "add_destination"]
        fn addDestination(publication: &SharedPtr<ExclusivePublication>, endpoint_channel: String) ->i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "remove_destination"]
        fn removeDestination(publication: &SharedPtr<ExclusivePublication>, endpoint_channel: String) ->i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "find_destination_response"]
        fn findDestinationResponse(publication: &SharedPtr<ExclusivePublication>, correlation_id: i64) -> bool;

        #[namespace = "aeron::exclusive_publication"]
        fn close(publication: &SharedPtr<ExclusivePublication>);

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "offer_part"]
        fn offer(publication: &SharedPtr<ExclusivePublication>, buffer: &AtomicBuffer, offset: i32, length: i32) -> i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "offer"]
        fn offer(publication: &SharedPtr<ExclusivePublication>, buffer: &AtomicBuffer) -> i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "try_claim"]
        fn tryClaim(publication: &SharedPtr<ExclusivePublication>, length: i32, bufferClaim : Pin<&mut BufferClaim>) ->i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "offer_opt"]
        fn offer(publication: &SharedPtr<ExclusivePublication>, buffer: &AtomicBuffer, offset: i32, length: i32, reservedValueSupplier: fn(buffer: Pin<&mut AtomicBuffer>, offset: i32, length: i32) -> i64) -> i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "offer_bulk"]
        fn offer(publication: &SharedPtr<ExclusivePublication>, buffer: &CxxVector<AtomicBuffer>, reservedValueSupplier: fn(buffer: Pin<&mut AtomicBuffer>, offset: i32, length: i32) -> i64) -> i64;

    }

    impl SharedPtr<ExclusivePublication> {}
}


unsafe impl Sync for ffi::ExclusivePublication {}
unsafe impl Send for ffi::ExclusivePublication {}
#[derive(Clone)]
pub struct ExclusivePublication {
    publication: SharedPtr<ffi::ExclusivePublication>,
}

impl ExclusivePublication {
    #[inline]
    pub fn new(publication: SharedPtr<ffi::ExclusivePublication>) -> Self {
        Self {
            publication
        }
    }

    #[inline]
    pub fn stream_id(&self) -> i32 {
        self.publication.stream_id()
    }

    #[inline]
    pub fn session_id(&self) -> i32 {
        self.publication.session_id()
    }

    #[inline]
    pub fn initial_term_id(&self) -> i32 {
        self.publication.initial_term_id()
    }

    #[inline]
    pub fn term_id(&self) -> i32 {
        self.publication.term_id()
    }

    #[inline]
    pub fn term_offset(&self) -> i32 {
        self.publication.term_offset()
    }

    #[inline]
    pub fn original_registration_id(&self) -> i64 {
        self.publication.original_registration_id()
    }

    #[inline]
    pub fn registration_id(&self) -> i64 {
        self.publication.registration_id()
    }

    #[inline]
    pub fn max_message_length(&self) -> i32 {
        self.publication.max_message_length()
    }

    #[inline]
    pub fn max_payload_length(&self) -> i32 {
        self.publication.max_payload_length()
    }

    #[inline]
    pub fn term_buffer_length(&self) -> i32 {
        self.publication.term_buffer_length()
    }

    #[inline]
    pub fn position_bits_to_shift(&self) -> i32 {
        self.publication.position_bits_to_shift()
    }

    #[inline]
    pub fn is_connected(&self) -> bool {
        self.publication.is_connected()
    }

    #[inline]
    pub fn is_closed(&self) -> bool {
        self.publication.is_closed()
    }

    #[inline]
    pub fn max_possible_position(&self) -> i64 {
        self.publication.max_possible_position()
    }

    #[inline]
    pub fn position(&self) -> i64 {
        self.publication.position()
    }

    #[inline]
    pub fn publication_limit(&self) -> i64 {
        self.publication.publication_limit()
    }

    #[inline]
    pub fn publication_limit_id(&self) -> i32 {
        self.publication.publication_limit_id()
    }

    #[inline]
    pub fn available_window(&self) -> i64 {
        self.publication.available_window()
    }

    #[inline]
    pub fn channel_status_id(&self) -> i32 {
        self.publication.channel_status_id()
    }

    #[inline]
    pub fn channel_status(&self) -> i64 {
        self.publication.channel_status()
    }

    #[inline]
    pub fn add_destination(&self, endpoint_channel: String) -> i64 {
        ffi::add_destination(&self.publication, endpoint_channel)
    }

    #[inline]
    pub fn remove_destination(&self, endpoint_channel: String) -> i64 {
        ffi::remove_destination(&self.publication, endpoint_channel)
    }

    #[inline]
    pub fn find_destination_response(&self, correlation_id: i64) -> bool {
        ffi::find_destination_response(&self.publication, correlation_id)
    }

    #[inline]
    pub fn close(&self) {
        ffi::close(&self.publication);
    }

    #[inline]
    pub fn offer_part(&self, buffer: &AtomicBuffer, offset: i32, length: i32) -> i64 {
        ffi::offer_part(&self.publication, buffer, offset, length)
    }

    #[inline]
    pub fn offer(&self, buffer: &AtomicBuffer) -> i64 {
        ffi::offer(&self.publication, buffer)
    }

    #[inline]
    pub fn try_claim(&self, length: i32, buffer_claim: Pin<&mut BufferClaim>) -> i64 {
        ffi::try_claim(&self.publication, length, buffer_claim)
    }

    #[inline]
    pub fn offer_opt(&self, buffer: &AtomicBuffer, offset: i32, length: i32, reserved_value_supplier: fn(buffer: Pin<&mut AtomicBuffer>, offset: i32, length: i32) -> i64) -> i64 {
        ffi::offer_opt(&self.publication, buffer, offset, length, reserved_value_supplier)
    }

    #[inline]
    pub fn offer_bulk(&self, buffer: &CxxVector<AtomicBuffer>, reserved_value_supplier: fn(buffer: Pin<&mut AtomicBuffer>, offset: i32, length: i32) -> i64) -> i64 {
        ffi::offer_bulk(&self.publication, buffer, reserved_value_supplier)
    }

    pub fn get_ref(&self) -> &SharedPtr<ffi::ExclusivePublication> {
        &self.publication
    }
}


impl From <SharedPtr<ffi::ExclusivePublication>> for ExclusivePublication{
    fn from(publication: SharedPtr<ffi::ExclusivePublication>) -> Self{
        Self::new(publication)
    }
}
