use cxx::SharedPtr;

#[cxx::bridge(namespace = "aeron")]
pub(crate) mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;


        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Counter.h");
        type Counter;
        #[rust_name = "registration_id"]
        fn registrationId(self: &Counter) -> i64;
        fn state(self: &Counter) -> i32;
        #[rust_name = "is_closed"]
        fn isClosed(self: &Counter) -> bool;


        include!("aeron-rust-wrapper/cxx_wrapper/Counter.cpp");

        #[namespace = "aeron::counter"]
        fn close(publication: &SharedPtr<Counter>);

        #[namespace = "aeron::counter"]
        fn label(publication: &SharedPtr<Counter>) -> String;


    }

    impl SharedPtr<Counter> {}
}


unsafe impl Sync for ffi::Counter {}
unsafe impl Send for ffi::Counter {}

#[derive(Clone)]
pub struct Counter {
    counter: SharedPtr<ffi::Counter>,
}

impl Counter {
    #[inline]
    pub fn new(counter: SharedPtr<ffi::Counter>) -> Self {
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

    pub fn get_ref(&self) -> &SharedPtr<ffi::Counter> {
        &self.counter
    }
}


impl From <SharedPtr<ffi::Counter>> for Counter{
    fn from(counter: SharedPtr<ffi::Counter>) -> Self{
        Self::new(counter)
    }
}
