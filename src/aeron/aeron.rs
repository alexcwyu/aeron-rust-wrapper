
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {

        #[namespace = "aeron"]
        type Counter = crate::aeron::counter::ffi::Counter;
        #[namespace = "aeron"]
        type Image = crate::aeron::image::ffi::Image;
        #[namespace = "aeron"]
        type Context = crate::aeron::context::ffi::Context;

        #[namespace = "aeron"]
        type Subscription = crate::aeron::subscription::ffi::Subscription;

        #[namespace = "aeron"]
        type Publication = crate::aeron::publication::ffi::Publication;

        #[namespace = "aeron"]
        type ExclusivePublication = crate::aeron::exclusive_publication::ffi::ExclusivePublication;

        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

        #[namespace = "aeron::concurrent"]
        type CountersReader = crate::aeron::concurrent::counters_reader::ffi::CountersReader;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Aeron.h");

        type Aeron;


        #[rust_name = "is_closed"]
        fn isClosed(self: Pin<&mut Aeron>) ->bool;
        //static std::shared_ptr<Aeron> connect(Context &context)
        //static std::shared_ptr<Aeron> connect()

        #[rust_name = "add_publication"]
        fn addPublication(self: Pin<&mut Aeron>, channel: &CxxString, stream_id: i32) -> i64;


        #[rust_name = "add_exclusive_publication"]
        fn addExclusivePublication(self: Pin<&mut Aeron>, channel: &CxxString, stream_id: i32) -> i64;

        #[rust_name = "find_publication"]
        fn findPublication(self: Pin<&mut Aeron>, registration_id: i64) -> SharedPtr<Publication>;

        #[rust_name = "find_exclusive_publication"]
        fn findExclusivePublication(self: Pin<&mut Aeron>, registration_id: i64) -> SharedPtr<ExclusivePublication>;

        #[rust_name = "add_subscription"]
        fn addSubscription(self: Pin<&mut Aeron>, channel: &CxxString, stream_id: i32) -> i64;

        #[rust_name = "find_subscription"]
        fn findSubscription(self: Pin<&mut Aeron>, registration_id: i64) -> SharedPtr<Subscription>;

        #[rust_name = "next_correlation_id"]
        fn nextCorrelationId(self: Pin<&mut Aeron>) -> i64;


        // c++ API: std::int64_t addCounter(std::int32_t typeId, const std::uint8_t *keyBuffer, std::size_t keyLength, const std::string &label)
        #[rust_name = "add_counter"]
        unsafe fn addCounter(self: Pin<&mut Aeron>, type_id: i32, key_buffer: * const u8, key_length: usize, label: &CxxString) -> i64;

        // c++ API: std::shared_ptr<Counter> findCounter(std::int64_t registrationId)
        #[rust_name = "find_counter"]
        fn findCounter(self: Pin<&mut Aeron>, registration_id: i64) -> SharedPtr<Counter>;

        // c++ API: void removeAvailableCounterHandler(std::int64_t registrationId)
        #[rust_name = "remove_available_counter_handler_by_id"]
        fn removeAvailableCounterHandler(self: Pin<&mut Aeron>, registration_id: i64);


        // c++ API:
        // void removeUnavailableCounterHandler(std::int64_t registrationId)
        #[rust_name = "remove_unavailable_counter_handler_by_id"]
        fn removeUnavailableCounterHandler(self: Pin<&mut Aeron>, registration_id: i64);

        // void removeCloseClientHandler(std::int64_t registrationId)
        #[rust_name = "remove_close_client_handler_by_id"]
        fn removeCloseClientHandler(self: Pin<&mut Aeron>, registration_id: i64);

        // #[rust_name = "conductor_agent_invoker"]
        // fn conductorAgentInvoker(self: Pin<&mut Aeron>) -> AgentInvoker<ClientConductor>;

        // c++ API: bool usesAgentInvoker() const
        #[rust_name = "uses_agent_invoker"]
        fn usesAgentInvoker(self: &Aeron) -> bool;

        // c++ API: CountersReader &countersReader()
        #[rust_name = "counters_reader"]
        fn countersReader(self: Pin<&mut Aeron>) -> Pin<&mut CountersReader>;

        // c++ API: std::int64_t clientId() const
        #[rust_name = "client_id"]
        fn clientId(self: &Aeron) -> i64;

        // c++ API: Context &context()
        #[rust_name = "context_mut"]
        fn context(self: Pin<&mut Aeron>) -> Pin<&mut Context>;

        // c++ API:: const Context &context() const
        #[rust_name = "context"]
        fn context(self: & Aeron) -> Pin<&Context>;

        include!("aeron-rust-wrapper/cxx_wrapper/Aeron.cpp");

        #[rust_name = "connect_with_context"]
        fn connect(context: Pin<&mut Context>) -> SharedPtr<Aeron>;
        #[rust_name = "connect"]
        fn connect() -> SharedPtr<Aeron>;

        fn version() -> String;


        #[rust_name = "add_subscription_with_handlers"]
        fn addSubscription(aeron: Pin<&mut Aeron>, channel: &CxxString, stream_id: i32, on_available_image_t: fn(image: Pin<&mut Image>) -> (), on_unavailable_image_t: fn(image: Pin<&mut Image>) -> ()) -> i64;

        // c++ API: std::int64_t addAvailableCounterHandler(const on_available_counter_t &handler)
        #[rust_name = "add_available_counter_handler"]
        fn addAvailableCounterHandler(aeron: Pin<&mut Aeron>, handler: fn(counters_reader: Pin<&mut CountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64;

        // c++ API: void removeAvailableCounterHandler(const on_available_counter_t &handler)
        #[rust_name = "remove_available_counter_handler"]
        fn removeAvailableCounterHandler(aeron: Pin<&mut Aeron>, handler: fn(counters_reader: Pin<&mut CountersReader>, registration_id: i64, counter_id: i32) -> ());

        // c++ API: std::int64_t addUnavailableCounterHandler(const on_unavailable_counter_t &handler)
        #[rust_name = "add_unavailable_counter_handler"]
        fn addUnavailableCounterHandler(aeron: Pin<&mut Aeron>, handler: fn(counters_reader: Pin<&mut CountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64;

        // c++ API: void removeUnavailableCounterHandler(const on_unavailable_counter_t &handler)
        #[rust_name = "remove_unavailable_counter_handler"]
        fn removeUnavailableCounterHandler(aeron: Pin<&mut Aeron>, handler: fn(counters_reader: Pin<&mut CountersReader>, registration_id: i64, counter_id: i32) -> ());


        // c++ API: std::int64_t addCloseClientHandler(const on_close_client_t &handler)
        #[rust_name = "add_close_client_handler"]
        fn addCloseClientHandler(aeron: Pin<&mut Aeron>, handler: fn() -> ()) -> i64;

        // c++ API: void removeCloseClientHandler(const on_close_client_t &handler)
        #[rust_name = "remove_close_client_handler"]
        fn removeCloseClientHandler(aeron: Pin<&mut Aeron>, handler: fn() -> ());

        // AgentInvoker<ClientConductor> &conductorAgentInvoker()


        #[rust_name = "say_hello"]
        fn sayHello();
    }

    impl SharedPtr<Aeron> {}
}