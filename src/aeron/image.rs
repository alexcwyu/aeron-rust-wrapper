use std::ops::Deref;

use cxx::SharedPtr;

use crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;
use crate::aeron::concurrent::logbuffer::header::ffi::CxxHeader;
use crate::aeron::image::ffi::{CxxControlledPollAction, CxxImage};

#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        #[rust_name = "CxxAtomicBuffer"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;
        #[namespace = "aeron::concurrent::logbuffer"]
        #[rust_name = "CxxHeader"]
        type Header = crate::aeron::concurrent::logbuffer::header::ffi::CxxHeader;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Image.h");

        include!("aeron-rust-wrapper/cxx_wrapper/Image.cpp");


        #[rust_name = "CxxImage"]
        type Image;

        #[rust_name = "CxxControlledPollAction"]
        type ControlledPollAction;

        #[rust_name = "term_buffer_length"]
        fn termBufferLength(self: &CxxImage) -> i32;

        #[rust_name = "position_bits_to_shift"]
        fn positionBitsToShift(self: &CxxImage) -> i32;

        #[rust_name = "session_id"]
        fn sessionId(self: &CxxImage) -> i32;

        #[rust_name = "correlation_id"]
        fn correlationId(self: &CxxImage) -> i64;

        #[rust_name = "subscription_registration_id"]
        fn subscriptionRegistrationId(self: &CxxImage) -> i64;

        #[rust_name = "join_position"]
        fn joinPosition(self: &CxxImage) -> i64;

        #[rust_name = "initial_term_id"]
        fn initialTermId(self: &CxxImage) -> i32;

        #[rust_name = "is_closed"]
        fn isClosed(self: &CxxImage) -> bool;
        fn position(self: &CxxImage) -> i64;

        #[rust_name = "subscriber_position_id"]
        fn subscriberPositionId(self: &CxxImage) -> i32;

        #[rust_name = "is_end_of_stream"]
        fn isEndOfStream(self: &CxxImage) -> bool;

        #[rust_name = "end_of_stream_position"]
        fn endOfStreamPosition(self: &CxxImage) -> i64;

        #[rust_name = "active_transport_count"]
        fn activeTransportCount(self: &CxxImage) -> i32;
        #[namespace = "aeron::image"]
        #[rust_name = "source_identity"]
        fn sourceIdentity(publication: &SharedPtr<CxxImage>) -> String;
        #[namespace = "aeron::image"]
        #[rust_name = "set_position"]
        fn position(publication: &SharedPtr<CxxImage>, newPosition: i64);
        #[namespace = "aeron::image"]
        fn close(publication: &SharedPtr<CxxImage>);
        #[namespace = "aeron::image"]
        fn poll(publication: &SharedPtr<CxxImage>, fragmentHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> (), fragmentLimit: i32) -> i32;

        #[namespace = "aeron::image"]
        #[rust_name = "bounded_poll"]
        fn boundedPoll(publication: &SharedPtr<CxxImage>, fragmentHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> (), limitPosition: i64, fragmentLimit: i32) -> i32;

        #[namespace = "aeron::image"]
        #[rust_name = "controlled_poll"]
        fn controlledPoll(publication: &SharedPtr<CxxImage>, fragmentHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> CxxControlledPollAction, fragmentLimit: i32) -> i32;

        #[namespace = "aeron::image"]
        #[rust_name = "bounded_controlled_poll"]
        fn boundedControlledPoll(publication: &SharedPtr<CxxImage>, fragmentHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> CxxControlledPollAction, limitPosition: i64, fragmentLimit: i32) -> i32;

        #[namespace = "aeron::image"]
        #[rust_name = "controlled_peek"]
        fn controlledPeek(publication: &SharedPtr<CxxImage>, initialPosition: i64, fragmentHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> CxxControlledPollAction, limitPosition: i64) -> i64;

        #[namespace = "aeron::image"]
        #[rust_name = "block_poll"]
        fn blockPoll(publication: &SharedPtr<CxxImage>, blockHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, sessionId: i32, termId: i32) -> (), blockLengthLimit: i32) -> i32;

    }

    impl SharedPtr<CxxImage> {}
}


unsafe impl Sync for ffi::CxxImage {}
unsafe impl Send for ffi::CxxImage {}


#[derive(Clone)]
pub struct Image {
    image: SharedPtr<ffi::CxxImage>,
}

impl Image {
    #[inline]
    pub fn new(image: SharedPtr<ffi::CxxImage>) -> Self {
        Self {
            image
        }
    }

    #[inline]
    pub fn term_buffer_length(&self) -> i32 {
        self.image.term_buffer_length()
    }

    #[inline]
    pub fn position_bits_to_shift(&self) -> i32 {
        self.image.position_bits_to_shift()
    }

    #[inline]
    pub fn session_id(&self) -> i32 {
        self.image.session_id()
    }

    #[inline]
    pub fn correlation_id(&self) -> i64 {
        self.image.correlation_id()
    }

    #[inline]
    pub fn subscription_registration_id(&self) -> i64 {
        self.image.subscription_registration_id()
    }

    #[inline]
    pub fn join_position(&self) -> i64 {
        self.image.join_position()
    }

    #[inline]
    pub fn initial_term_id(&self) -> i32 {
        self.image.initial_term_id()
    }

    #[inline]
    pub fn is_closed(&self) -> bool {
        self.image.is_closed()
    }

    #[inline]
    pub fn position(&self) -> i64 {
        self.image.position()
    }

    #[inline]
    pub fn subscriber_position_id(&self) -> i32 {
        self.image.subscriber_position_id()
    }

    #[inline]
    pub fn is_end_of_stream(&self) -> bool {
        self.image.is_end_of_stream()
    }

    #[inline]
    pub fn end_of_stream_position(&self) -> i64 {
        self.image.end_of_stream_position()
    }

    #[inline]
    pub fn active_transport_count(&self) -> i32 {
        self.image.active_transport_count()
    }

    #[inline]
    pub fn source_identity(&self) -> String {
        ffi::source_identity(&self.image)
    }

    #[inline]
    pub fn set_position(&self, newPosition: i64) {
        ffi::set_position(&self.image, newPosition);
    }

    #[inline]
    pub fn close(&self) {
        ffi::close(&self.image);
    }

    #[inline]
    pub fn poll(&self, fragment_handler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> (), fragment_limit: i32) -> i32 {
        ffi::poll(&self.image, fragment_handler, fragment_limit)
    }

    #[inline]
    pub fn bounded_poll(&self, fragment_handler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> (), limit_position: i64, fragment_limit: i32) -> i32 {
        ffi::bounded_poll(&self.image, fragment_handler, limit_position, fragment_limit)
    }

    #[inline]
    pub fn controlled_poll(&self, fragment_handler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> CxxControlledPollAction, fragment_limit: i32) -> i32 {
        ffi::controlled_poll(&self.image, fragment_handler, fragment_limit)
    }

    #[inline]
    pub fn bounded_controlled_poll(&self, fragment_handler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> CxxControlledPollAction, limit_position: i64, fragment_limit: i32) -> i32 {
        ffi::bounded_controlled_poll(&self.image, fragment_handler, limit_position, fragment_limit)
    }

    #[inline]
    pub fn controlled_peek(&self, initial_position: i64, fragment_handler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> CxxControlledPollAction, limit_position: i64) -> i64 {
        ffi::controlled_peek(&self.image, initial_position, fragment_handler, limit_position)
    }

    #[inline]
    pub fn block_poll(&self, block_handler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, session_id: i32, term_id: i32) -> (), block_length_limit: i32) -> i32 {
        ffi::block_poll(&self.image, block_handler, block_length_limit)
    }

    pub fn get_ref(&self) -> &SharedPtr<CxxImage> {
        &self.image
    }
}


impl Deref for Image {
    type Target = CxxImage;

    fn deref(&self) -> &Self::Target {
        match self.image.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null ffi::CxxImage"
            ),
        }
    }
}

impl From <SharedPtr<CxxImage>> for Image{
    fn from(image: SharedPtr<CxxImage>) -> Self{
        Self::new(image)
    }
}