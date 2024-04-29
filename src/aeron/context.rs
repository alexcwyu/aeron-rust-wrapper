use std::ffi::CString;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

use cxx::{CxxString, SharedPtr, UniquePtr};

use crate::aeron::concurrent::counters_reader::ffi::CxxCountersReader;
use crate::aeron::image::ffi::CxxImage;

pub trait OnAvailableImage {
    fn call(&self, image: &CxxImage);
    fn clone_box(&self) -> Box<dyn OnAvailableImage>;
}

impl<T> OnAvailableImage for T
    where
        T: Fn(&CxxImage) + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn OnAvailableImage> {
        Box::new(self.clone())
    }

    fn call(&self, image: &CxxImage) {
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
    fn call(&self, image: &CxxImage);
    fn clone_box(&self) -> Box<dyn OnUnavailableImage>;
}

impl Clone for Box<dyn OnUnavailableImage> {
    fn clone(&self) -> Box<dyn OnUnavailableImage> {
        self.clone_box()
    }
}

impl<F> OnUnavailableImage for F
    where
        F: Fn(&CxxImage) + Clone + 'static,
{
    fn call(&self, image: &CxxImage) {
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
    fn call(&self, channel: &CxxString, stream_id: i32, correlation_id: i64);
    fn clone_box(&self) -> Box<dyn OnNewSubscription>;
}

impl Clone for Box<dyn OnNewSubscription> {
    fn clone(&self) -> Box<dyn OnNewSubscription> {
        self.clone_box()
    }
}

impl<F> OnNewSubscription for F
    where
        F: Fn(&CxxString, i32, i64) + Clone + 'static,
{
    fn call(&self, channel: &CxxString, stream_id: i32, correlation_id: i64) {
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
    fn call(&self, counters_reader: &CxxCountersReader, registration_id: i64, counter_id: i32);
    fn clone_box(&self) -> Box<dyn OnAvailableCounter>;
}

impl Clone for Box<dyn OnAvailableCounter> {
    fn clone(&self) -> Box<dyn OnAvailableCounter> {
        self.clone_box()
    }
}

impl<F> OnAvailableCounter for F
    where
        F: Fn(&CxxCountersReader, i64, i32) + Clone + 'static,
{
    fn call(&self, counters_reader: &CxxCountersReader, registration_id: i64, counter_id: i32) {
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
    fn call(&self, counters_reader: &CxxCountersReader, registration_id: i64, counter_id: i32);
    fn clone_box(&self) -> Box<dyn OnUnavailableCounter>;
}

impl Clone for Box<dyn OnUnavailableCounter> {
    fn clone(&self) -> Box<dyn OnUnavailableCounter> {
        self.clone_box()
    }
}

impl<F> OnUnavailableCounter for F
    where
        F: Fn(&CxxCountersReader, i64, i32) + Clone + 'static,
{
    fn call(&self, counters_reader: &CxxCountersReader, registration_id: i64, counter_id: i32) {
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



fn default_on_new_publication_handler(_channel: &CxxString, _stream_id: i32, _session_id: i32, _correlation_id: i64) {}

fn default_on_available_image_handler(_img: &CxxImage) {}

fn default_on_new_subscription_handler(_channel: CString, _stream_id: i32, _correlation_id: i64) {}

fn default_on_unavailable_image_handler(_img: &CxxImage) {}

fn default_on_available_counter_handler(_counters_reader: &CxxCountersReader, _registration_id: i64, _counter_id: i32) {}

fn default_on_unavailable_counter_handler(_counters_reader: &CxxCountersReader, _registration_id: i64, _counter_id: i32) {}

fn default_on_close_client_handler() {}


#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {

        #[namespace = "aeron"]
        #[rust_name = "CxxCounter"]
        type Counter = crate::aeron::counter::ffi::CxxCounter;
        #[namespace = "aeron"]
        #[rust_name = "CxxImage"]
        type Image = crate::aeron::image::ffi::CxxImage;

        #[namespace = "aeron::concurrent"]
        #[rust_name = "CxxCountersReader"]
        type CountersReader = crate::aeron::concurrent::counters_reader::ffi::CxxCountersReader;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Context.h");

        include!("aeron-rust-wrapper/cxx_wrapper/Context.cpp");


        // #[rust_name = "error_handler"]
        // this_t &errorHandler(const exception_handler_t &handler)

        #[rust_name = "CxxContext"]
        type Context;

        #[rust_name = "idle_sleep_duration"]
        fn idleSleepDuration(self: &CxxContext) -> i64;
        #[rust_name = "media_driver_timeout"]
        fn mediaDriverTimeout(self: &CxxContext) -> i64;

        #[namespace = "aeron::context"]
        #[rust_name = "new_instance"]
        fn newInstance(clientName: &str) -> UniquePtr<CxxContext>;

        #[namespace = "aeron::context"]
        fn conclude(context: &UniquePtr<CxxContext>);
        
        #[namespace = "aeron::context"]
        #[rust_name = "set_aeron_dir"]
        fn aeronDir(context: &UniquePtr<CxxContext>, directory: & str);

        #[namespace = "aeron::context"]
        #[rust_name = "client_name"]
        fn clientName(context: &UniquePtr<CxxContext>) -> String;

        #[namespace = "aeron::context"]
        #[rust_name = "set_client_name"]
        fn clientName(context: &UniquePtr<CxxContext>, client_name: & str);

        #[namespace = "aeron::context"]
        #[rust_name = "cnc_file_name"]
        fn cncFileName(context: &UniquePtr<CxxContext>) -> String;

        #[namespace = "aeron::context"]
        #[rust_name = "set_idle_sleep_duration"]
        fn idleSleepDuration(context: &UniquePtr<CxxContext>, value: i64);

        #[namespace = "aeron::context"]
        #[rust_name = "set_media_driver_timeout"]
        fn mediaDriverTimeout(context: &UniquePtr<CxxContext>, value: i64);

        #[namespace = "aeron::context"]
        #[rust_name = "set_resource_linger_timeout"]
        fn resourceLingerTimeout(context: &UniquePtr<CxxContext>, value: i64);
        
        #[namespace = "aeron::context"]
        #[rust_name = "use_conductor_agent_invoker"]
        fn useConductorAgentInvoker(context: &UniquePtr<CxxContext>, value: bool);

        #[namespace = "aeron::context"]
        #[rust_name = "pre_touch_mapped_memory"]
        fn preTouchMappedMemory(context: &UniquePtr<CxxContext>, value: bool);

        #[namespace = "aeron::context"]
        #[rust_name = "new_publication_handler"]
        fn newPublicationHandler(context: &UniquePtr<CxxContext>, handler: fn(channel: &CxxString, stream_id: i32, session_id: i32, correlation_id: i64) ->());

        #[namespace = "aeron::context"]
        #[rust_name = "new_exclusive_publication_handler"]
        fn newExclusivePublicationHandler(context: &UniquePtr<CxxContext>, handler: fn(channel: &CxxString, stream_id: i32, session_id: i32, correlation_id: i64) ->());

        #[namespace = "aeron::context"]
        #[rust_name = "new_subscription_handler"]
        fn newSubscriptionHandler(context: &UniquePtr<CxxContext>, handler: fn(channel: &CxxString, stream_id: i32, correlation_id: i64) ->());

        #[namespace = "aeron::context"]
        #[rust_name = "available_image_handler"]
        fn availableImageHandler(context: &UniquePtr<CxxContext>, handler: fn(counters_reader: Pin<&mut CxxImage>) ->());

        #[namespace = "aeron::context"]
        #[rust_name = "unavailable_image_handler"]
        fn unavailableImageHandler(context: &UniquePtr<CxxContext>, handler: fn(counters_reader: Pin<&mut CxxImage>) ->()) ;

        #[namespace = "aeron::context"]
        #[rust_name = "available_counter_handler"]
        fn availableCounterHandler(context: &UniquePtr<CxxContext>, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) ->());

        #[namespace = "aeron::context"]
        #[rust_name = "unavailable_counter_handler"]
        fn unavailableCounterHandler(context: &UniquePtr<CxxContext>, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32) ->());

        #[namespace = "aeron::context"]
        #[rust_name = "close_client_handler"]
        fn closeClientHandler(context: &UniquePtr<CxxContext>, handler: fn() ->());

        #[namespace = "aeron::context"]
        #[rust_name = "request_driver_termination"]
        unsafe fn requestDriverTermination(directory: &str, tokenBuffer: *const u8, token_length: usize) -> bool;

        #[namespace = "aeron::context"]
        #[rust_name = "default_aeron_path"]
        fn defaultAeronPath() -> String;

    }

    impl UniquePtr<CxxContext> {}
}

unsafe impl Sync for ffi::CxxContext {}
unsafe impl Send for ffi::CxxContext {}


pub struct Context {
    context: UniquePtr<ffi::CxxContext>,
}

impl Context {
    #[inline]
    pub fn new(context: UniquePtr<ffi::CxxContext>) -> Self {
        Self {
            context
        }
    }

    #[inline]
    pub fn new_instance(client_name: &str) -> Self {
        Self {
            context: ffi::new_instance(client_name)
        }
    }

    #[inline]
    pub fn client_name(&self) -> String {
        ffi::client_name(&self.context)
    }

    #[inline]
    pub fn idle_sleep_duration(&self) -> i64 {
        self.context.idle_sleep_duration()
    }

    #[inline]
    pub fn media_driver_timeout(&self) -> i64 {
        self.context.media_driver_timeout()
    }


    #[inline]
    pub fn conclude(&self) {
        ffi::conclude(&self.context);
    }

    #[inline]
    pub fn set_aeron_dir(&self, directory: &str) {
        ffi::set_aeron_dir(&self.context, directory);
    }

    #[inline]
    pub fn set_client_name(&self, client_name: &str) {
        ffi::set_client_name(&self.context, client_name);
    }


    #[inline]
    pub fn default_aeron_path(&self) -> String {
        ffi::default_aeron_path()
    }

    #[inline]
    pub fn cnc_file_name(&self) -> String {
        ffi::cnc_file_name(&self.context)
    }

    #[inline]
    pub fn set_idle_sleep_duration(&self, value: i64) {
        ffi::set_idle_sleep_duration(&self.context, value);
    }

    #[inline]
    pub fn set_media_driver_timeout(&self, value: i64) {
        ffi::set_media_driver_timeout(&self.context, value);
    }

    #[inline]
    pub fn set_resource_linger_timeout(&self, value: i64) {
        ffi::set_resource_linger_timeout(&self.context, value);
    }

    #[inline]
    pub fn use_conductor_agent_invoker(&self, value: bool) {
        ffi::use_conductor_agent_invoker(&self.context, value);
    }

    #[inline]
    pub fn pre_touch_mapped_memory(&self, value: bool) {
        ffi::pre_touch_mapped_memory(&self.context, value);
    }

    #[inline]
    pub fn new_publication_handler(&self, handler: fn(channel: &CxxString, stream_id: i32, session_id: i32, correlation_id: i64)) {
        ffi::new_publication_handler(&self.context, handler);
    }

    #[inline]
    pub fn new_exclusive_publication_handler(&self, handler: fn(channel: &CxxString, stream_id: i32, session_id: i32, correlation_id: i64)) {
        ffi::new_exclusive_publication_handler(&self.context, handler);
    }

    #[inline]
    pub fn new_subscription_handler(&self, handler: fn(channel: &CxxString, stream_id: i32, correlation_id: i64)) {
        ffi::new_subscription_handler(&self.context, handler);
    }

    #[inline]
    pub fn available_image_handler(&self, handler: fn(counters_reader: Pin<&mut CxxImage>)) {
        ffi::available_image_handler(&self.context, handler);
    }

    #[inline]
    pub fn unavailable_image_handler(&self, handler: fn(counters_reader: Pin<&mut CxxImage>)) {
        ffi::unavailable_image_handler(&self.context, handler);
    }

    #[inline]
    pub fn available_counter_handler(&self, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32)) {
        ffi::available_counter_handler(&self.context, handler);
    }

    #[inline]
    pub fn unavailable_counter_handler(&self, handler: fn(counters_reader: Pin<&mut CxxCountersReader>, registration_id: i64, counter_id: i32)) {
        ffi::unavailable_counter_handler(&self.context, handler);
    }

    #[inline]
    pub fn close_client_handler(&self, handler: fn()) {
        ffi::close_client_handler(&self.context, handler);
    }

    #[inline]
    pub unsafe fn request_driver_termination(directory: &str, token_buffer: *const u8, token_length: usize) -> bool {
        ffi::request_driver_termination(directory, token_buffer, token_length)
    }

    pub fn get_ref(&self) -> &UniquePtr<ffi::CxxContext> {
        &self.context
    }

    pub fn as_mut(&mut self) -> Pin<&mut ffi::CxxContext> {
        self.context.as_mut().unwrap()
    }
}


impl Deref for Context {
    type Target = ffi::CxxContext;

    fn deref(&self) -> &Self::Target {
        match self.context.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null ffi::CxxContext"
            ),
        }
    }
}

// impl DerefMut for Context {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         match self.context.as_mut() {
//             Some(target) => Pin::into_inner(target),
//             None => panic!(
//                 "called deref_mut on a null ffi::CxxContext"
//             ),
//         }
//     }
// }

impl From <UniquePtr<ffi::CxxContext>> for Context{
    fn from(context: UniquePtr<ffi::CxxContext>) -> Self{
        Self::new(context)
    }
}
