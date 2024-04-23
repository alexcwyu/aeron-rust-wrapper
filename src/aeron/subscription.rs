
#[cxx::bridge(namespace = "aeron")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

        #[namespace = "aeron::concurrent::logbuffer"]
        type Header = crate::aeron::concurrent::logbuffer::header::ffi::Header;
        #[namespace = "aeron"]
        type ControlledPollAction = crate::aeron::image::ffi::ControlledPollAction;

        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/Subscription.h");
        include!("aeron-rust-wrapper/cxx_wrapper/Subscription.cpp");

        type Subscription;
        fn say_hello_subscription();

        fn channel(self: &Subscription) -> &CxxString;
        fn streamId(self: &Subscription) -> i32;
        fn registrationId(self: &Subscription) -> i64;
        fn channelStatusId(self: &Subscription) -> i32;
        fn channelStatus(self: &Subscription) -> i64;
        fn addDestination(self: Pin<&mut Subscription>, endpointChannel: &CxxString) ->i64;
        fn removeDestination(self: Pin<&mut Subscription>, endpointChannel: &CxxString) ->i64;
        fn findDestinationResponse(self: Pin<&mut Subscription>, correlationId: i64) -> bool;

        fn isConnected(self: &Subscription) -> bool;
        fn imageCount(self: &Subscription) -> i32;
        fn isClosed(self: &Subscription) -> bool;
        fn hasImage(self: &Subscription, correlationId: i64) -> bool;


        //std::vector<std::string> localSocketAddresses() const

        //std::string tryResolveChannelEndpointPort() const

        //std::string resolvedEndpoint() const

        //std::shared_ptr<Image> imageBySessionId(std::int32_t sessionId) const

        //std::shared_ptr<Image> imageByIndex(std::size_t index) const

        //Image &imageAtIndex(std::size_t index) const

        //std::shared_ptr<std::vector<Image>> images() cost

        //std::shared_ptr<std::vector<std::shared_ptr<Image>>> copyOfImageList() const

        //int forEachImage(F &&func) const

        //Image::array_t addImage(std::shared_ptr<Image> image)

        //std::pair<Image::array_t, std::size_t> removeImage(std::int64_t correlationId)

        //std::pair<Image::array_t, std::size_t> closeAndRemoveImages()

        fn subscription_poll(subscription: Pin<&mut Subscription>, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> (), fragmentLimit: i32) -> i32;
        fn subscription_controlled_poll(subscription: Pin<&mut Subscription>, fragmentHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, header: &Header) -> ControlledPollAction, fragmentLimit: i32) -> i32;
        fn subscription_block_poll(subscription: Pin<&mut Subscription>, blockHandler: fn(buffer: &AtomicBuffer, offset: i32, length: i32, sessionId: i32, termId: i32) -> (), blockLengthLimit: i32) -> i32;

    }
}