use std::pin::Pin;

use cxx::UniquePtr;

use crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

#[cxx::bridge(namespace = "aeron::concurrent::logbuffer")]
pub mod ffi {
    // Shared structs with fields visible to both languages.

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/logbuffer/BufferClaim.h");

        type BufferClaim;

        fn wrap(self: Pin<&mut BufferClaim>, buffer : Pin<&mut AtomicBuffer>, offset: i32, length: i32);

        fn offset(self: &BufferClaim) -> i32;
        fn length(self: &BufferClaim) -> i32;
        fn flags(self: &BufferClaim) -> u8;
        #[rust_name = "header_type"]
        fn headerType(self: &BufferClaim) -> u16;
        #[rust_name = "reserved_value"]
        fn reservedValue(self: &BufferClaim) -> i64;
        fn commit(self: Pin<&mut BufferClaim>);
        fn abort(self: Pin<&mut BufferClaim>);

        //this_t& flags(const std::uint8_t flags)

        //this_t &reservedValue(const std::int64_t value)

        include!("aeron-rust-wrapper/cxx_wrapper/concurrent/logbuffer/BufferClaim.cpp");
        #[namespace = "aeron::concurrent::logbuffer::buffer_claim"]
        #[rust_name = "new_instance"]
        fn newInstance() -> UniquePtr<BufferClaim>;

        #[namespace = "aeron::concurrent::logbuffer::buffer_claim"]
        #[rust_name = "wrap_raw_buffer"]
        unsafe fn wrapRawBuffer(buffer_claim: Pin<&mut BufferClaim>, buffer: *mut u8, length: i32);

        #[namespace = "aeron::concurrent::logbuffer::buffer_claim"]
        #[rust_name = "get_buffer"]
        fn getBuffer(buffer_claim: Pin<&mut BufferClaim>, buffer: & UniquePtr<AtomicBuffer>);

    }


    impl SharedPtr<BufferClaim> {}
    impl UniquePtr<BufferClaim> {}
}


unsafe impl Sync for ffi::BufferClaim {}
unsafe impl Send for ffi::BufferClaim {}

pub struct BufferClaim {
    buffer_claim: UniquePtr<ffi::BufferClaim>,
}

impl BufferClaim {
    #[inline]
    pub fn new(buffer_claim: UniquePtr<ffi::BufferClaim>) -> Self {
        Self {
            buffer_claim
        }
    }

    #[inline]
    pub fn new_instance() -> Self {
        Self {
            buffer_claim : ffi::new_instance()
        }
    }

    #[inline]
    pub unsafe fn wrap_raw_buffer(&mut self, buffer: *mut u8, length: i32){
        ffi::wrap_raw_buffer( self.buffer_claim.as_mut().unwrap(),
            buffer,
            length);
    }

    #[inline]
    pub fn wrap(&mut self, buffer: Pin<&mut ffi::AtomicBuffer>, offset: i32, length: i32) {
        self.buffer_claim.as_mut().unwrap().wrap(buffer, offset, length);
    }

    #[inline]
    pub fn offset(&self) -> i32 {
        self.buffer_claim.offset()
    }

    #[inline]
    pub fn length(&self) -> i32 {
        self.buffer_claim.length()
    }

    #[inline]
    pub fn flags(&self) -> u8 {
        self.buffer_claim.flags()
    }

    #[inline]
    pub fn header_type(&self) -> u16 {
        self.buffer_claim.header_type()
    }

    #[inline]
    pub fn reserved_value(&self) -> i64 {
        self.buffer_claim.reserved_value()
    }

    #[inline]
    pub fn commit(&mut self) {
        self.buffer_claim.as_mut().unwrap().commit();
    }

    #[inline]
    pub fn abort(&mut self) {
        self.buffer_claim.as_mut().unwrap().abort();
    }

    #[inline]
    pub fn get_buffer(&mut self, buffer: & UniquePtr<AtomicBuffer>) {
        ffi::get_buffer(self.buffer_claim.as_mut().unwrap(), buffer);
    }
    #[inline]
    pub fn get_ref(&self) -> &UniquePtr<ffi::BufferClaim> {
        &self.buffer_claim
    }
    #[inline]
    pub fn as_ref(&self) -> &ffi::BufferClaim {
        self.buffer_claim.as_ref().unwrap()
    }
    #[inline]
    pub fn as_mut(&mut self) -> Pin<& mut ffi::BufferClaim> {
        self.buffer_claim.as_mut().unwrap()
    }
}


impl From <UniquePtr<ffi::BufferClaim>> for BufferClaim {
    fn from(buffer_claim: UniquePtr<ffi::BufferClaim>) -> Self{
        Self::new(buffer_claim)
    }
}