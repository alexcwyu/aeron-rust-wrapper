use std::ops::Deref;
use std::pin::Pin;

use cxx::{CxxVector, SharedPtr};

use crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;
use crate::aeron::concurrent::logbuffer::buffer_claim::ffi::CxxBufferClaim;

#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        #[rust_name = "CxxAtomicBuffer"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;

        #[namespace = "aeron::concurrent::logbuffer"]
        #[rust_name = "CxxBufferClaim"]
        type BufferClaim = crate::aeron::concurrent::logbuffer::buffer_claim::ffi::CxxBufferClaim;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/ExclusivePublication.h");
        include!("aeron-rust-wrapper/cxx_wrapper/ExclusivePublication.cpp");

        #[rust_name = "CxxExclusivePublication"]
        type ExclusivePublication;

        #[rust_name = "stream_id"]
        fn streamId(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "session_id"]
        fn sessionId(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "initial_term_id"]
        fn initialTermId(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "term_id"]
        fn termId(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "term_offset"]
        fn termOffset(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "original_registration_id"]
        fn originalRegistrationId(self: &CxxExclusivePublication) -> i64;

        #[rust_name = "registration_id"]
        fn registrationId(self: &CxxExclusivePublication) -> i64;

        #[rust_name = "max_message_length"]
        fn maxMessageLength(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "max_payload_length"]
        fn maxPayloadLength(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "term_buffer_length"]
        fn termBufferLength(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "position_bits_to_shift"]
        fn positionBitsToShift(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "is_connected"]
        fn isConnected(self: &CxxExclusivePublication) -> bool;

        #[rust_name = "is_closed"]
        fn isClosed(self: &CxxExclusivePublication) -> bool;

        #[rust_name = "max_possible_position"]
        fn maxPossiblePosition(self: &CxxExclusivePublication) -> i64;
        fn position(self: &CxxExclusivePublication) -> i64;

        #[rust_name = "publication_limit"]
        fn publicationLimit(self: &CxxExclusivePublication) -> i64;

        #[rust_name = "publication_limit_id"]
        fn publicationLimitId(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "available_window"]
        fn availableWindow(self: &CxxExclusivePublication) -> i64;

        #[rust_name = "channel_status_id"]
        fn channelStatusId(self: &CxxExclusivePublication) -> i32;

        #[rust_name = "channel_status"]
        fn channelStatus(self: &CxxExclusivePublication) -> i64;
        #[namespace = "aeron::exclusive_publication"]
        fn channel(publication: &SharedPtr<CxxExclusivePublication>) -> String;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "add_destination"]
        fn addDestination(publication: &SharedPtr<CxxExclusivePublication>, endpoint_channel: String) ->i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "remove_destination"]
        fn removeDestination(publication: &SharedPtr<CxxExclusivePublication>, endpoint_channel: String) ->i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "find_destination_response"]
        fn findDestinationResponse(publication: &SharedPtr<CxxExclusivePublication>, correlation_id: i64) -> bool;

        #[namespace = "aeron::exclusive_publication"]
        fn close(publication: &SharedPtr<CxxExclusivePublication>);

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "offer_part"]
        fn offer(publication: &SharedPtr<CxxExclusivePublication>, buffer: &CxxAtomicBuffer, offset: i32, length: i32) -> i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "offer"]
        fn offer(publication: &SharedPtr<CxxExclusivePublication>, buffer: &CxxAtomicBuffer) -> i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "try_claim"]
        fn tryClaim(publication: &SharedPtr<CxxExclusivePublication>, length: i32, bufferClaim : Pin<&mut CxxBufferClaim>) ->i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "offer_opt"]
        fn offer(publication: &SharedPtr<CxxExclusivePublication>, buffer: &CxxAtomicBuffer, offset: i32, length: i32, reservedValueSupplier: fn(buffer: Pin<&mut CxxAtomicBuffer>, offset: i32, length: i32) -> i64) -> i64;

        #[namespace = "aeron::exclusive_publication"]
        #[rust_name = "offer_bulk"]
        fn offer(publication: &SharedPtr<CxxExclusivePublication>, buffer: &CxxVector<CxxAtomicBuffer>, reservedValueSupplier: fn(buffer: Pin<&mut CxxAtomicBuffer>, offset: i32, length: i32) -> i64) -> i64;

    }

    impl SharedPtr<CxxExclusivePublication> {}
}


unsafe impl Sync for ffi::CxxExclusivePublication {}
unsafe impl Send for ffi::CxxExclusivePublication {}
#[derive(Clone)]
pub struct ExclusivePublication {
    publication: SharedPtr<ffi::CxxExclusivePublication>,
}

impl ExclusivePublication {
    #[inline]
    pub fn new(publication: SharedPtr<ffi::CxxExclusivePublication>) -> Self {
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
    pub fn offer_part(&self, buffer: &CxxAtomicBuffer, offset: i32, length: i32) -> i64 {
        ffi::offer_part(&self.publication, buffer, offset, length)
    }

    #[inline]
    pub fn offer(&self, buffer: &CxxAtomicBuffer) -> i64 {
        ffi::offer(&self.publication, buffer)
    }

    #[inline]
    pub fn try_claim(&self, length: i32, buffer_claim: Pin<&mut CxxBufferClaim>) -> i64 {
        ffi::try_claim(&self.publication, length, buffer_claim)
    }

    #[inline]
    pub fn offer_opt(&self, buffer: &CxxAtomicBuffer, offset: i32, length: i32, reserved_value_supplier: fn(buffer: Pin<&mut CxxAtomicBuffer>, offset: i32, length: i32) -> i64) -> i64 {
        ffi::offer_opt(&self.publication, buffer, offset, length, reserved_value_supplier)
    }

    #[inline]
    pub fn offer_bulk(&self, buffer: &CxxVector<CxxAtomicBuffer>, reserved_value_supplier: fn(buffer: Pin<&mut CxxAtomicBuffer>, offset: i32, length: i32) -> i64) -> i64 {
        ffi::offer_bulk(&self.publication, buffer, reserved_value_supplier)
    }

    pub fn get_ref(&self) -> &SharedPtr<ffi::CxxExclusivePublication> {
        &self.publication
    }
}

impl Deref for ExclusivePublication {
    type Target = ffi::CxxExclusivePublication;

    fn deref(&self) -> &Self::Target {
        match self.publication.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null ffi::CxxExclusivePublication"
            ),
        }
    }
}

impl From <SharedPtr<ffi::CxxExclusivePublication>> for ExclusivePublication{
    fn from(publication: SharedPtr<ffi::CxxExclusivePublication>) -> Self{
        Self::new(publication)
    }
}
