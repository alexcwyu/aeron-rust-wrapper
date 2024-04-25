
#[cxx::bridge(namespace = "aeron::concurrent")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;


        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/CountersReader.h");
        type CountersReader;

        #[rust_name = "find_by_registration_id"]
        fn findByRegistrationId(self: &CountersReader, registration_id: i64) -> i32;

        #[rust_name = "find_by_type_id_and_registration_id"]
        fn findByTypeIdAndRegistrationId(self: &CountersReader, type_id: i32, registration_id: i64) -> i32;
        #[rust_name = "max_counter_id"]
        fn maxCounterId(self: &CountersReader) -> i32;

        #[rust_name = "get_counter_value"]
        fn getCounterValue(self: &CountersReader, id: i32) -> i64;

        #[rust_name = "get_counter_registration_id"]
        fn getCounterRegistrationId(self: &CountersReader, id: i32) -> i64;

        #[rust_name = "get_counter_owner_id"]
        fn getCounterOwnerId(self: &CountersReader, id: i32) -> i64;


        #[rust_name = "get_counter_state"]
        fn getCounterState(self: &CountersReader, id: i32) -> i32;

        #[rust_name = "get_counter_type_id"]
        fn getCounterTypeId(self: &CountersReader, id: i32) -> i32;

        #[rust_name = "get_free_for_reuse_deadline"]
        fn getFreeForReuseDeadline(self: &CountersReader, id: i32) -> i64;

        // #[rust_name = "get_counter_label"]
        // fn getCounterLabel(self: &CountersReader, id: i32) -> &CxxString;


        include!("aeron-rust-wrapper/cxx_wrapper/concurrent/CountersReader.cpp");

        #[namespace = "aeron::concurrent::counter_reader"]
        #[rust_name = "say_hello"]
        fn sayHello();

        // #[namespace = "aeron::concurrent::counter_reader"]
        // #[rust_name = "values_buffer"]
        // fn valuesBuffer(counter_reader: &CountersReader) -> SharedPtr<AtomicBuffer>;
        //
        // #[namespace = "aeron::concurrent::counter_reader"]
        // #[rust_name = "meta_data_buffer"]
        // fn metaDataBuffer(counter_reader: &CountersReader) -> SharedPtr<AtomicBuffer>;

        #[namespace = "aeron::concurrent::counter_reader"]
        #[rust_name = "values_buffer"]
        fn valuesBuffer(counter_reader: &CountersReader) -> UniquePtr<AtomicBuffer>;

        #[namespace = "aeron::concurrent::counter_reader"]
        #[rust_name = "meta_data_buffer"]
        fn metaDataBuffer(counter_reader: &CountersReader) -> UniquePtr<AtomicBuffer>;

    }

    impl SharedPtr<CountersReader> {}
}