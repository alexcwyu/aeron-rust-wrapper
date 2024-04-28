use cxx::UniquePtr;

use crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

#[cxx::bridge(namespace = "aeron::concurrent::logbuffer")]
pub mod ffi {
    // Shared structs with fields visible to both languages.

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/logbuffer/Header.h");

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



        include!("aeron-rust-wrapper/cxx_wrapper/concurrent/logbuffer/Header.cpp");

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "get_type"]
        fn getType(header: & Header) -> u16;

        #[namespace = "aeron::concurrent::logbuffer::header"]
        #[rust_name = "get_buffer"]
        fn getBuffer(header: & Header, buffer: & UniquePtr<AtomicBuffer>);


        //void fragmentedFrameLength(std::int32_t fragmentedFrameLength)

        //void initialTermId(std::int32_t initialTermId)

        //void offset(util::index_t offset)

        //void buffer(AtomicBuffer &buffer)

        //std::uint16_t getType() const

        //void *context() const

    }
}

pub struct Header {
    header: ffi::Header,
}

impl Header {
    #[inline]
    pub fn new(header: ffi::Header) -> Self {
        Self {
            header
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
    pub fn get_buffer(&self, buffer: & UniquePtr<AtomicBuffer>) {
        ffi::get_buffer(&self.header, buffer);
    }
}


impl From <ffi::Header> for Header{
    fn from(header: ffi::Header) -> Self{
        Self::new(header)
    }
}
