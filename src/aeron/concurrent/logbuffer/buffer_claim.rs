use std::ops::{Deref, DerefMut};
use std::pin::Pin;

use cxx::UniquePtr;

use crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;

#[cxx::bridge(namespace = "aeron::concurrent::logbuffer")]
pub mod ffi {
    // Shared structs with fields visible to both languages.

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type CxxAtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/logbuffer/BufferClaim.h");

        include!("aeron-rust-wrapper/cxx_wrapper/concurrent/logbuffer/BufferClaim.cpp");
        
        //this_t& flags(const std::uint8_t flags)
        //this_t &reservedValue(const std::int64_t value)

        #[rust_name = "CxxBufferClaim"]
        type BufferClaim;

        fn wrap(self: Pin<&mut CxxBufferClaim>, buffer : Pin<&mut CxxAtomicBuffer>, offset: i32, length: i32);

        fn offset(self: &CxxBufferClaim) -> i32;
        fn length(self: &CxxBufferClaim) -> i32;
        fn flags(self: &CxxBufferClaim) -> u8;
        #[rust_name = "header_type"]
        fn headerType(self: &CxxBufferClaim) -> u16;
        #[rust_name = "reserved_value"]
        fn reservedValue(self: &CxxBufferClaim) -> i64;
        fn commit(self: Pin<&mut CxxBufferClaim>);
        fn abort(self: Pin<&mut CxxBufferClaim>);
        #[namespace = "aeron::concurrent::logbuffer::buffer_claim"]
        #[rust_name = "new_instance"]
        fn newInstance() -> UniquePtr<CxxBufferClaim>;

        #[namespace = "aeron::concurrent::logbuffer::buffer_claim"]
        #[rust_name = "wrap_raw_buffer"]
        unsafe fn wrapRawBuffer(buffer_claim: Pin<&mut CxxBufferClaim>, buffer: *mut u8, length: i32);

        #[namespace = "aeron::concurrent::logbuffer::buffer_claim"]
        #[rust_name = "get_buffer"]
        fn getBuffer(buffer_claim: Pin<&mut CxxBufferClaim>, buffer: & UniquePtr<CxxAtomicBuffer>);

    }


    impl SharedPtr<CxxBufferClaim> {}
    impl UniquePtr<CxxBufferClaim> {}
}


unsafe impl Sync for ffi::CxxBufferClaim {}
unsafe impl Send for ffi::CxxBufferClaim {}

pub struct BufferClaim {
    buffer_claim: UniquePtr<ffi::CxxBufferClaim>,
}

impl BufferClaim {
    #[inline]
    pub fn new(buffer_claim: UniquePtr<ffi::CxxBufferClaim>) -> Self {
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
    pub fn wrap(&mut self, buffer: Pin<&mut CxxAtomicBuffer>, offset: i32, length: i32) {
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
    pub fn get_buffer(&mut self, buffer: & UniquePtr<CxxAtomicBuffer>) {
        ffi::get_buffer(self.buffer_claim.as_mut().unwrap(), buffer);
    }
    #[inline]
    pub fn get_ref(&self) -> &UniquePtr<ffi::CxxBufferClaim> {
        &self.buffer_claim
    }


}

impl Deref for BufferClaim {
    type Target = ffi::CxxBufferClaim;

    fn deref(&self) -> &Self::Target {
        &self.buffer_claim.as_ref().unwrap()
    }
}

impl DerefMut for BufferClaim {
    fn deref_mut(&mut self) -> Pin<&mut Self::Target> {
        self.buffer_claim.as_mut().unwrap()
    }
}

impl From <UniquePtr<ffi::CxxBufferClaim>> for BufferClaim {
    fn from(buffer_claim: UniquePtr<ffi::CxxBufferClaim>) -> Self{
        Self::new(buffer_claim)
    }
}