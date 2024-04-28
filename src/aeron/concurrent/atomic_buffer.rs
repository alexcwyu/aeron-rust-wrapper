use std::pin::Pin;

use cxx::{CxxString, ExternType, UniquePtr};

#[cxx::bridge(namespace = "aeron::concurrent")]
pub mod ffi{
    // Shared structs with fields visible to both languages.

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/AtomicBuffer.h");


        type AtomicBuffer;

        fn capacity(&self) -> i32;


        #[rust_name = "set_capacity"]
        fn capacity(self: Pin<&mut AtomicBuffer>, length: usize);

        fn buffer(&self) -> *mut u8;


        #[rust_name = "put_i32"]
        fn putInt32(self: Pin<&mut AtomicBuffer>, offset: i32, value: i32);
        #[rust_name = "get_i32"]
        fn getInt32(&self, offset: i32) -> i32;

        #[rust_name = "put_i64"]
        fn putInt64(self: Pin<&mut AtomicBuffer>, offset: i32, value: i64);

        #[rust_name = "get_i64"]
        fn getInt64(&self, offset: i32) -> i64;

        #[rust_name = "put_i16"]
        fn putInt16(self: Pin<&mut AtomicBuffer>, offset: i32, value: i16);

        #[rust_name = "get_i16"]
        fn getInt16(&self, offset: i32) -> i16;

        #[rust_name = "put_u16"]
        fn putUInt16(self: Pin<&mut AtomicBuffer>, offset: i32, value: u16);

        #[rust_name = "get_u16"]
        fn getUInt16(&self, offset: i32) -> u16;

        #[rust_name = "put_u8"]
        fn putUInt8(self: Pin<&mut AtomicBuffer>, offset: i32, value: u8);

        #[rust_name = "get_u8"]
        fn getUInt8(&self, offset: i32) -> u8;


        #[rust_name = "put_i32_ordered"]
        fn putInt32Ordered(self: Pin<&mut AtomicBuffer>, offset: i32, value: i32);
        #[rust_name = "get_i32_volatile"]
        fn getInt32Volatile(&self, offset: i32) -> i32;

        #[rust_name = "put_i64_ordered"]
        fn putInt64Ordered(self: Pin<&mut AtomicBuffer>, offset: i32, value: i64);

        #[rust_name = "get_i64_volatile"]
        fn getInt64Volatile(&self, offset: i32) -> i64;

        #[rust_name = "put_i32_atomic"]
        fn putInt32Atomic(self: Pin<&mut AtomicBuffer>, offset: i32, value: i32);
        #[rust_name = "put_i64_atomic"]
        fn putInt64Atomic(self: Pin<&mut AtomicBuffer>, offset: i32, value: i64);

        #[rust_name = "add_i64_ordered"]
        fn addInt64Ordered(self: Pin<&mut AtomicBuffer>, offset: i32, delta: i64);

        #[rust_name = "compare_and_set_i64"]
        fn compareAndSetInt64(self: Pin<&mut AtomicBuffer>, offset: i32, expected_value: i64, update_value: i64) -> bool;

        #[rust_name = "get_and_set_i64"]
        fn getAndSetInt64(self: Pin<&mut AtomicBuffer>, offset: i32, value: i64) -> i64;

        #[rust_name = "get_and_set_i32"]
        fn getAndSetInt32(self: Pin<&mut AtomicBuffer>, offset: i32, value: i32) -> i32;

        #[rust_name = "get_and_add_i64"]
        fn getAndAddInt64(self: Pin<&mut AtomicBuffer>, offset: i32, delta: i64) -> i64;

        #[rust_name = "add_i32_ordered"]
        fn addInt32Ordered(self: Pin<&mut AtomicBuffer>, offset: i32, delta: i32);

        #[rust_name = "compare_and_set_i32"]
        fn compareAndSetInt32(self: Pin<&mut AtomicBuffer>, offset: i32, expected_value: i32, update_value: i32) -> bool;

        #[rust_name = "get_and_add_i32"]
        fn getAndAddInt32(self: Pin<&mut AtomicBuffer>, offset: i32, delta: i32) -> i32;

        #[rust_name = "put_bytes_from_buffer"]
        fn putBytes(self: Pin<&mut AtomicBuffer>, index: i32, srcBuffer: &AtomicBuffer, srcIndex: i32, length: i32);

        #[rust_name = "put_bytes"]
        unsafe fn putBytes(self: Pin<&mut AtomicBuffer>, index: i32, srcBuffer: *const u8, length: i32);

        #[rust_name = "set_memory"]
        fn setMemory(self: Pin<&mut AtomicBuffer>, offset: i32, length: usize, value: u8);

        #[rust_name = "get_bytes"]
        unsafe fn getBytes(&self, index: i32, dst: *mut u8, length: i32);


        #[rust_name = "get_string_length"]
        fn getStringLength(&self, offset: i32) -> i32;

        #[rust_name = "put_string"]
        fn putString(self: Pin<&mut AtomicBuffer>, offset: i32, value: &CxxString) -> i32;

        #[rust_name = "put_string_without_length"]
        fn putStringWithoutLength(self: Pin<&mut AtomicBuffer>, offset: i32, value: &CxxString) -> i32;

        include!("aeron-rust-wrapper/cxx_wrapper/concurrent/AtomicBuffer.cpp");

        #[namespace = "aeron::concurrent::atomic_buffer"]
        #[rust_name = "new_instance"]
        unsafe fn newInstance(buffer: *mut u8, length: usize) -> UniquePtr<AtomicBuffer>;

        #[namespace = "aeron::concurrent::atomic_buffer"]
        #[rust_name = "wrap_atomic_buffer"]
        fn wrapAtomicBuffer(buffer : &AtomicBuffer) -> UniquePtr<AtomicBuffer>;

        #[namespace = "aeron::concurrent::atomic_buffer"]
        #[rust_name = "get_string"]
        fn getString(buffer : &AtomicBuffer, offset: i32) -> String;

        #[namespace = "aeron::concurrent::atomic_buffer"]
        #[rust_name = "get_string_without_length"]
        fn getStringWithoutLength(buffer : &AtomicBuffer, offset: i32, length: usize) -> String;


    }

    impl SharedPtr<AtomicBuffer> {}
    impl UniquePtr<AtomicBuffer> {}


}

unsafe impl Sync for ffi::AtomicBuffer {}
unsafe impl Send for ffi::AtomicBuffer {}


pub struct AtomicBuffer {
    atomic_buffer: UniquePtr<ffi::AtomicBuffer>,
}

impl AtomicBuffer {
    #[inline]
    pub fn new(atomic_buffer: UniquePtr<ffi::AtomicBuffer>) -> Self {
        Self {
            atomic_buffer
        }
    }


    #[inline]
    pub unsafe fn wrap_bytes(buffer: *mut u8, length: usize) -> Self {
        Self {
            atomic_buffer: ffi::new_instance(buffer, length)
        }
    }

    #[inline]
    pub fn wrap_atomic_buffer(buffer: &ffi::AtomicBuffer) -> Self {
        Self {
            atomic_buffer: ffi::wrap_atomic_buffer(buffer)
        }
    }


    #[inline]
    pub fn set_capacity(&mut self, length: usize) {
        self.atomic_buffer.as_mut().unwrap().set_capacity(length);
    }

    #[inline]
    pub fn buffer(&self) -> *mut u8 {
        self.atomic_buffer.buffer()
    }

    #[inline]
    pub fn put_i32(&mut self, offset: i32, value: i32) {
        self.atomic_buffer.as_mut().unwrap().put_i32(offset, value);
    }

    #[inline]
    pub fn get_i32(&self, offset: i32) -> i32 {
        self.atomic_buffer.get_i32(offset)
    }

    #[inline]
    pub fn put_i64(&mut self, offset: i32, value: i64) {
        self.atomic_buffer.as_mut().unwrap().put_i64(offset, value);
    }

    #[inline]
    pub fn get_i64(&self, offset: i32) -> i64 {
        self.atomic_buffer.get_i64(offset)
    }

    #[inline]
    pub fn put_i16(&mut self, offset: i32, value: i16) {
        self.atomic_buffer.as_mut().unwrap().put_i16(offset, value);
    }

    #[inline]
    pub fn get_i16(&self, offset: i32) -> i16 {
        self.atomic_buffer.get_i16(offset)
    }

    #[inline]
    pub fn put_u16(&mut self, offset: i32, value: u16) {
        self.atomic_buffer.as_mut().unwrap().put_u16(offset, value);
    }

    #[inline]
    pub fn get_u16(&self, offset: i32) -> u16 {
        self.atomic_buffer.get_u16(offset)
    }

    #[inline]
    pub fn put_u8(&mut self, offset: i32, value: u8) {
        self.atomic_buffer.as_mut().unwrap().put_u8(offset, value);
    }

    #[inline]
    pub fn get_u8(&self, offset: i32) -> u8 {
        self.atomic_buffer.get_u8(offset)
    }


    #[inline]
    pub fn put_i32_ordered(&mut self, offset: i32, value: i32) {
        self.atomic_buffer.as_mut().unwrap().put_i32_ordered(offset, value);
    }

    #[inline]
    pub fn get_i32_volatile(&self, offset: i32) -> i32 {
        self.atomic_buffer.get_i32_volatile(offset)
    }

    #[inline]
    pub fn put_i64_ordered(&mut self, offset: i32, value: i64) {
        self.atomic_buffer.as_mut().unwrap().put_i64_ordered(offset, value);
    }

    #[inline]
    pub fn get_i64_volatile(&self, offset: i32) -> i64 {
        self.atomic_buffer.get_i64_volatile(offset)
    }

    #[inline]
    pub fn put_i32_atomic(&mut self, offset: i32, value: i32) {
        self.atomic_buffer.as_mut().unwrap().put_i32_atomic(offset, value);
    }

    #[inline]
    pub fn put_i64_atomic(&mut self, offset: i32, value: i64) {
        self.atomic_buffer.as_mut().unwrap().put_i64_atomic(offset, value);
    }

    #[inline]
    pub fn add_i64_ordered(&mut self, offset: i32, delta: i64) {
        self.atomic_buffer.as_mut().unwrap().add_i64_ordered(offset, delta);
    }

    #[inline]
    pub fn compare_and_set_i64(&mut self, offset: i32, expected_value: i64, update_value: i64) -> bool {
        self.atomic_buffer.as_mut().unwrap().compare_and_set_i64(offset, expected_value, update_value)
    }

    #[inline]
    pub fn get_and_set_i64(&mut self, offset: i32, value: i64) -> i64 {
        self.atomic_buffer.as_mut().unwrap().get_and_set_i64(offset, value)
    }

    #[inline]
    pub fn get_and_set_i32(&mut self, offset: i32, value: i32) -> i32 {
        self.atomic_buffer.as_mut().unwrap().get_and_set_i32(offset, value)
    }

    #[inline]
    pub fn get_and_add_i64(&mut self, offset: i32, delta: i64) -> i64 {
        self.atomic_buffer.as_mut().unwrap().get_and_add_i64(offset, delta)
    }

    #[inline]
    pub fn add_i32_ordered(&mut self, offset: i32, delta: i32) {
        self.atomic_buffer.as_mut().unwrap().add_i32_ordered(offset, delta);
    }

    #[inline]
    pub fn compare_and_set_i32(&mut self, offset: i32, expected_value: i32, update_value: i32) -> bool {
        self.atomic_buffer.as_mut().unwrap().compare_and_set_i32(offset, expected_value, update_value)
    }

    #[inline]
    pub fn get_and_add_i32(&mut self, offset: i32, delta: i32) -> i32 {
        self.atomic_buffer.as_mut().unwrap().get_and_add_i32(offset, delta)
    }

    #[inline]
    pub fn put_bytes_from_buffer(&mut self, index: i32, src_buffer: &ffi::AtomicBuffer, src_index: i32, length: i32) {
        self.atomic_buffer.as_mut().unwrap().put_bytes_from_buffer(index, src_buffer, src_index, length);
    }

    #[inline]
    pub unsafe fn put_bytes(&mut self, index: i32, src_buffer: *const u8, length: i32) {
        self.atomic_buffer.as_mut().unwrap().put_bytes(index, src_buffer, length);
    }

    #[inline]
    pub fn set_memory(&mut self, offset: i32, length: usize, value: u8) {
        self.atomic_buffer.as_mut().unwrap().set_memory(offset, length, value);
    }

    #[inline]
    pub unsafe fn get_bytes(&self, index: i32, dst: *mut u8, length: i32) {
        self.atomic_buffer.get_bytes(index, dst, length);
    }

    #[inline]
    pub fn get_string_length(&self, offset: i32) -> i32 {
        self.atomic_buffer.get_string_length(offset)
    }

    #[inline]
    pub fn put_string(&mut self, offset: i32, value: &CxxString) -> i32 {
        self.atomic_buffer.as_mut().unwrap().put_string(offset, value)
    }

    #[inline]
    pub fn put_string_without_length(&mut self, offset: i32, value: &CxxString) -> i32 {
        self.atomic_buffer.as_mut().unwrap().put_string_without_length(offset, value)
    }

    #[inline]
    pub fn get_string(&self, offset: i32) -> String {
        ffi::get_string(&self.atomic_buffer, offset)
    }

    #[inline]
    pub fn get_string_without_length(&self, offset: i32, length: usize) -> String {
        ffi::get_string_without_length(&self.atomic_buffer, offset, length)
    }

    pub fn get_ref(&self) -> &UniquePtr<ffi::AtomicBuffer> {
        &self.atomic_buffer
    }
    pub fn as_ref(&self) -> &ffi::AtomicBuffer {
        self.atomic_buffer.as_ref().unwrap()
    }
    pub fn as_mut(&mut self) -> Pin<& mut ffi::AtomicBuffer> {
        self.atomic_buffer.as_mut().unwrap()
    }
}


impl From <UniquePtr<ffi::AtomicBuffer>> for AtomicBuffer{
    fn from(atomic_buffer: UniquePtr<ffi::AtomicBuffer>) -> Self{
        Self::new(atomic_buffer)
    }
}
