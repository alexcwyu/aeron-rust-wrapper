
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

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

        fn channel(self: &Subscription) -> &CxxString;

        #[rust_name = "stream_id"]
        fn streamId(self: &Subscription) -> i32;

        #[rust_name = "registration_id"]
        fn registrationId(self: &Subscription) -> i64;

        #[rust_name = "channel_status_id"]
        fn channelStatusId(self: &Subscription) -> i32;

        #[rust_name = "channel_status"]
        fn channelStatus(self: &Subscription) -> i64;

        #[rust_name = "add_destination"]
        fn addDestination(self: Pin<&mut Subscription>, endpointChannel: &CxxString) ->i64;

        #[rust_name = "remove_destination"]
        fn removeDestination(self: Pin<&mut Subscription>, endpointChannel: &CxxString) ->i64;

        #[rust_name = "find_destination_response"]
        fn findDestinationResponse(self: Pin<&mut Subscription>, correlationId: i64) -> bool;

        #[rust_name = "is_connected"]
        fn isConnected(self: &Subscription) -> bool;
        #[rust_name = "image_count"]
        fn imageCount(self: &Subscription) -> i32;


        #[rust_name = "image_by_session_id"]
        fn imageBySessionId(self: &Subscription, correlationId: i32) -> SharedPtr<Image>;
        #[rust_name = "image_by_index"]
        fn imageByIndex(self: &Subscription, index: usize) -> SharedPtr<Image>;

        // #[rust_name = "image_at_index"]
        // fn imageAtIndex(self: Pin<&mut Subscription>, index: usize) -> Pin<&mut Image>;

        // fn copyOfImageList(self: &Subscription) -> SharedPtr<CxxVector<SharedPtr<Image>>>;

        #[rust_name = "is_closed"]
        fn isClosed(self: &Subscription) -> bool;
        #[rust_name = "has_image"]
        fn hasImage(self: &Subscription, correlationId: i64) -> bool;

        unsafe fn addImage(self: Pin<&mut Subscription>, image: SharedPtr<Image>) -> *mut SharedPtr<Image>;



        //std::vector<std::string> localSocketAddresses() const

        //std::string tryResolveChannelEndpointPort() const

        //std::string resolvedEndpoint() const

        //Image &imageAtIndex(std::size_t index) const

        //std::shared_ptr<std::vector<Image>> images() cost

        //std::shared_ptr<std::vector<std::shared_ptr<Image>>> copyOfImageList() const

        include!("aeron-rust-wrapper/cxx_wrapper/Subscription.cpp");


        #[namespace = "aeron::subscription"]
        type Images;

        #[namespace = "aeron::subscription"]
        fn size(self: &Images) -> usize;

        #[namespace = "aeron::subscription"]
        #[rust_name = "get_image"]
        fn getImage(self: &Images, index: usize) -> SharedPtr<Image>;

        #[namespace = "aeron::subscription"]
        #[rust_name = "subscription"]
        unsafe fn getImages(self: &Images) -> *mut SharedPtr<Image>;



        #[namespace = "aeron::subscription"]
        fn poll(subscription: Pin<&mut Subscription>, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> (), fragmentLimit: i32) -> i32;

        #[namespace = "aeron::subscription"]
        #[rust_name = "controlled_poll"]
        fn controlledPoll(subscription: Pin<&mut Subscription>, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> ControlledPollAction, fragmentLimit: i32) -> i32;

        #[namespace = "aeron::subscription"]
        #[rust_name = "block_poll"]
        fn blockPoll(subscription: Pin<&mut Subscription>, blockHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, sessionId: i32, termId: i32) -> (), blockLengthLimit: i32) -> i32;


        #[namespace = "aeron::subscription"]
        #[rust_name = "for_each_image"]
        fn forEachImage(subscription: Pin<&mut Subscription>, imageHandler: fn(image: Pin<&mut Image>) -> ()) -> i32;


        #[namespace = "aeron::subscription"]
        #[rust_name = "remove_image"]
        fn removeImage(subscription: Pin<&mut Subscription>, correlationId : i64) -> UniquePtr<Images> ;



        #[namespace = "aeron::subscription"]
        #[rust_name = "close_and_remove_images"]
        fn closeAndRemoveImages(subscription: Pin<&mut Subscription>) -> UniquePtr<Images> ;


        #[namespace = "aeron::subscription"]
        #[rust_name = "say_hello"]
        fn sayHello();
    }


    impl SharedPtr<Subscription> {}
    impl SharedPtr<Images> {}
}