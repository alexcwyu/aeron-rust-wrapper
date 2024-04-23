
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {

        #[namespace = "aeron"]
        type Context = crate::aeron::context::ffi::Context;
        #[namespace = "aeron"]
        type Subscription = crate::aeron::subscription::ffi::Subscription;

        #[namespace = "aeron"]
        type Publication = crate::aeron::publication::ffi::Publication;
        #[namespace = "aeron"]
        type ExclusivePublication = crate::aeron::exclusive_publication::ffi::ExclusivePublication;


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

        // #[rust_name = "find_publication"]
        // fn findPublication(self: Pin<&mut Aeron>, registration_id: i64) -> SharedPtr<Publication>;

        // #[rust_name = "find_exclusive_publication"]
        // fn findExclusivePublication(self: Pin<&mut Aeron>, registration_id: i64) -> SharedPtr<ExclusivePublication>;

        #[rust_name = "add_subscription"]
        fn addSubscription(self: Pin<&mut Aeron>, channel: &CxxString, stream_id: i32) -> i64;

        // #[rust_name = "find_subscription"]
        // fn findSubscription(self: Pin<&mut Aeron>, registration_id: i64) -> SharedPtr<Subscription>;

        #[rust_name = "next_correlation_id"]
        fn nextCorrelationId(self: Pin<&mut Aeron>) -> i64;

        //std::int64_t addSubscription(const std::string &channel, std::int32_t streamId, const on_available_image_t &onAvailableImageHandler, const on_unavailable_image_t &onUnavailableImageHandler)
        // std::int64_t addCounter(std::int32_t typeId, const std::uint8_t *keyBuffer, std::size_t keyLength, const std::string &label)
        // std::shared_ptr<Counter> findCounter(std::int64_t registrationId)
        // std::int64_t addAvailableCounterHandler(const on_available_counter_t &handler)
        // void removeAvailableCounterHandler(const on_available_counter_t &handler)
        // void removeAvailableCounterHandler(std::int64_t registrationId)
        // std::int64_t addUnavailableCounterHandler(const on_unavailable_counter_t &handler)
        // void removeUnavailableCounterHandler(const on_unavailable_counter_t &handler)
        // void removeUnavailableCounterHandler(std::int64_t registrationId)
        // std::int64_t addCloseClientHandler(const on_close_client_t &handler)
        // void removeCloseClientHandler(const on_close_client_t &handler)
        // void removeCloseClientHandler(std::int64_t registrationId)
        // AgentInvoker<ClientConductor> &conductorAgentInvoker()
        // bool usesAgentInvoker() const
        // CountersReader &countersReader()
        // std::int64_t clientId() const
        // Context &context()
        // const Context &context() const


        include!("aeron-rust-wrapper/cxx_wrapper/Aeron.cpp");
        #[namespace = "aeron::aeron"]
        #[rust_name = "say_hello"]
        fn sayHello();
    }
}