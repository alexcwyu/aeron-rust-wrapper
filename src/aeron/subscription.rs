use std::pin::Pin;

use cxx::SharedPtr;

use crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;
use crate::aeron::concurrent::logbuffer::header::ffi::Header;
use crate::aeron::image::ffi::{ControlledPollAction, Image};

#[cxx::bridge(namespace = "aeron")]
pub(crate) mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

        #[namespace = "aeron::concurrent::logbuffer"]
        type Header = crate::aeron::concurrent::logbuffer::header::ffi::Header;
        #[namespace = "aeron"]
        type ControlledPollAction = crate::aeron::image::ffi::ControlledPollAction;

        #[namespace = "aeron"]
        type Image = crate::aeron::image::ffi::Image;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Subscription.h");

        type Subscription;

        //fn channel(self: &Subscription) -> &CxxString;

        #[rust_name = "stream_id"]
        fn streamId(self: &Subscription) -> i32;

        #[rust_name = "registration_id"]
        fn registrationId(self: &Subscription) -> i64;

        #[rust_name = "channel_status_id"]
        fn channelStatusId(self: &Subscription) -> i32;

        #[rust_name = "channel_status"]
        fn channelStatus(self: &Subscription) -> i64;

        // #[rust_name = "add_destination"]
        // fn addDestination(self: Pin<&mut Subscription>, endpointChannel: &CxxString) ->i64;
        //
        // #[rust_name = "remove_destination"]
        // fn removeDestination(self: Pin<&mut Subscription>, endpointChannel: &CxxString) ->i64;
        //
        // #[rust_name = "find_destination_response"]
        // fn findDestinationResponse(self: Pin<&mut Subscription>, correlationId: i64) -> bool;

        #[rust_name = "is_connected"]
        fn isConnected(self: &Subscription) -> bool;
        #[rust_name = "image_count"]
        fn imageCount(self: &Subscription) -> i32;


        #[rust_name = "image_by_session_id"]
        fn imageBySessionId(self: &Subscription, correlationId: i32) -> SharedPtr<Image>;
        #[rust_name = "image_by_index"]
        fn imageByIndex(self: &Subscription, index: usize) -> SharedPtr<Image>;


        #[rust_name = "is_closed"]
        fn isClosed(self: &Subscription) -> bool;
        #[rust_name = "has_image"]
        fn hasImage(self: &Subscription, correlationId: i64) -> bool;


        //std::vector<std::string> localSocketAddresses() const

        //std::string tryResolveChannelEndpointPort() const

        //std::string resolvedEndpoint() const

        //Image &imageAtIndex(std::size_t index) const

        //std::shared_ptr<std::vector<Image>> images() cost

        //std::shared_ptr<std::vector<std::shared_ptr<Image>>> copyOfImageList() const

        include!("aeron-rust-wrapper/cxx_wrapper/Subscription.cpp");


        // #[namespace = "aeron::subscription"]
        // type Images;
        //
        // #[namespace = "aeron::subscription"]
        // fn size(self: &Images) -> usize;
        //
        // #[namespace = "aeron::subscription"]
        // #[rust_name = "get_image"]
        // fn getImage(self: &Images, index: usize) -> SharedPtr<Image>;
        //
        // #[namespace = "aeron::subscription"]
        // #[rust_name = "subscription"]
        // unsafe fn getImages(self: &Images) -> *mut SharedPtr<Image>;



        #[namespace = "aeron::subscription"]
        fn channel(subscription: &SharedPtr<Subscription>) -> String;
        #[namespace = "aeron::subscription"]
        #[rust_name = "add_image"]
        fn addImage(subscription: &SharedPtr<Subscription>, image: SharedPtr<Image>);

        #[namespace = "aeron::subscription"]
        #[rust_name = "add_destination"]
        fn addDestination(subscription: &SharedPtr<Subscription>, endpointChannel: String) ->i64;

        #[namespace = "aeron::subscription"]
        #[rust_name = "remove_destination"]
        fn removeDestination(subscription: &SharedPtr<Subscription>, endpointChannel: String) ->i64;

        #[namespace = "aeron::subscription"]
        #[rust_name = "find_destination_response"]
        fn findDestinationResponse(subscription: &SharedPtr<Subscription>, correlationId: i64) -> bool;


        #[namespace = "aeron::subscription"]
        fn poll(subscription: &SharedPtr<Subscription>, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> (), fragmentLimit: i32) -> i32;

        #[namespace = "aeron::subscription"]
        #[rust_name = "controlled_poll"]
        fn controlledPoll(subscription: &SharedPtr<Subscription>, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> ControlledPollAction, fragmentLimit: i32) -> i32;

        #[namespace = "aeron::subscription"]
        #[rust_name = "block_poll"]
        fn blockPoll(subscription: &SharedPtr<Subscription>, blockHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, sessionId: i32, termId: i32) -> (), blockLengthLimit: i32) -> i32;


        #[namespace = "aeron::subscription"]
        #[rust_name = "for_each_image"]
        fn forEachImage(subscription: &SharedPtr<Subscription>, imageHandler: fn(image: Pin<&mut Image>) -> ()) -> i32;


        #[namespace = "aeron::subscription"]
        #[rust_name = "remove_image"]
        fn removeImage(subscription: &SharedPtr<Subscription>, correlationId : i64) ;



        #[namespace = "aeron::subscription"]
        #[rust_name = "close_and_remove_images"]
        fn closeAndRemoveImages(subscription: &SharedPtr<Subscription>) ;

        #[namespace = "aeron::subscription"]
        #[rust_name = "try_resolve_channel_endpoint_port"]
        fn tryResolveChannelEndpointPort(subscription: &SharedPtr<Subscription>) -> String;


        #[namespace = "aeron::subscription"]
        #[rust_name = "resolved_endpoint"]
        fn resolvedEndpoint(subscription: &SharedPtr<Subscription>) -> String;

    }


    impl SharedPtr<Subscription> {}
    // impl SharedPtr<Images> {}

}


unsafe impl Sync for ffi::Subscription {}
unsafe impl Send for ffi::Subscription {}

#[derive(Clone)]
pub struct Subscription{
    subscription: SharedPtr<ffi::Subscription>,
}

impl Subscription{
    pub fn new(subscription: SharedPtr<ffi::Subscription>) -> Self{
        Self{
            subscription
        }
    }

    #[inline]
    pub fn add_image(&self, image: SharedPtr<Image>){
        return ffi::add_image(& self.subscription, image);
    }

    #[inline]
    pub fn poll(&self, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> (), fragmentLimit: i32) -> i32{
        return ffi::poll(& self.subscription, fragmentHandler, fragmentLimit);
    }

    #[inline]
    pub fn controlled_poll(&self, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> ControlledPollAction, fragmentLimit: i32) -> i32{
        return ffi::controlled_poll(& self.subscription, fragmentHandler, fragmentLimit);
    }

    #[inline]
    pub fn block_poll(&self, blockHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, sessionId: i32, termId: i32) -> (), blockLengthLimit: i32) -> i32{
        return ffi::block_poll(& self.subscription, blockHandler, blockLengthLimit);
    }

    #[inline]
    pub fn for_each_image(&self, imageHandler: fn(image: Pin<&mut Image>) -> ()) -> i32{
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

    //#[inline]
    //pub fn channel(&self) -> &CxxString{
    //   return self.subscription.channel();
    //}

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
    pub fn image_count(&self) -> i32{
        return self.subscription.image_count();
    }

    #[inline]
    pub fn image_by_session_id(&self, correlationId: i32) -> SharedPtr<Image>{
        return self.subscription.image_by_session_id(correlationId);
    }

    #[inline]
    pub fn image_by_index(&self, index: usize) -> SharedPtr<Image>{
        return self.subscription.image_by_index(index);
    }

    #[inline]
    pub fn is_closed(&self) -> bool{
        return self.subscription.is_closed();
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

    pub fn get_ref(&self) -> &SharedPtr<ffi::Subscription> {
        &self.subscription
    }
}


impl From <SharedPtr<ffi::Subscription>> for Subscription{
    fn from(subscription: SharedPtr<ffi::Subscription>) -> Self{
        Self::new(subscription)
    }
}
