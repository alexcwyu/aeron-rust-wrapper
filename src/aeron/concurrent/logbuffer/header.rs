use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use cxx::UniquePtr;

use crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;
use crate::aeron::concurrent::logbuffer::header::ffi::CxxHeader;

#[cxx::bridge(namespace = "aeron::concurrent::logbuffer")]
pub mod ffi {
    // Shared structs with fields visible to both languages.

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        #[rust_name = "CxxAtomicBuffer"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/logbuffer/Header.h");
        include!("aeron-rust-wrapper/cxx_wrapper/concurrent/logbuffer/Header.cpp");

        #[rust_name = "CxxHeader"]
        type Header;

        #[rust_name = "initial_term_id"]
        fn initialTermId(self: &CxxHeader) -> i32;
        fn offset(self: &CxxHeader) -> i32;

        #[rust_name = "frame_length"]
        fn frameLength(self: &CxxHeader) -> i32;

        #[rust_name = "session_id"]
        fn sessionId(self: &CxxHeader) -> i32;

        #[rust_name = "stream_id"]
        fn streamId(self: &CxxHeader) -> i32;

        #[rust_name = "term_id"]
        fn termId(self: &CxxHeader) -> i32;

        #[rust_name = "term_offset"]
        fn termOffset(self: &CxxHeader) -> i32;

        #[rust_name = "next_term_offset"]
        fn nextTermOffset(self: &CxxHeader) -> i32;
        fn flags(self: &CxxHeader) -> u8;
        fn position(self: &CxxHeader) -> i64;

        #[rust_name = "position_bits_to_shift"]
        fn positionBitsToShift(self: &CxxHeader) -> i32;

        #[rust_name = "reserved_value"]
        fn reservedValue(self: &CxxHeader) -> i64;

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "empty_instance"]
        fn newInstance() -> UniquePtr<CxxHeader>;

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "new_instance"]
        fn newInstance(initialTermId: i32, positionBitsToShift: i32) -> UniquePtr<CxxHeader>;

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "get_type"]
        fn getType(header: & CxxHeader) -> u16;

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "get_buffer"]
        fn getBuffer(header: & CxxHeader, buffer: & UniquePtr<CxxAtomicBuffer>);

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "copy_from"]
        fn copyFrom(header: Pin<&mut CxxHeader>, src_header: &CxxHeader);

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "set_fragmented_frameL_length"]
        fn fragmentedFrameLength(header: Pin<&mut CxxHeader>, fragmentedFrameLength: i32);

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "set_initial_term_id"]
        fn initialTermId(header: Pin<&mut CxxHeader>, initialTermId : i32);

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "set_offset"]
        fn offset(header: Pin<&mut CxxHeader>, offset: i32);

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "set_buffer"]
        fn buffer(header: Pin<&mut CxxHeader>, buffer: Pin<&mut CxxAtomicBuffer>);

        //void *context() const

    }
    impl SharedPtr<CxxHeader> {}
}

pub struct Header {
    header: UniquePtr<CxxHeader>,
}

impl Header {
    #[inline]
    pub fn new(header: UniquePtr<CxxHeader>) -> Self {
        Self {
            header
        }
    }

    #[inline]
    pub fn empty_instance() -> Self{
        Self {
            header: ffi::empty_instance()
        }
    }

    #[inline]
    pub fn new_instance(initialTermId: i32, positionBitsToShift: i32) -> Self{
        Self {
            header: ffi::new_instance(initialTermId, positionBitsToShift)
        }
    }

    #[inline]
    pub fn initial_term_id(&self) -> i32 {
        self.header.initial_term_id()
    }

    #[inline]
    pub fn offset(&self) -> i32 {
        self.header.offset()
    }

    #[inline]
    pub fn frame_length(&self) -> i32 {
        self.header.frame_length()
    }

    #[inline]
    pub fn session_id(&self) -> i32 {
        self.header.session_id()
    }

    #[inline]
    pub fn stream_id(&self) -> i32 {
        self.header.stream_id()
    }

    #[inline]
    pub fn term_id(&self) -> i32 {
        self.header.term_id()
    }

    #[inline]
    pub fn term_offset(&self) -> i32 {
        self.header.term_offset()
    }

    #[inline]
    pub fn next_term_offset(&self) -> i32 {
        self.header.next_term_offset()
    }

    #[inline]
    pub fn flags(&self) -> u8 {
        self.header.flags()
    }

    #[inline]
    pub fn position(&self) -> i64 {
        self.header.position()
    }

    #[inline]
    pub fn position_bits_to_shift(&self) -> i32 {
        self.header.position_bits_to_shift()
    }

    #[inline]
    pub fn reserved_value(&self) -> i64 {
        self.header.reserved_value()
    }

    #[inline]
    pub fn get_type(&self) -> u16 {
        ffi::get_type(&self.header)
    }

    #[inline]
    pub fn get_buffer(&self, buffer: & UniquePtr<CxxAtomicBuffer>) {
        ffi::get_buffer(&self.header, buffer);
    }

    #[inline]
    pub fn copy_from(&mut self, src_header: &CxxHeader) {
        ffi::copy_from(self.header.as_mut().unwrap(), src_header);
    }

    #[inline]
    pub fn set_fragmented_frameL_length(&mut self, fragmentedFrameLength: i32) {
        ffi::set_fragmented_frameL_length(self.header.as_mut().unwrap(), fragmentedFrameLength);
    }

    #[inline]
    pub fn set_initial_term_id(&mut self, initialTermId: i32) {
        ffi::set_initial_term_id(self.header.as_mut().unwrap(), initialTermId);
    }

    #[inline]
    pub fn set_offset(&mut self, offset: i32) {
        ffi::set_offset(self.header.as_mut().unwrap(), offset);
    }

    #[inline]
    pub fn set_buffer(&mut self, buffer: Pin<& mut ffi::CxxAtomicBuffer>) {
        ffi::set_buffer(self.header.as_mut().unwrap(), buffer);
    }

    #[inline]
    pub fn get_ref(&self) -> &UniquePtr<ffi::CxxHeader> {
        &self.header
    }
    #[inline]
    pub fn as_mut(&mut self) -> Pin<&mut ffi::CxxHeader> {
        self.header.as_mut().unwrap()
    }
}

impl Deref for Header {
    type Target = CxxHeader;

    fn deref(&self) -> &Self::Target {
        match self.header.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null ffi::CxxHeader"
            ),
        }
    }
}

// impl DerefMut for Header {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         match self.header.as_mut() {
//             Some(target) => Pin::into_inner(target),
//             None => panic!(
//                 "called deref_mut on a null ffi::CxxHeader"
//             ),
//         }
//     }
// }


impl From <UniquePtr<ffi::CxxHeader>> for Header{
    fn from(header: UniquePtr<ffi::CxxHeader>) -> Self{
        Self::new(header)
    }
}
