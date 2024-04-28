extern crate aeron_rust_wrapper;

use aeron_rs::concurrent::atomic_buffer::AlignedBuffer;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;

fn main() {
    let aligned_buffer = AlignedBuffer::with_capacity(256);
    let rust_atomic_buffer =  AtomicBuffer::from_aligned(&aligned_buffer);
    unsafe {
        let mut atomic_buffer = AtomicBuffer::new_instance(aligned_buffer.ptr, aligned_buffer.len as usize);

        let mut pinned_atomic_buffer = atomic_buffer.as_mut().unwrap();
        pinned_atomic_buffer.as_mut().put_i64(0, 1234567890);


        let mut atomic_buffer2 = AtomicBuffer::wrap_atomic_buffer(&atomic_buffer);
        let mut pinned_atomic_buffer2 = atomic_buffer.as_mut().unwrap();
        pinned_atomic_buffer2.as_mut().put_i32(8, 65535);

        println!("value1 {}", atomic_buffer.get_i64(0));
        println!("value1 {}", atomic_buffer.get_i32(8));

        println!("value2 {}", atomic_buffer2.get_i64(0));
        println!("value2 {}", atomic_buffer2.get_i32(8));

        println!("rust value {}", rust_atomic_buffer.get::<i64>(0));
        println!("rust value {}", rust_atomic_buffer.get::<i32>(8));
    }



    //println!("default_aeron_path{}", aeron::context::ffi::default_aeron_path());
}