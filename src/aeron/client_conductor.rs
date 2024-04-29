use std::ops::Deref;
use std::pin::Pin;

use cxx::{CxxString, SharedPtr};

use crate::aeron::concurrent::counters_reader::ffi::CxxCountersReader;
use crate::aeron::counter::ffi::CxxCounter;
use crate::aeron::exclusive_publication::ffi::CxxExclusivePublication;
use crate::aeron::image::ffi::CxxImage;
use crate::aeron::publication::ffi::CxxPublication;
use crate::aeron::subscription::ffi::CxxSubscription;

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
        #[rust_name = "CxxSubscription"]
        type Subscription = crate::aeron::subscription::ffi::CxxSubscription;

        #[namespace = "aeron"]
        #[rust_name = "CxxPublication"]
        type Publication = crate::aeron::publication::ffi::CxxPublication;

        #[namespace = "aeron"]
        #[rust_name = "CxxExclusivePublication"]
        type ExclusivePublication = crate::aeron::exclusive_publication::ffi::CxxExclusivePublication;

        #[namespace = "aeron::concurrent"]
        #[rust_name = "CxxCountersReader"]
        type CountersReader = crate::aeron::concurrent::counters_reader::ffi::CxxCountersReader;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/ClientConductor.h");
        include!("aeron-rust-wrapper/cxx_wrapper/ClientConductor.cpp");

        // #[rust_name = "release_subscription"]
        // unsafe fn releaseSubscription(conductor: &SharedPtr<ClientConductor>, registration_id: i64, image_array: * SharedPtr<Image>, length: usize);


        // #[namespace = "aeron::client_conductor"]
        // #[rust_name = "close_all_resources"]
        // fn closeAllResources(conductor: &SharedPtr<ClientConductor>, nowMs: i128);


        // #[namespace = "aeron::client_conductor"]
        // #[rust_name = "counters_reader"]
        // fn countersReader(conductor: &SharedPtr<ClientConductor>) -> Pin<&mut CountersReader>;


        #[rust_name = "CxxClientConductor"]
        type ClientConductor;


        #[rust_name = "channel_status"]
        fn channelStatus(self: &CxxClientConductor, counter_id: i32) -> i64;

        #[rust_name = "is_closed"]
        fn isClosed(self: &CxxClientConductor) -> bool;


        #[rust_name = "ensure_open"]
        fn ensureOpen(self: &CxxClientConductor);


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_start"]
        fn onStart(conductor: &SharedPtr<CxxClientConductor>);

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "do_work"]
        fn doWork(conductor: &SharedPtr<CxxClientConductor>) -> i32;


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_close"]
        fn onClose(conductor: &SharedPtr<CxxClientConductor>);

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_publication"]
        fn addPublication(conductor: &SharedPtr<CxxClientConductor>, channel: String, stream_id: i32) -> i64;


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "find_publication"]
        fn findPublication(conductor: &SharedPtr<CxxClientConductor>, registration_id: i64) -> SharedPtr<CxxPublication>;


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "release_publication"]
        fn releasePublication(conductor: &SharedPtr<CxxClientConductor>, registration_id: i64);

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_exclusive_publication"]
        fn addExclusivePublication(conductor: &SharedPtr<CxxClientConductor>, channel: String, stream_id: i32) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "find_exclusive_publication"]
        fn findExclusivePublication(conductor: &SharedPtr<CxxClientConductor>, registration_id: i64) -> SharedPtr<CxxExclusivePublication>;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "release_exclusive_publication"]
        fn releaseExclusivePublication(conductor: &SharedPtr<CxxClientConductor>, registration_id: i64);

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "find_subscription"]
        fn findSubscription(conductor: &SharedPtr<CxxClientConductor>, registration_id: i64) -> SharedPtr<CxxSubscription>;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_counter"]
        unsafe fn addCounter(conductor: &SharedPtr<CxxClientConductor>, type_id: i32, key_buffer: *const u8, key_length: usize, label: String) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "find_counter"]
        fn findCounter(conductor: &SharedPtr<CxxClientConductor>, registration_id: i64) -> SharedPtr<CxxCounter>;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "release_counter"]
        fn releaseCounter(conductor: &SharedPtr<CxxClientConductor>, registration_id: i64);

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "find_destination_response"]
        fn findDestinationResponse(conductor: &SharedPtr<CxxClientConductor>, correlation_id: i64) -> bool;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_new_publication"]
        fn onNewPublication(
            conductor: &SharedPtr<CxxClientConductor>,
            registrationId: i64,
            originalRegistrationId: i64,
            streamId: i32,
            sessionId: i32,
            publicationLimitCounterId: i32,
            channelStatusIndicatorId: i32,
            logFilename: &CxxString,
        );

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_new_exclusive_publication"]
        fn onNewExclusivePublication(
            conductor: &SharedPtr<CxxClientConductor>,
            registrationId: i64,
            originalRegistrationId: i64,
            streamId: i32,
            sessionId: i32,
            publicationLimitCounterId: i32,
            channelStatusIndicatorId: i32,
            logFilename: &CxxString,
        );

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_subscription_ready"]
        fn onSubscriptionReady(conductor: &SharedPtr<CxxClientConductor>, registrationId: i64, channelStatusId: i32);

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_operation_success"]
        fn onOperationSuccess(conductor: &SharedPtr<CxxClientConductor>, correlationId: i64);

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_channel_endpoint_error_response"]
        fn onChannelEndpointErrorResponse(conductor: &SharedPtr<CxxClientConductor>, channelStatusId: i32, errorMessage: &CxxString);

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_error_response"]
        fn onErrorResponse(
            conductor: &SharedPtr<CxxClientConductor>,
            offendingCommandCorrelationId: i64,
            errorCode: i32,
            errorMessage: &CxxString,
        );


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_available_image"]
        fn onAvailableImage(
            conductor: &SharedPtr<CxxClientConductor>,
            correlationId: i64,
            sessionId: i32,
            subscriberPositionId: i32,
            subscriptionRegistrationId: i64,
            logFilename: &CxxString,
            sourceIdentity: &CxxString,
        );

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_unavailable_image"]
        fn onUnavailableImage(conductor: &SharedPtr<CxxClientConductor>, correlationId: i64, subscriptionRegistrationId: i64);


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_available_counter"]
        fn onAvailableCounter(conductor: &SharedPtr<CxxClientConductor>, registrationId: i64, counterId: i32);


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_unavailable_counter"]
        fn onUnavailableCounter(conductor: &SharedPtr<CxxClientConductor>, registrationId: i64, counterId: i32);


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "on_client_timeout"]
        fn onClientTimeout(conductor: &SharedPtr<CxxClientConductor>, clientId: i64);

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_destination"]
        fn addDestination(conductor: &SharedPtr<CxxClientConductor>, publicationRegistrationId: i64, endpointChannel: String) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_destination"]
        fn removeDestination(conductor: &SharedPtr<CxxClientConductor>, publicationRegistrationId: i64, endpointChannel: String) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_rcv_destination"]
        fn addRcvDestination(conductor: &SharedPtr<CxxClientConductor>, subscriptionRegistrationId: i64, endpointChannel: String) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_rcv_destination"]
        fn removeRcvDestination(conductor: &SharedPtr<CxxClientConductor>, subscriptionRegistrationId: i64, endpointChannel: String) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_available_counter_handler_by_id"]
        fn removeAvailableCounterHandler(conductor: &SharedPtr<CxxClientConductor>, registrationId: i64);


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_unavailable_counter_handler_by_id"]
        fn removeUnavailableCounterHandler(conductor: &SharedPtr<CxxClientConductor>, registrationId: i64);

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_close_client_handler_by_id"]
        fn removeCloseClientHandler(conductor: &SharedPtr<CxxClientConductor>, registrationId: i64);


        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_subscription"]
        fn addSubscription(conductor: &SharedPtr<CxxClientConductor>, channel: String, stream_id: i32, on_available_image_t: fn(image: Pin<&mut CxxImage>) -> (), on_unavailable_image_t: fn(image: Pin<&mut CxxImage>) -> ()) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_available_counter_handler"]
        fn addAvailableCounterHandler(conductor: &SharedPtr<CxxClientConductor>, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_available_counter_handler"]
        fn removeAvailableCounterHandler(conductor: &SharedPtr<CxxClientConductor>, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ());

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_unavailable_counter_handler"]
        fn addUnavailableCounterHandler(conductor: &SharedPtr<CxxClientConductor>, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_unavailable_counter_handler"]
        fn removeUnavailableCounterHandler(conductor: &SharedPtr<CxxClientConductor>, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ());

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "add_close_client_handler"]
        fn addCloseClientHandler(conductor: &SharedPtr<CxxClientConductor>, handler: fn() -> ()) -> i64;

        #[namespace = "aeron::client_conductor"]
        #[rust_name = "remove_close_client_handler"]
        fn removeCloseClientHandler(conductor: &SharedPtr<CxxClientConductor>, handler: fn() -> ());

    }

    impl SharedPtr<CxxClientConductor> {}
}

unsafe impl Sync for ffi::CxxClientConductor {}
unsafe impl Send for ffi::CxxClientConductor {}


#[derive(Clone)]
pub struct ClientConductor {
    conductor: SharedPtr<ffi::CxxClientConductor>,
}

impl ClientConductor {

    pub fn new(conductor: SharedPtr<ffi::CxxClientConductor>) -> Self{
        Self{
            conductor
        }
    }
    #[inline]
    pub fn channel_status(&self, counter_id: i32) -> i64 {
        self.conductor.channel_status(counter_id)
    }

    #[inline]
    pub fn is_closed(&self) -> bool {
        self.conductor.is_closed()
    }

    #[inline]
    pub fn ensure_open(&self) {
        self.conductor.ensure_open()
    }

    #[inline]
    pub fn on_start(&self) {
        ffi::on_start(&self.conductor)
    }

    #[inline]
    pub fn do_work(&self) -> i32 {
        ffi::do_work(&self.conductor)
    }

    #[inline]
    pub fn on_close(&self) {
        ffi::on_close(&self.conductor)
    }

    #[inline]
    pub fn add_publication(&self, channel: String, stream_id: i32) -> i64 {
        ffi::add_publication(&self.conductor, channel, stream_id)
    }

    #[inline]
    pub fn find_publication(&self, registration_id: i64) -> SharedPtr<CxxPublication> {
        ffi::find_publication(&self.conductor, registration_id)
    }

    #[inline]
    pub fn release_publication(&self, registration_id: i64) {
        ffi::release_publication(&self.conductor, registration_id)
    }

    #[inline]
    pub fn add_exclusive_publication(&self, channel: String, stream_id: i32) -> i64 {
        ffi::add_exclusive_publication(&self.conductor, channel, stream_id)
    }

    #[inline]
    pub fn find_exclusive_publication(&self, registration_id: i64) -> SharedPtr<CxxExclusivePublication> {
        ffi::find_exclusive_publication(&self.conductor, registration_id)
    }

    #[inline]
    pub fn release_exclusive_publication(&self, registration_id: i64) {
        ffi::release_exclusive_publication(&self.conductor, registration_id);
    }

    #[inline]
    pub fn find_subscription(&self, registration_id: i64) -> SharedPtr<CxxSubscription> {
        ffi::find_subscription(&self.conductor, registration_id)
    }

    #[inline]
    pub unsafe fn add_counter(&self, type_id: i32, key_buffer: *const u8, key_length: usize, label: String) -> i64 {
        ffi::add_counter(&self.conductor, type_id, key_buffer, key_length, label)
    }

    #[inline]
    pub fn find_counter(&self, registration_id: i64) -> SharedPtr<CxxCounter> {
        ffi::find_counter(&self.conductor, registration_id)
    }

    #[inline]
    pub fn release_counter(&self, registration_id: i64) {
        ffi::release_counter(&self.conductor, registration_id);
    }

    #[inline]
    pub fn find_destination_response(&self, correlation_id: i64) -> bool {
        ffi::find_destination_response(&self.conductor, correlation_id)
    }

    #[inline]
    pub fn on_new_publication(&self, registration_id: i64, original_registration_id: i64, stream_id: i32, session_id: i32, publication_limit_counter_id: i32, channel_status_indicator_id: i32, log_filename: &CxxString) {
        ffi::on_new_publication(&self.conductor, registration_id, original_registration_id, stream_id, session_id, publication_limit_counter_id, channel_status_indicator_id, log_filename);
    }

    #[inline]
    pub fn on_new_exclusive_publication(&self, registration_id: i64, original_registration_id: i64, stream_id: i32, session_id: i32, publication_limit_counter_id: i32, channel_status_indicator_id: i32, log_filename: &CxxString) {
        ffi::on_new_exclusive_publication(&self.conductor, registration_id, original_registration_id, stream_id, session_id, publication_limit_counter_id, channel_status_indicator_id, log_filename);
    }

    #[inline]
    pub fn on_subscription_ready(&self, registration_id: i64, channel_status_id: i32) {
        ffi::on_subscription_ready(&self.conductor, registration_id, channel_status_id);
    }

    #[inline]
    pub fn on_operation_success(&self, correlation_id: i64) {
        ffi::on_operation_success(&self.conductor, correlation_id);
    }

    #[inline]
    pub fn on_channel_endpoint_error_response(&self, channel_status_id: i32, error_message: &CxxString) {
        ffi::on_channel_endpoint_error_response(&self.conductor, channel_status_id, error_message);
    }

    #[inline]
    pub fn on_error_response(&self, offending_command_correlation_id: i64, error_code: i32, error_message: &CxxString) {
        ffi::on_error_response(&self.conductor, offending_command_correlation_id, error_code, error_message);
    }


    #[inline]
    pub fn on_available_image(&self, correlation_id: i64, session_id: i32, subscriber_position_id: i32, subscription_registration_id: i64, log_filename: &CxxString, source_identity: &CxxString) {
        ffi::on_available_image(&self.conductor, correlation_id, session_id, subscriber_position_id, subscription_registration_id, log_filename, source_identity);
    }

    #[inline]
    pub fn on_unavailable_image(&self, correlation_id: i64, subscription_registration_id: i64) {
        ffi::on_unavailable_image(&self.conductor, correlation_id, subscription_registration_id);
    }

    #[inline]
    pub fn on_available_counter(&self, registration_id: i64, counter_id: i32) {
        ffi::on_available_counter(&self.conductor, registration_id, counter_id);
    }

    #[inline]
    pub fn on_unavailable_counter(&self, registration_id: i64, counter_id: i32) {
        ffi::on_unavailable_counter(&self.conductor, registration_id, counter_id);
    }

    #[inline]
    pub fn on_client_timeout(&self, client_id: i64) {
        ffi::on_client_timeout(&self.conductor, client_id);
    }

    #[inline]
    pub fn add_destination(&self, publication_registration_id: i64, endpoint_channel: String) -> i64 {
        ffi::add_destination(&self.conductor, publication_registration_id, endpoint_channel)
    }

    #[inline]
    pub fn remove_destination(&self, publication_registration_id: i64, endpoint_channel: String) -> i64 {
        ffi::remove_destination(&self.conductor, publication_registration_id, endpoint_channel)
    }

    #[inline]
    pub fn add_rcv_destination(&self, subscription_registration_id: i64, endpoint_channel: String) -> i64 {
        ffi::add_rcv_destination(&self.conductor, subscription_registration_id, endpoint_channel)
    }

    #[inline]
    pub fn remove_rcv_destination(&self, subscription_registration_id: i64, endpoint_channel: String) -> i64 {
        ffi::remove_rcv_destination(&self.conductor, subscription_registration_id, endpoint_channel)
    }

    #[inline]
    pub fn remove_available_counter_handler_by_id(&self, registration_id: i64) {
        ffi::remove_available_counter_handler_by_id(&self.conductor, registration_id);
    }

    #[inline]
    pub fn remove_unavailable_counter_handler_by_id(&self, registration_id: i64) {
        ffi::remove_unavailable_counter_handler_by_id(&self.conductor, registration_id);
    }

    #[inline]
    pub fn remove_close_client_handler_by_id(&self, registration_id: i64) {
        ffi::remove_close_client_handler_by_id(&self.conductor, registration_id);
    }

    #[inline]
    pub fn add_subscription(&self, channel: String, stream_id: i32, on_available_image_t: fn(image: Pin<&mut CxxImage>) -> (), on_unavailable_image_t: fn(image: Pin<&mut CxxImage>) -> ()) -> i64 {
        ffi::add_subscription(&self.conductor, channel, stream_id, on_available_image_t, on_unavailable_image_t)
    }

    #[inline]
    pub fn add_available_counter_handler(&self, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64 {
        ffi::add_available_counter_handler(&self.conductor, handler)
    }

    #[inline]
    pub fn remove_available_counter_handler(&self, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) {
        ffi::remove_available_counter_handler(&self.conductor, handler);
    }

    #[inline]
    pub fn add_unavailable_counter_handler(&self, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) -> i64 {
        ffi::add_unavailable_counter_handler(&self.conductor, handler)
    }

    #[inline]
    pub fn remove_unavailable_counter_handler(&self, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) -> ()) {
        ffi::remove_unavailable_counter_handler(&self.conductor, handler);
    }

    #[inline]
    pub fn add_close_client_handler(&self, handler: fn() -> ()) -> i64 {
        ffi::add_close_client_handler(&self.conductor, handler)
    }

    #[inline]
    pub fn remove_close_client_handler(&self, handler: fn() -> ()) {
        ffi::remove_close_client_handler(&self.conductor, handler);
    }

    pub fn get_ref(&self) -> &SharedPtr<ffi::CxxClientConductor> {
        &self.conductor
    }
}

impl Deref for ClientConductor {
    type Target = ffi::CxxClientConductor;

    fn deref(&self) -> &Self::Target {
        match self.conductor.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null ffi::CxxClientConductor"
            ),
        }
    }
}


impl From <SharedPtr<ffi::CxxClientConductor>> for ClientConductor {
    fn from(conductor: SharedPtr<ffi::CxxClientConductor>) -> Self{
        Self::new(conductor)
    }
}
