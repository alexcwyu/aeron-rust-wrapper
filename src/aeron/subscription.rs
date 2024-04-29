use std::ops::Deref;
use std::pin::Pin;

use cxx::SharedPtr;

use crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;
use crate::aeron::concurrent::logbuffer::header::ffi::CxxHeader;
use crate::aeron::image::ffi::{CxxControlledPollAction, CxxImage};

#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type CxxAtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;

        #[namespace = "aeron::concurrent::logbuffer"]
        type CxxHeader = crate::aeron::concurrent::logbuffer::header::ffi::CxxHeader;
        #[namespace = "aeron"]
        type CxxControlledPollAction = crate::aeron::image::ffi::CxxControlledPollAction;

        #[namespace = "aeron"]
        type CxxImage = crate::aeron::image::ffi::CxxImage;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Subscription.h");
        include!("aeron-rust-wrapper/cxx_wrapper/Subscription.cpp");

        //std::vector<std::string> localSocketAddresses() const

        //Image &imageAtIndex(std::size_t index) const

        //std::shared_ptr<std::vector<Image>> images() cost

        //std::shared_ptr<std::vector<std::shared_ptr<Image>>> copyOfImageList() const

        #[rust_name = "CxxSubscription"]
        type Subscription;

        //fn channel(self: &Subscription) -> &CxxString;

        #[rust_name = "stream_id"]
        fn streamId(self: &CxxSubscription) -> i32;

        #[rust_name = "registration_id"]
        fn registrationId(self: &CxxSubscription) -> i64;

        #[rust_name = "channel_status_id"]
        fn channelStatusId(self: &CxxSubscription) -> i32;

        #[rust_name = "channel_status"]
        fn channelStatus(self: &CxxSubscription) -> i64;

        #[rust_name = "is_connected"]
        fn isConnected(self: &CxxSubscription) -> bool;
        #[rust_name = "image_count"]
        fn imageCount(self: &CxxSubscription) -> i32;

        #[rust_name = "image_by_session_id"]
        fn imageBySessionId(self: &CxxSubscription, correlationId: i32) -> SharedPtr<CxxImage>;
        #[rust_name = "image_by_index"]
        fn imageByIndex(self: &CxxSubscription, index: usize) -> SharedPtr<CxxImage>;

        #[rust_name = "is_closed"]
        fn isClosed(self: &CxxSubscription) -> bool;
        #[rust_name = "has_image"]
        fn hasImage(self: &CxxSubscription, correlationId: i64) -> bool;

        #[namespace = "aeron::subscription"]
        fn channel(subscription: &SharedPtr<CxxSubscription>) -> String;
        #[namespace = "aeron::subscription"]
        #[rust_name = "add_image"]
        fn addImage(subscription: &SharedPtr<CxxSubscription>, image: SharedPtr<CxxImage>);

        #[namespace = "aeron::subscription"]
        #[rust_name = "add_destination"]
        fn addDestination(subscription: &SharedPtr<CxxSubscription>, endpointChannel: String) ->i64;

        #[namespace = "aeron::subscription"]
        #[rust_name = "remove_destination"]
        fn removeDestination(subscription: &SharedPtr<CxxSubscription>, endpointChannel: String) ->i64;

        #[namespace = "aeron::subscription"]
        #[rust_name = "find_destination_response"]
        fn findDestinationResponse(subscription: &SharedPtr<CxxSubscription>, correlationId: i64) -> bool;

        #[namespace = "aeron::subscription"]
        fn poll(subscription: &SharedPtr<CxxSubscription>, fragmentHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> (), fragmentLimit: i32) -> i32;

        #[namespace = "aeron::subscription"]
        #[rust_name = "controlled_poll"]
        fn controlledPoll(subscription: &SharedPtr<CxxSubscription>, fragmentHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> CxxControlledPollAction, fragmentLimit: i32) -> i32;

        #[namespace = "aeron::subscription"]
        #[rust_name = "block_poll"]
        fn blockPoll(subscription: &SharedPtr<CxxSubscription>, blockHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, sessionId: i32, termId: i32) -> (), blockLengthLimit: i32) -> i32;

        #[namespace = "aeron::subscription"]
        #[rust_name = "for_each_image"]
        fn forEachImage(subscription: &SharedPtr<CxxSubscription>, imageHandler: fn(image: Pin<&mut CxxImage>) -> ()) -> i32;

        #[namespace = "aeron::subscription"]
        #[rust_name = "remove_image"]
        fn removeImage(subscription: &SharedPtr<CxxSubscription>, correlationId : i64) ;

        #[namespace = "aeron::subscription"]
        #[rust_name = "close_and_remove_images"]
        fn closeAndRemoveImages(subscription: &SharedPtr<CxxSubscription>) ;

        #[namespace = "aeron::subscription"]
        #[rust_name = "try_resolve_channel_endpoint_port"]
        fn tryResolveChannelEndpointPort(subscription: &SharedPtr<CxxSubscription>) -> String;

        #[namespace = "aeron::subscription"]
        #[rust_name = "resolved_endpoint"]
        fn resolvedEndpoint(subscription: &SharedPtr<CxxSubscription>) -> String;

    }
    impl SharedPtr<CxxSubscription> {}

}


unsafe impl Sync for ffi::CxxSubscription {}
unsafe impl Send for ffi::CxxSubscription {}

#[derive(Clone)]
pub struct Subscription{
    subscription: SharedPtr<ffi::CxxSubscription>,
}

impl Subscription{
    pub fn new(subscription: SharedPtr<ffi::CxxSubscription>) -> Self{
        Self{
            subscription
        }
    }

    #[inline]
    pub fn add_image(&self, image: SharedPtr<CxxImage>){
        return ffi::add_image(& self.subscription, image);
    }

    #[inline]
    pub fn poll(&self, fragmentHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> (), fragmentLimit: i32) -> i32{
        return ffi::poll(& self.subscription, fragmentHandler, fragmentLimit);
    }

    #[inline]
    pub fn controlled_poll(&self, fragmentHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, header: &CxxHeader) -> CxxControlledPollAction, fragmentLimit: i32) -> i32{
        return ffi::controlled_poll(& self.subscription, fragmentHandler, fragmentLimit);
    }

    #[inline]
    pub fn block_poll(&self, blockHandler: fn(buffer: &CxxAtomicBuffer, offset: i32, length: i32, sessionId: i32, termId: i32) -> (), blockLengthLimit: i32) -> i32{
        return ffi::block_poll(& self.subscription, blockHandler, blockLengthLimit);
    }

    #[inline]
    pub fn for_each_image(&self, imageHandler: fn(image: Pin<&mut CxxImage>) -> ()) -> i32{
        return ffi::for_each_image(& self.subscription, imageHandler);
    }

    #[inline]
    pub fn remove_image(&self, correlationId : i64){
        return ffi::remove_image(& self.subscription, correlationId);
    }

    #[inline]
    pub fn close_and_remove_images(&self){
        return ffi::close_and_remove_images(& self.subscription);
    }

    #[inline]
    pub fn try_resolve_channel_endpoint_port(&self) -> String{
        return ffi::try_resolve_channel_endpoint_port(& self.subscription);
    }
    #[inline]
    pub fn resolved_endpoint(&self) -> String{
        return ffi::resolved_endpoint(& self.subscription);
    }

    #[inline]
    pub fn channel(&self) -> String{
        return ffi::channel(& self.subscription);
    }

    #[inline]
    pub fn stream_id(&self) -> i32{
        return self.subscription.stream_id();
    }

    #[inline]
    pub fn registration_id(&self) -> i64{
        return self.subscription.registration_id();
    }

    #[inline]
    pub fn channel_status_id(&self) -> i32{
        return self.subscription.channel_status_id();
    }

    #[inline]
    pub fn channel_status(&self) -> i64{
        return self.subscription.channel_status();
    }

    #[inline]
    pub fn is_connected(&self) -> bool{
        return self.subscription.is_connected();
    }


    #[inline]
    pub fn is_null(&self) -> bool{
        return self.subscription.is_null();
    }

    #[inline]
    pub fn is_closed(&self) -> bool{
        return self.subscription.is_closed();
    }


    #[inline]
    pub fn image_count(&self) -> i32{
        return self.subscription.image_count();
    }

    #[inline]
    pub fn image_by_session_id(&self, correlationId: i32) -> SharedPtr<CxxImage>{
        return self.subscription.image_by_session_id(correlationId);
    }

    #[inline]
    pub fn image_by_index(&self, index: usize) -> SharedPtr<CxxImage>{
        return self.subscription.image_by_index(index);
    }

    #[inline]
    pub fn add_destination(&self, endpointChannel: String) ->i64{
        return ffi::add_destination(& self.subscription, endpointChannel);
    }

    #[inline]
    pub fn remove_destination(&self, endpointChannel: String) ->i64{
        return ffi::remove_destination(& self.subscription, endpointChannel);
    }

    #[inline]
    pub fn find_destination_response(&self, correlationId: i64) ->bool{
        return ffi::find_destination_response(& self.subscription, correlationId);
    }

    pub fn get_ref(&self) -> &SharedPtr<ffi::CxxSubscription> {
        &self.subscription
    }
}


impl Deref for Subscription {
    type Target = ffi::CxxSubscription;

    fn deref(&self) -> &Self::Target {
        &self.subscription.as_ref().unwrap()
    }
}


impl From <SharedPtr<ffi::CxxSubscription>> for Subscription{
    fn from(subscription: SharedPtr<ffi::CxxSubscription>) -> Self{
        Self::new(subscription)
    }
}
