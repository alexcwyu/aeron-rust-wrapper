extern crate aeron_rust_wrapper;

use aeron_rs::concurrent::atomic_buffer::AlignedBuffer;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rust_wrapper::aeron;

fn main() {
    let aligned_buffer = AlignedBuffer::with_capacity(256);
    let rust_atomic_buffer =  AtomicBuffer::from_aligned(&aligned_buffer);
    unsafe {
        let mut atomic_buffer = aeron::concurrent::atomic_buffer::ffi::new_instance(aligned_buffer.ptr, aligned_buffer.len as usize);

        let mut pinned_atomic_buffer = atomic_buffer.as_mut().unwrap();
        pinned_atomic_buffer.as_mut().put_i64(0, 1234567890);


        let mut atomic_buffer2 = aeron::concurrent::atomic_buffer::ffi::wrap_atomic_buffer(&atomic_buffer);
        let mut pinned_atomic_buffer2 = atomic_buffer.as_mut().unwrap();
        pinned_atomic_buffer2.as_mut().put_i32(8, 65535);

        println!("value1 {}", atomic_buffer.get_i64(0));
        println!("value1 {}", atomic_buffer.get_i32(8));

        println!("value2 {}", atomic_buffer2.get_i64(0));
        println!("value2 {}", atomic_buffer2.get_i32(8));

        println!("rust value {}", rust_atomic_buffer.get::<i64>(0));
        println!("rust value {}", rust_atomic_buffer.get::<i32>(8));
    }

    aeron::concurrent::logbuffer::header::ffi::say_hello();
    aeron::aeron::ffi::say_hello();
    aeron::context::ffi::say_hello();
    aeron::exclusive_publication::ffi::say_hello();
    aeron::image::ffi::say_hello();
    aeron::log_buffers::ffi::say_hello();
    aeron::publication::ffi::say_hello();
    aeron::subscription::ffi::say_hello();
    aeron::concurrent::logbuffer::buffer_claim::ffi::say_hello();

}