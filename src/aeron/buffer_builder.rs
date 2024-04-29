use std::ops::Deref;
use std::pin::Pin;
use cxx::{SharedPtr, UniquePtr};
use crate::aeron::buffer_builder::ffi::CxxBufferBuilder;
use crate::aeron::concurrent::atomic_buffer::AtomicBuffer;
use crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;
use crate::aeron::concurrent::logbuffer::header::ffi::CxxHeader;

#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        #[rust_name = "CxxAtomicBuffer"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;
        #[namespace = "aeron::concurrent::logbuffer"]
        #[rust_name = "CxxHeader"]
        type Header = crate::aeron::concurrent::logbuffer::header::ffi::CxxHeader;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/BufferBuilder.h");
        include!("aeron-rust-wrapper/cxx_wrapper/BufferBuilder.cpp");

        #[rust_name = "CxxBufferBuilder"]
        type BufferBuilder;

        fn buffer(self: &CxxBufferBuilder) -> *mut u8;
        fn limit(self: &CxxBufferBuilder) -> u32;
        fn capacity(self: &CxxBufferBuilder) -> u32;
        #[rust_name = "next_term_offset"]
        fn nextTermOffset(self: &CxxBufferBuilder) -> i32;

        #[namespace = "aeron::buffer_builder"]
        #[rust_name = "new_instance"]
        fn newInstance(initialCapacity: u32) -> UniquePtr<CxxBufferBuilder>;
        #[namespace = "aeron::buffer_builder"]
        #[rust_name = "set_limit"]
        fn limit(builder: &UniquePtr<CxxBufferBuilder>, limit: u32);
        #[namespace = "aeron::buffer_builder"]
        #[rust_name = "set_next_term_offset"]
        fn nextTermOffset(nextTermOffset: &UniquePtr<CxxBufferBuilder>, offset: i32);
        #[namespace = "aeron::buffer_builder"]
        fn reset(nextTermOffset: &UniquePtr<CxxBufferBuilder>);

        #[namespace = "aeron::buffer_builder"]
        fn compact(nextTermOffset: &UniquePtr<CxxBufferBuilder>);

        #[namespace = "aeron::buffer_builder"]
        fn append(nextTermOffset: &UniquePtr<CxxBufferBuilder>, src: &CxxAtomicBuffer, srcOffset: i32, length: i32);

        #[namespace = "aeron::buffer_builder"]
        #[rust_name = "capture_header"]
        fn captureHeader(nextTermOffset: &UniquePtr<CxxBufferBuilder>, header: &CxxHeader);

        #[namespace = "aeron::buffer_builder"]
        #[rust_name = "complete_header"]
        fn completeHeader(nextTermOffset: &UniquePtr<CxxBufferBuilder>, header: &CxxHeader)-> SharedPtr<CxxHeader>;

        #[namespace = "aeron::buffer_builder"]
        #[rust_name = "header"]
        fn completeHeader(nextTermOffset: &UniquePtr<CxxBufferBuilder>) -> SharedPtr<CxxHeader>;


    }
}


unsafe impl Sync for ffi::CxxBufferBuilder {}
unsafe impl Send for ffi::CxxBufferBuilder {}


pub struct BufferBuilder {
    buffer_builder: UniquePtr<ffi::CxxBufferBuilder>,
}

impl BufferBuilder {
    pub fn new(buffer_builder: UniquePtr<ffi::CxxBufferBuilder>) -> Self {
        Self {
            buffer_builder,
        }
    }

    pub fn new_instance(initial_capacity: u32) -> Self {
        Self {
            buffer_builder: ffi::new_instance(initial_capacity),
        }
    }

    #[inline]
    pub fn buffer(&self) -> *mut u8 {
        self.buffer_builder.buffer()
    }

    #[inline]
    pub fn limit(&self) -> u32 {
        self.buffer_builder.limit()
    }

    #[inline]
    pub fn capacity(&self) -> u32 {
        self.buffer_builder.capacity()
    }

    #[inline]
    pub fn next_term_offset(&self) -> i32 {
        self.buffer_builder.next_term_offset()
    }

    #[inline]
    pub fn set_limit(&self, limit: u32) {
        ffi::set_limit(&self.buffer_builder, limit);
    }

    #[inline]
    pub fn set_next_term_offset(&self, offset: i32) {
        ffi::set_next_term_offset(&self.buffer_builder, offset);
    }

    #[inline]
    pub fn reset(&self) {
        ffi::reset(&self.buffer_builder);
    }

    #[inline]
    pub fn compact(&self) {
        ffi::compact(&self.buffer_builder);
    }

    #[inline]
    pub fn append(&self, src: &CxxAtomicBuffer, src_offset: i32, length: i32) {
        ffi::append(&self.buffer_builder, src, src_offset, length);
    }

    #[inline]
    pub fn capture_header(&self, header: &CxxHeader) {
        ffi::capture_header(&self.buffer_builder, header);
    }

    #[inline]
    pub fn complete_header(&self, header: &CxxHeader) -> SharedPtr<CxxHeader> {
        ffi::complete_header(&self.buffer_builder, header)
    }

    #[inline]
    pub fn header(&self) -> SharedPtr<CxxHeader> {
        ffi::header(&self.buffer_builder)
    }


    #[inline]
    pub fn get_ref(&self) -> &UniquePtr<CxxBufferBuilder> {
        &self.buffer_builder
    }

    #[inline]
    pub fn as_mut(&mut self) -> Pin<&mut CxxBufferBuilder> {
        self.buffer_builder.as_mut().unwrap()
    }
}


impl Deref for BufferBuilder {
    type Target = CxxBufferBuilder;

    fn deref(&self) -> &Self::Target {
        match self.buffer_builder.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null ffi::CxxBufferBuilder"
            ),
        }
    }
}

impl From <UniquePtr<CxxBufferBuilder>> for BufferBuilder{
    fn from(buffer_builder: UniquePtr<CxxBufferBuilder>) -> Self{
        Self::new(buffer_builder)
    }
}