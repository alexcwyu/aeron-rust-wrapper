use std::ffi::CString;
use crate::aeron::concurrent::counters_reader::ffi::CountersReader;
use crate::aeron::image::ffi::Image;

pub trait OnAvailableImage {
    fn call(&self, image: &Image);
    fn clone_box(&self) -> Box<dyn OnAvailableImage>;
}

impl<T> OnAvailableImage for T
    where
        T: Fn(&Image) + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn OnAvailableImage> {
        Box::new(self.clone())
    }

    fn call(&self, image: &Image) {
        self(image)
    }
}

impl Clone for Box<dyn OnAvailableImage> {
    fn clone(&self) -> Box<dyn OnAvailableImage> {
        self.clone_box()
    }
}

/**
 * Function called by Aeron to deliver notification that an Image has become unavailable for polling.
 *
 * The Image passed is not guaranteed to be valid after the callback.
 *
 * Implementations should do the minimum work for passing off state to another thread for later processing
 * and should not make a reentrant call back into the Aeron instance.
 *
 * @param image that has become unavailable
 */
pub trait OnUnavailableImage {
    fn call(&self, image: &Image);
    fn clone_box(&self) -> Box<dyn OnUnavailableImage>;
}

impl Clone for Box<dyn OnUnavailableImage> {
    fn clone(&self) -> Box<dyn OnUnavailableImage> {
        self.clone_box()
    }
}

impl<F> OnUnavailableImage for F
    where
        F: Fn(&Image) + Clone + 'static,
{
    fn call(&self, image: &Image) {
        self(image)
    }

    fn clone_box(&self) -> Box<dyn OnUnavailableImage> {
        Box::new(self.clone())
    }
}

/**
 * Function called by Aeron to deliver notification that the media driver has added a Publication successfully.
 *
 * Implementations should do the minimum work for passing off state to another thread for later processing
 * and should not make a reentrant call back into the Aeron instance.
 *
 * @param channel of the Publication
 * @param stream_id within the channel of the Publication
 * @param session_id of the Publication
 * @param correlation_id used by the Publication for adding. Aka the registration_id returned by
 * Aeron::add_publication
 */
pub trait OnNewPublication {
    fn call(&self, channel: CString, stream_id: i32, session_id: i32, correlation_id: i64);
    fn clone_box(&self) -> Box<dyn OnNewPublication>;
}

impl Clone for Box<dyn OnNewPublication> {
    fn clone(&self) -> Box<dyn OnNewPublication> {
        self.clone_box()
    }
}

impl<F> OnNewPublication for F
    where
        F: Fn(CString, i32, i32, i64) + Clone + 'static,
{
    fn call(&self, channel: CString, stream_id: i32, session_id: i32, correlation_id: i64) {
        self(channel, stream_id, session_id, correlation_id)
    }

    fn clone_box(&self) -> Box<dyn OnNewPublication> {
        Box::new(self.clone())
    }
}

/**
 * Function called by Aeron to deliver notification that the media driver has added a Subscription successfully.
 *
 * Implementations should do the minimum work for passing off state to another thread for later processing
 * and should not make a reentrant call back into the Aeron instance.
 *
 * @param channel of the Subscription
 * @param stream_id within the channel of the Subscription
 * @param correlation_id used by the Subscription for adding. Aka the registration_id returned by
 * Aeron::add_subscription
 */
pub trait OnNewSubscription {
    fn call(&self, channel: CString, stream_id: i32, correlation_id: i64);
    fn clone_box(&self) -> Box<dyn OnNewSubscription>;
}

impl Clone for Box<dyn OnNewSubscription> {
    fn clone(&self) -> Box<dyn OnNewSubscription> {
        self.clone_box()
    }
}

impl<F> OnNewSubscription for F
    where
        F: Fn(CString, i32, i64) + Clone + 'static,
{
    fn call(&self, channel: CString, stream_id: i32, correlation_id: i64) {
        self(channel, stream_id, correlation_id)
    }

    fn clone_box(&self) -> Box<dyn OnNewSubscription> {
        Box::new(self.clone())
    }
}

/**
 * Function called by Aeron to deliver notification of a Counter being available.
 *
 * Implementations should do the minimum work for passing off state to another thread for later processing
 * and should not make a reentrant call back into the Aeron instance.
 *
 * @param counters_reader for more detail on the counter.
 * @param registration_id for the counter.
 * @param counter_id      that is available.
 */
pub trait OnAvailableCounter {
    fn call(&self, counters_reader: &CountersReader, registration_id: i64, counter_id: i32);
    fn clone_box(&self) -> Box<dyn OnAvailableCounter>;
}

impl Clone for Box<dyn OnAvailableCounter> {
    fn clone(&self) -> Box<dyn OnAvailableCounter> {
        self.clone_box()
    }
}

impl<F> OnAvailableCounter for F
    where
        F: Fn(&CountersReader, i64, i32) + Clone + 'static,
{
    fn call(&self, counters_reader: &CountersReader, registration_id: i64, counter_id: i32) {
        self(counters_reader, registration_id, counter_id)
    }

    fn clone_box(&self) -> Box<dyn OnAvailableCounter> {
        Box::new(self.clone())
    }
}

/**
 * Function called by Aeron to deliver notification of counter being removed.
 *
 * Implementations should do the minimum work for passing off state to another thread for later processing
 * and should not make a reentrant call back into the Aeron instance.
 *
 * @param counters_reader for more counter details.
 * @param registration_id for the counter.
 * @param counter_id      that is unavailable.
 */
pub trait OnUnavailableCounter {
    fn call(&self, counters_reader: &CountersReader, registration_id: i64, counter_id: i32);
    fn clone_box(&self) -> Box<dyn OnUnavailableCounter>;
}

impl Clone for Box<dyn OnUnavailableCounter> {
    fn clone(&self) -> Box<dyn OnUnavailableCounter> {
        self.clone_box()
    }
}

impl<F> OnUnavailableCounter for F
    where
        F: Fn(&CountersReader, i64, i32) + Clone + 'static,
{
    fn call(&self, counters_reader: &CountersReader, registration_id: i64, counter_id: i32) {
        self(counters_reader, registration_id, counter_id)
    }

    fn clone_box(&self) -> Box<dyn OnUnavailableCounter> {
        Box::new(self.clone())
    }
}

/**
 * Function called when the Aeron client is closed to notify that the client or any of it associated resources
 * should not be used after this event.
 */
pub trait OnCloseClient {
    fn call(&self);
    fn clone_box(&self) -> Box<dyn OnCloseClient>;
}

impl Clone for Box<dyn OnCloseClient> {
    fn clone(&self) -> Box<dyn OnCloseClient> {
        self.clone_box()
    }
}

impl<F> OnCloseClient for F
    where
        F: Fn() + Clone + 'static,
{
    fn call(&self) {
        self()
    }
    fn clone_box(&self) -> Box<dyn OnCloseClient> {
        Box::new(self.clone())
    }
}



fn default_on_new_publication_handler(_channel: CString, _stream_id: i32, _session_id: i32, _correlation_id: i64) {}

fn default_on_available_image_handler(_img: &Image) {}

fn default_on_new_subscription_handler(_channel: CString, _stream_id: i32, _correlation_id: i64) {}

fn default_on_unavailable_image_handler(_img: &Image) {}

fn default_on_available_counter_handler(_counters_reader: &CountersReader, _registration_id: i64, _counter_id: i32) {}

fn default_on_unavailable_counter_handler(_counters_reader: &CountersReader, _registration_id: i64, _counter_id: i32) {}

fn default_on_close_client_handler() {}


#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {

        #[namespace = "aeron"]
        type Counter = crate::aeron::counter::ffi::Counter;
        #[namespace = "aeron"]
        type Image = crate::aeron::image::ffi::Image;


        #[namespace = "aeron::concurrent"]
        type CountersReader = crate::aeron::concurrent::counters_reader::ffi::CountersReader;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Context.h");

        type Context;


        fn conclude<'a>(self: Pin<&'a mut Context>) ->Pin<&'a mut Context>;
        

        #[rust_name = "set_aeron_dir"]
        fn aeronDir<'a>(self: Pin<&'a mut Context>, directory: & CxxString) ->Pin<&'a mut Context>;


        #[rust_name = "set_client_name"]
        fn clientName<'a>(self: Pin<&'a mut Context>, client_name: & CxxString) ->Pin<&'a mut Context>;


        #[rust_name = "client_name"]
        fn clientName(self: & Context) -> &CxxString;

        #[rust_name = "cnc_file_name"]
        fn cncFileName(self: &Context) -> &CxxString;

        #[rust_name = "set_idle_sleep_duration"]
        fn idleSleepDuration<'a>(self: Pin<&'a mut Context>, value: i64) ->Pin<&'a mut Context>;

        #[rust_name = "idle_sleep_duration"]
        fn idleSleepDuration(self: &Context) -> i64;


        #[rust_name = "set_media_driver_timeout"]
        fn mediaDriverTimeout<'a>(self: Pin<&'a mut Context>, value: i64) ->Pin<&'a mut Context>;

        #[rust_name = "media_driver_timeout"]
        fn mediaDriverTimeout(self: &Context) -> i64;

        #[rust_name = "set_resource_linger_timeout"]
        fn resourceLingerTimeout<'a>(self: Pin<&'a mut Context>, value: i64) ->Pin<&'a mut Context>;

        // #[rust_name = "resource_linger_timeout"]
        // fn resourceLingerTimeout(self: &Context) -> i64;

        #[rust_name = "use_conductor_agent_invoker"]
        fn useConductorAgentInvoker<'a>(self: Pin<&'a mut Context>, value: bool) ->Pin<&'a mut Context>;

        #[rust_name = "pre_touch_mapped_memory"]
        fn preTouchMappedMemory<'a>(self: Pin<&'a mut Context>, value: bool) ->Pin<&'a mut Context>;



        // #[rust_name = "error_handler"]
        // this_t &errorHandler(const exception_handler_t &handler)




        include!("aeron-rust-wrapper/cxx_wrapper/Context.cpp");
        #[namespace = "aeron::context"]
        #[rust_name = "new_publication_handler"]
        fn newPublicationHandler<'a>(context: Pin<&'a mut Context>, handler: fn(channel: &CxxString, stream_id: i32, session_id: i32, correlation_id: i64)->()) -> Pin<&'a mut Context>;

        #[namespace = "aeron::context"]
        #[rust_name = "new_exclusive_publication_handler"]
        fn newExclusivePublicationHandler<'a>(context: Pin<&'a mut Context>, handler: fn(channel: &CxxString, stream_id: i32, session_id: i32, correlation_id: i64) ->()) -> Pin<&'a mut Context>;

        #[namespace = "aeron::context"]
        #[rust_name = "new_subscription_handler"]
        fn newSubscriptionHandler<'a>(context: Pin<&'a mut Context>, handler: fn(channel: &CxxString, stream_id: i32, correlation_id: i64) ->()) -> Pin<&'a mut Context>;


        #[namespace = "aeron::context"]
        #[rust_name = "available_image_handler"]
        fn availableImageHandler<'a>(context: Pin<&'a mut Context>, handler: fn(counters_reader: Pin<&mut Image>) ->()) -> Pin<&'a mut Context>;


        #[namespace = "aeron::context"]
        #[rust_name = "unavailable_image_handler"]
        fn unavailableImageHandler<'a>(context: Pin<&'a mut Context>, handler: fn(counters_reader: Pin<&mut Image>) ->()) -> Pin<&'a mut Context>;



        #[namespace = "aeron::context"]
        #[rust_name = "available_counter_handler"]
        fn availableCounterHandler<'a>(context: Pin<&'a mut Context>, handler: fn(counters_reader: Pin<&mut CountersReader>, registration_id: i64, counter_id: i32) ->()) -> Pin<&'a mut Context>;


        #[namespace = "aeron::context"]
        #[rust_name = "unavailable_counter_handler"]
        fn unavailableCounterHandler<'a>(context: Pin<&'a mut Context>, handler: fn(counters_reader: Pin<&mut CountersReader>, registration_id: i64, counter_id: i32) ->()) -> Pin<&'a mut Context>;


        #[namespace = "aeron::context"]
        #[rust_name = "close_client_handler"]
        fn closeClientHandler<'a>(context: Pin<&'a mut Context>, handler: fn() ->()) -> Pin<&'a mut Context>;


        #[namespace = "aeron::context"]
        #[rust_name = "requestDriverTermination"]
        unsafe fn requestDriverTermination(directory: &CxxString, tokenBuffer: *const u8, token_length: usize) -> bool;


        #[namespace = "aeron::context"]
        #[rust_name = "default_aeron_path"]
        fn defaultAeronPath() -> String;

        #[namespace = "aeron::context"]
        #[rust_name = "say_hello"]
        fn sayHello();
    }

    impl SharedPtr<Context> {}
}