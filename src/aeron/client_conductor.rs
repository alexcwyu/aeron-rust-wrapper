#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {

        #[namespace = "aeron"]
        type Counter = crate::aeron::counter::ffi::Counter;
        #[namespace = "aeron"]
        type Image = crate::aeron::image::ffi::Image;
        #[namespace = "aeron"]
        type Subscription = crate::aeron::subscription::ffi::Subscription;

        #[namespace = "aeron"]
        type Publication = crate::aeron::publication::ffi::Publication;

        #[namespace = "aeron"]
        type ExclusivePublication = crate::aeron::exclusive_publication::ffi::ExclusivePublication;

        #[namespace = "aeron::concurrent"]
        type CountersReader = crate::aeron::concurrent::counters_reader::ffi::CountersReader;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/ClientConductor.h");

        type ClientConductor;


        #[rust_name = "on_start"]
        fn onStart(self: Pin<&mut ClientConductor>);

        #[rust_name = "do_work"]
        fn doWork(self: Pin<& mut ClientConductor>) -> i32;
        #[rust_name = "on_close"]
        fn onClose(self: Pin<&mut ClientConductor>);

        #[rust_name = "add_publication"]
        fn addPublication(self: Pin<&mut ClientConductor>, channel: &CxxString, stream_id: i32) -> i64;


        #[rust_name = "find_publication"]
        fn findPublication(self: Pin<&mut ClientConductor>, registration_id: i64) -> SharedPtr<Publication>;


        #[rust_name = "release_publication"]
        fn releasePublication(self: Pin<&mut ClientConductor>, registration_id: i64);

        #[rust_name = "add_exclusive_publication"]
        fn addExclusivePublication(self: Pin<&mut ClientConductor>, channel: &CxxString, stream_id: i32) -> i64;

        #[rust_name = "find_exclusive_publication"]
        fn findExclusivePublication(self: Pin<&mut ClientConductor>, registration_id: i64) -> SharedPtr<ExclusivePublication>;

        #[rust_name = "release_exclusive_publication"]
        fn releaseExclusivePublication(self: Pin<&mut ClientConductor>, registration_id: i64);

        #[rust_name = "find_subscription"]
        fn findSubscription(self: Pin<&mut ClientConductor>, registration_id: i64) -> SharedPtr<Subscription>;

        // #[rust_name = "release_subscription"]
        // unsafe fn releaseSubscription(self: Pin<&mut ClientConductor>, registration_id: i64, image_array: * SharedPtr<Image>, length: usize);

        #[rust_name = "add_counter"]
        unsafe fn addCounter(self: Pin<&mut ClientConductor>, type_id: i32, key_buffer: *const u8, key_length: usize, label: &CxxString) -> i64;

        #[rust_name = "find_counter"]
        fn findCounter(self: Pin<&mut ClientConductor>, registration_id: i64) -> SharedPtr<Counter>;

        #[rust_name = "release_counter"]
        fn releaseCounter(self: Pin<&mut ClientConductor>, registration_id: i64);

        #[rust_name = "find_destination_response"]
        fn findDestinationResponse(self: Pin<&mut ClientConductor>, correlation_id: i64) -> bool;

        #[rust_name = "on_new_publication"]
        fn onNewPublication(
            self: Pin<&mut ClientConductor>,
            registrationId: i64,
            originalRegistrationId: i64,
            streamId: i32,
            sessionId: i32,
            publicationLimitCounterId: i32,
            channelStatusIndicatorId: i32,
            logFilename: &CxxString,
        );

        #[rust_name = "on_new_exclusive_publication"]
        fn onNewExclusivePublication(
            self: Pin<&mut ClientConductor>,
            registrationId: i64,
            originalRegistrationId: i64,
            streamId: i32,
            sessionId: i32,
            publicationLimitCounterId: i32,
            channelStatusIndicatorId: i32,
            logFilename: &CxxString,
        );

        #[rust_name = "on_subscription_ready"]
        fn onSubscriptionReady(self: Pin<&mut ClientConductor>, registrationId: i64, channelStatusId: i32);

        #[rust_name = "on_operation_success"]
        fn onOperationSuccess(self: Pin<&mut ClientConductor>, correlationId: i64);

        #[rust_name = "on_channel_endpoint_error_response"]
        fn onChannelEndpointErrorResponse(self: Pin<&mut ClientConductor>, channelStatusId: i32, errorMessage: &CxxString);

        #[rust_name = "on_error_response"]
        fn onErrorResponse(
            self: Pin<&mut ClientConductor>,
            offendingCommandCorrelationId: i64,
            errorCode: i32,
            errorMessage: &CxxString,
        );
        #[rust_name = "on_available_image"]
        fn onAvailableImage(
            self: Pin<&mut ClientConductor>,
            correlationId: i64,
            sessionId: i32,
            subscriberPositionId: i32,
            subscriptionRegistrationId: i64,
            logFilename: &CxxString,
            sourceIdentity: &CxxString,
        );

        #[rust_name = "on_unavailable_image"]
        fn onUnavailableImage(self: Pin<&mut ClientConductor>, correlationId: i64, subscriptionRegistrationId: i64);


        #[rust_name = "on_available_counter"]
        fn onAvailableCounter(self: Pin<&mut ClientConductor>, registrationId: i64, counterId: i32);


        #[rust_name = "on_unavailable_counter"]
        fn onUnavailableCounter(self: Pin<&mut ClientConductor>, registrationId: i64, counterId: i32);


        #[rust_name = "on_client_timeout"]
        fn onClientTimeout(self: Pin<&mut ClientConductor>, clientId: i64);

        // #[rust_name = "close_all_resources"]
        // fn closeAllResources(self: Pin<&mut ClientConductor>, nowMs: i128);

        #[rust_name = "add_destination"]
        fn addDestination(self: Pin<&mut ClientConductor>, publicationRegistrationId: i64, endpointChannel: &CxxString) -> i64;

        #[rust_name = "remove_destination"]
        fn removeDestination(self: Pin<&mut ClientConductor>, publicationRegistrationId: i64, endpointChannel: &CxxString) -> i64;

        #[rust_name = "add_rcv_destination"]
        fn addRcvDestination(self: Pin<&mut ClientConductor>, subscriptionRegistrationId: i64, endpointChannel: &CxxString) -> i64;

        #[rust_name = "remove_rcv_destination"]
        fn removeRcvDestination(self: Pin<&mut ClientConductor>, subscriptionRegistrationId: i64, endpointChannel: &CxxString) -> i64;

        #[rust_name = "remove_available_counter_handler_by_id"]
        fn removeAvailableCounterHandler(self: Pin<&mut ClientConductor>, registrationId: i64);


        #[rust_name = "remove_unavailable_counter_handler_by_id"]
        fn removeUnavailableCounterHandler(self: Pin<&mut ClientConductor>, registrationId: i64);

        #[rust_name = "remove_close_client_handler_by_id"]
        fn removeCloseClientHandler(self: Pin<&mut ClientConductor>, registrationId: i64);

        #[rust_name = "counters_reader"]
        fn countersReader(self: Pin<&mut ClientConductor>) -> Pin<&mut CountersReader>;

        #[rust_name = "channel_status"]
        fn channelStatus(self: &ClientConductor, counter_id: i32) -> i64;

        #[rust_name = "is_closed"]
        fn isClosed(self: &ClientConductor) -> bool;


        #[rust_name = "ensure_open"]
        fn ensureOpen(self: &ClientConductor);


        include!("aeron-rust-wrapper/cxx_wrapper/ClientConductor.cpp");


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_subscription"]
        fn addSubscription(client_conductor: Pin<&mut ClientConductor>, channel: &CxxString, stream_id: i32, on_available_image_t: fn(image: Pin<&mut Image>) -> (), on_unavailable_image_t: fn(image: Pin<&mut Image>) -> ()) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_available_counter_handler"]
        fn addAvailableCounterHandler(client_conductor: Pin<&mut ClientConductor>, handler: fn(counters_reader: Pin<&mut CountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_available_counter_handler"]
        fn removeAvailableCounterHandler(client_conductor: Pin<&mut ClientConductor>, handler: fn(counters_reader: Pin<&mut CountersReader>, registration_id: i64, counter_id: i32) -> ());

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_unavailable_counter_handler"]
        fn addUnavailableCounterHandler(client_conductor: Pin<&mut ClientConductor>, handler: fn(counters_reader: Pin<&mut CountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_unavailable_counter_handler"]
        fn removeUnavailableCounterHandler(client_conductor: Pin<&mut ClientConductor>, handler: fn(counters_reader: Pin<&mut CountersReader>, registration_id: i64, counter_id: i32) -> ());

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_close_client_handler"]
        fn addCloseClientHandler(client_conductor: Pin<&mut ClientConductor>, handler: fn() -> ()) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_close_client_handler"]
        fn removeCloseClientHandler(client_conductor: Pin<&mut ClientConductor>, handler: fn() -> ());

    }
}