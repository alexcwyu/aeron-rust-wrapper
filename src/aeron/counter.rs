use std::ops::Deref;
use cxx::SharedPtr;

#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        // #[namespace = "aeron::concurrent"]
        // type CxxAtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;


        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Counter.h");
        include!("aeron-rust-wrapper/cxx_wrapper/Counter.cpp");

        #[rust_name = "CxxCounter"]
        type Counter;
        #[rust_name = "registration_id"]
        fn registrationId(self: &CxxCounter) -> i64;
        fn state(self: &CxxCounter) -> i32;
        #[rust_name = "is_closed"]
        fn isClosed(self: &CxxCounter) -> bool;

        #[namespace = "aeron::counter"]
        fn close(publication: &SharedPtr<CxxCounter>);

        #[namespace = "aeron::counter"]
        fn label(publication: &SharedPtr<CxxCounter>) -> String;
    }
    impl SharedPtr<CxxCounter> {}
}


unsafe impl Sync for ffi::CxxCounter {}
unsafe impl Send for ffi::CxxCounter {}

#[derive(Clone)]
pub struct Counter {
    counter: SharedPtr<ffi::CxxCounter>,
}

impl Counter {
    #[inline]
    pub fn new(counter: SharedPtr<ffi::CxxCounter>) -> Self {
        Self {
            counter
        }
    }

    #[inline]
    pub fn registration_id(&self) -> i64 {
        self.counter.registration_id()
    }

    #[inline]
    pub fn state(&self) -> i32 {
        self.counter.state()
    }

    #[inline]
    pub fn is_closed(&self) -> bool {
        self.counter.is_closed()
    }

    #[inline]
    pub fn close(&self) {
        ffi::close(&self.counter);
    }

    #[inline]
    pub fn label(&self) -> String {
        ffi::label(&self.counter)
    }

    pub fn get_ref(&self) -> &SharedPtr<ffi::CxxCounter> {
        &self.counter
    }
}

impl Deref for Counter {
    type Target = ffi::CxxCounter;

    fn deref(&self) -> &Self::Target {
        &self.counter.as_ref().unwrap()
    }
}

impl From <SharedPtr<ffi::CxxCounter>> for Counter{
    fn from(counter: SharedPtr<ffi::CxxCounter>) -> Self{
        Self::new(counter)
    }
}
