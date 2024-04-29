use std::ops::Deref;
use std::pin::Pin;

use cxx::SharedPtr;
use crate::aeron::concurrent::counters_reader::CountersReader;

use crate::aeron::concurrent::counters_reader::ffi::CxxCountersReader;
use crate::aeron::context::ffi::CxxContext;
use crate::aeron::counter::Counter;
use crate::aeron::exclusive_publication::ExclusivePublication;
use crate::aeron::publication::Publication;
use crate::aeron::subscription::Subscription;


#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {

        #[namespace = "aeron"]
        #[rust_name = "CxxCounter"]
        type Counter = crate::aeron::counter::ffi::CxxCounter;
        #[namespace = "aeron"]
        #[rust_name = "CxxImage"]
        type Image = crate::aeron::image::ffi::CxxImage;
        #[namespace = "aeron"]
        #[rust_name = "CxxContext"]
        type Context = crate::aeron::context::ffi::CxxContext;

        #[namespace = "aeron"]
        #[rust_name = "CxxSubscription"]
        type Subscription = crate::aeron::subscription::ffi::CxxSubscription;

        #[namespace = "aeron"]
        #[rust_name = "CxxPublication"]
        type Publication = crate::aeron::publication::ffi::CxxPublication;

        #[namespace = "aeron"]
        #[rust_name = "CxxExclusivePublication"]
        type ExclusivePublication = crate::aeron::exclusive_publication::ffi::CxxExclusivePublication;

        #[namespace = "aeron::concurrent"]
        #[rust_name = "CxxAtomicBuffer"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;

        #[namespace = "aeron::concurrent"]
        #[rust_name = "CxxCountersReader"]
        type CountersReader = crate::aeron::concurrent::counters_reader::ffi::CxxCountersReader;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Aeron.h");

        #[rust_name = "CxxAeron"]
        type Aeron;

        // c++ API:: const Context &context() const
        #[rust_name = "context"]
        fn context(self: &CxxAeron) -> Pin<&CxxContext>;

        // c++ API: bool usesAgentInvoker() const
        #[rust_name = "uses_agent_invoker"]
        fn usesAgentInvoker(self: &CxxAeron) -> bool;

        // c++ API: std::int64_t clientId() const
        #[rust_name = "client_id"]
        fn clientId(self: &CxxAeron) -> i64;



        include!("aeron-rust-wrapper/cxx_wrapper/Aeron.cpp");

        #[rust_name = "is_closed"]
        fn isClosed(aeron: &SharedPtr<CxxAeron>) ->bool;

        #[rust_name = "add_publication"]
        fn addPublication(aeron: &SharedPtr<CxxAeron>, channel: String, stream_id: i32) -> i64;


        #[rust_name = "add_exclusive_publication"]
        fn addExclusivePublication(aeron: &SharedPtr<CxxAeron>, channel: String, stream_id: i32) -> i64;

        #[rust_name = "find_publication"]
        fn findPublication(aeron: &SharedPtr<CxxAeron>, registration_id: i64) -> SharedPtr<CxxPublication>;

        #[rust_name = "find_exclusive_publication"]
        fn findExclusivePublication(aeron: &SharedPtr<CxxAeron>, registration_id: i64) -> SharedPtr<CxxExclusivePublication>;

        #[rust_name = "add_subscription"]
        fn addSubscription(aeron: &SharedPtr<CxxAeron>, channel: String, stream_id: i32) -> i64;

        #[rust_name = "find_subscription"]
        fn findSubscription(aeron: &SharedPtr<CxxAeron>, registration_id: i64) -> SharedPtr<CxxSubscription>;

        #[rust_name = "next_correlation_id"]
        fn nextCorrelationId(aeron: &SharedPtr<CxxAeron>) -> i64;


        // c++ API: std::int64_t addCounter(std::int32_t typeId, const std::uint8_t *keyBuffer, std::size_t keyLength, const std::string &label)
        #[rust_name = "add_counter"]
        unsafe fn addCounter(aeron: &SharedPtr<CxxAeron>, type_id: i32, key_buffer: * const u8, key_length: usize, label: String) -> i64;

        // c++ API: std::shared_ptr<Counter> findCounter(std::int64_t registrationId)
        #[rust_name = "find_counter"]
        fn findCounter(aeron: &SharedPtr<CxxAeron>, registration_id: i64) -> SharedPtr<CxxCounter>;

        // c++ API: void removeAvailableCounterHandler(std::int64_t registrationId)
        #[rust_name = "remove_available_counter_handler_by_id"]
        fn removeAvailableCounterHandler(aeron: &SharedPtr<CxxAeron>, registration_id: i64);


        // c++ API:
        // void removeUnavailableCounterHandler(std::int64_t registrationId)
        #[rust_name = "remove_unavailable_counter_handler_by_id"]
        fn removeUnavailableCounterHandler(aeron: &SharedPtr<CxxAeron>, registration_id: i64);

        // void removeCloseClientHandler(std::int64_t registrationId)
        #[rust_name = "remove_close_client_handler_by_id"]
        fn removeCloseClientHandler(aeron: &SharedPtr<CxxAeron>, registration_id: i64);

        // #[rust_name = "conductor_agent_invoker"]
        // fn conductorAgentInvoker(aeron: &SharedPtr<Aeron>) -> AgentInvoker<ClientConductor>;


        // c++ API: CountersReader &countersReader()
        #[rust_name = "counters_reader"]
        fn countersReader(aeron: &SharedPtr<CxxAeron>) ->SharedPtr<CxxCountersReader>;


        // c++ API: Context &context()
        // #[rust_name = "context_mut"]
        // fn context(aeron: &SharedPtr<Aeron>) -> Pin<&mut Context>;

        ///////////

        #[rust_name = "new_instance_with_context"]
        fn newInstance(context: Pin<&mut CxxContext>) -> SharedPtr<CxxAeron>;
        // #[rust_name = "new_instance"]
        // fn newInstance() -> UniquePtr<Aeron>;

        #[rust_name = "connect_with_context"]
        fn connect(context: Pin<&mut CxxContext>) -> SharedPtr<CxxAeron>;

        #[rust_name = "connect"]
        fn connect() -> SharedPtr<CxxAeron>;

        fn version() -> String;


        #[rust_name = "add_subscription_with_handlers"]
        fn addSubscription(aeron: &SharedPtr<CxxAeron>, channel: String, stream_id: i32) -> i64;

        // c++ API: std::int64_t addAvailableCounterHandler(const on_available_counter_t &handler)
        #[rust_name = "add_available_counter_handler"]
        fn addAvailableCounterHandler(aeron: &SharedPtr<CxxAeron>, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64;

        // c++ API: void removeAvailableCounterHandler(const on_available_counter_t &handler)
        #[rust_name = "remove_available_counter_handler"]
        fn removeAvailableCounterHandler(aeron: &SharedPtr<CxxAeron>, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ());

        // c++ API: std::int64_t addUnavailableCounterHandler(const on_unavailable_counter_t &handler)
        #[rust_name = "add_unavailable_counter_handler"]
        fn addUnavailableCounterHandler(aeron: &SharedPtr<CxxAeron>, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64;

        // c++ API: void removeUnavailableCounterHandler(const on_unavailable_counter_t &handler)
        #[rust_name = "remove_unavailable_counter_handler"]
        fn removeUnavailableCounterHandler(aeron: &SharedPtr<CxxAeron>, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ());


        // c++ API: std::int64_t addCloseClientHandler(const on_close_client_t &handler)
        #[rust_name = "add_close_client_handler"]
        fn addCloseClientHandler(aeron: &SharedPtr<CxxAeron>, handler: fn() -> ()) -> i64;

        // c++ API: void removeCloseClientHandler(const on_close_client_t &handler)
        #[rust_name = "remove_close_client_handler"]
        fn removeCloseClientHandler(aeron: &SharedPtr<CxxAeron>, handler: fn() -> ());

        // AgentInvoker<ClientConductor> &conductorAgentInvoker()

    }

    impl SharedPtr<CxxAeron> {}
}

unsafe impl Sync for ffi::CxxAeron {}
unsafe impl Send for ffi::CxxAeron {}

pub struct Aeron {
    aeron: SharedPtr<ffi::CxxAeron>,
}

impl Aeron {
    pub fn new(aeron: SharedPtr<ffi::CxxAeron>) -> Self {
        Self { aeron }
    }


    #[inline]
    pub fn new_instance_with_context(context: Pin<&mut CxxContext>) -> Self {
        Self { aeron : ffi::new_instance_with_context(context) }
    }

    #[inline]
    pub fn connect_with_context(context: Pin<&mut CxxContext>) -> Self {
        Self { aeron : ffi::connect_with_context(context) }
    }

    #[inline]
    pub fn connect() -> Self {
        Self { aeron : ffi::connect() }
    }

    #[inline]
    pub fn context(&self) -> Pin<&ffi::CxxContext> {
        self.aeron.context()
    }

    #[inline]
    pub fn uses_agent_invoker(&self) -> bool {
        self.aeron.uses_agent_invoker()
    }

    #[inline]
    pub fn client_id(&self) -> i64 {
        self.aeron.client_id()
    }

    #[inline]
    pub fn is_closed(&self) -> bool {
        ffi::is_closed(&self.aeron)
    }

    #[inline]
    pub fn add_publication(&self, channel: String, stream_id: i32) -> i64 {
        ffi::add_publication(&self.aeron, channel, stream_id)
    }

    #[inline]
    pub fn add_exclusive_publication(&self, channel: String, stream_id: i32) -> i64 {
        ffi::add_exclusive_publication(&self.aeron, channel, stream_id)
    }

    #[inline]
    pub fn find_publication(&self, registration_id: i64) -> Publication {
        Publication::from(ffi::find_publication(&self.aeron, registration_id).clone())
    }

    #[inline]
    pub fn find_exclusive_publication(&self, registration_id: i64) -> ExclusivePublication {
        ExclusivePublication::from(ffi::find_exclusive_publication(&self.aeron, registration_id).clone())
    }

    #[inline]
    pub fn add_subscription(&self, channel: String, stream_id: i32) -> i64 {
        ffi::add_subscription(&self.aeron, channel, stream_id)
    }

    #[inline]
    pub fn find_subscription(&self, registration_id: i64) -> Subscription {
        Subscription::from(ffi::find_subscription(&self.aeron, registration_id).clone())
    }

    #[inline]
    pub fn next_correlation_id(&self) -> i64 {
        ffi::next_correlation_id(&self.aeron)
    }

    #[inline]
    pub unsafe fn add_counter(&self, type_id: i32, key_buffer: * const u8, key_length: usize, label: String) -> i64 {
        ffi::add_counter(&self.aeron, type_id, key_buffer, key_length, label)
    }

    #[inline]
    pub fn find_counter(&self, registration_id: i64) -> Counter {
        Counter::from(ffi::find_counter(&self.aeron, registration_id).clone())
    }

    #[inline]
    pub fn remove_available_counter_handler_by_id(&self, registration_id: i64) {
        ffi::remove_available_counter_handler_by_id(&self.aeron, registration_id)
    }

    #[inline]
    pub fn remove_unavailable_counter_handler_by_id(&self, registration_id: i64) {
        ffi::remove_unavailable_counter_handler_by_id(&self.aeron, registration_id)
    }

    #[inline]
    pub fn remove_close_client_handler_by_id(&self, registration_id: i64) {
        ffi::remove_close_client_handler_by_id(&self.aeron, registration_id)
    }

    #[inline]
    pub fn counters_reader(&self) -> CountersReader {
        CountersReader::from(ffi::counters_reader(&self.aeron).clone())
    }

    #[inline]
    pub fn version() -> String {
        ffi::version()
    }

    #[inline]
    pub fn add_subscription_with_handlers(&self, channel: String, stream_id: i32) -> i64 {
        ffi::add_subscription_with_handlers(&self.aeron, channel, stream_id)
    }

    #[inline]
    pub fn add_available_counter_handler(&self, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64 {
        ffi::add_available_counter_handler(&self.aeron, handler)
    }

    #[inline]
    pub fn remove_available_counter_handler(&self, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) {
        ffi::remove_available_counter_handler(&self.aeron, handler)
    }

    #[inline]
    pub fn add_unavailable_counter_handler(&self, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64 {
        ffi::add_unavailable_counter_handler(&self.aeron, handler)
    }

    #[inline]
    pub fn remove_unavailable_counter_handler(&self, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) {
        ffi::remove_unavailable_counter_handler(&self.aeron, handler)
    }

    #[inline]
    pub fn add_close_client_handler(&self, handler: fn() -> ()) -> i64 {
        ffi::add_close_client_handler(&self.aeron, handler)
    }

    #[inline]
    pub fn remove_close_client_handler(&self, handler: fn() -> ()) {
        ffi::remove_close_client_handler(&self.aeron, handler)
    }

    pub fn get_ref(&self) -> &SharedPtr<ffi::CxxAeron> {
        &self.aeron
    }

}

impl Deref for Aeron {
    type Target = ffi::CxxAeron;

    fn deref(&self) -> &Self::Target {
        match self.aeron.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null ffi::CxxAeron"
            ),
        }
    }
}

impl From <SharedPtr<ffi::CxxAeron>> for Aeron {
    fn from(aeron: SharedPtr<ffi::CxxAeron>) -> Self{
        Self::new(aeron)
    }
}
