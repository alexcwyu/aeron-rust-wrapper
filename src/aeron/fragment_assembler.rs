
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        #[rust_name = "CxxAtomicBuffer"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;

        #[namespace = "aeron::concurrent::logbuffer"]
        #[rust_name = "CxxHeader"]
        type Header = crate::aeron::concurrent::logbuffer::header::ffi::CxxHeader;
        #[namespace = "aeron"]
        #[rust_name = "CxxControlledPollAction"]
        type ControlledPollAction = crate::aeron::image::ffi::CxxControlledPollAction;

        #[namespace = "aeron"]
        #[rust_name = "CxxImage"]
        type Image = crate::aeron::image::ffi::CxxImage;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/FragmentAssembler.h");
        #[rust_name = "CxxFragmentAssembler"]
        type FragmentAssembler;


        include!("aeron-rust-wrapper/cxx_wrapper/FragmentAssembler.cpp");


        //fn handler(self: &CxxFragmentAssembler) -> fn(&CxxAtomicBuffer, i32, i32, &CxxHeader) -> ();

        #[namespace = "aeron::fragment_assembler"]
        fn deleteSessionBuffer(fragment_assembler: &UniquePtr<CxxFragmentAssembler>, sessionId: i32);


        #[namespace = "aeron::fragment_assembler"]
        fn newInstance(handler: fn(&CxxAtomicBuffer, i32, i32, &CxxHeader) -> (), initialBufferLength: usize) -> UniquePtr<CxxFragmentAssembler>;
    }

}