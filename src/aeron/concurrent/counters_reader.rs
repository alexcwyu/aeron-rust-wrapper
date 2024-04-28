use cxx::{CxxString, UniquePtr};

use crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;

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

        include!("aeron-rust-wrapper/cxx_wrapper/concurrent/CountersReader.cpp");

        #[namespace = "aeron::concurrent::counter_reader"]
        #[rust_name = "get_counter_label"]
        fn getCounterLabel(counter_reader: &CountersReader, id: i32) -> String;


        #[namespace = "aeron::concurrent::counter_reader"]
        #[rust_name = "get_values_buffer"]
        fn getValuesBuffer(counter_reader: &CountersReader, dest: & UniquePtr<AtomicBuffer>) ;

        #[namespace = "aeron::concurrent::counter_reader"]
        #[rust_name = "get_meta_data_buffer"]
        fn getMetaDataBuffer(counter_reader: &CountersReader, dest: & UniquePtr<AtomicBuffer>);

        #[namespace = "aeron::concurrent::counter_reader"]
        #[rust_name = "for_each"]
        fn forEach(counter_reader: &CountersReader, function: fn(id: i32, type_id: i32, buffer: &AtomicBuffer, label: &CxxString) -> ());

        #[namespace = "aeron::concurrent::counter_reader"]
        #[rust_name = "new_instance"]
        fn newInstance(metadataBuffer :&AtomicBuffer, valuesBuffer: & AtomicBuffer) -> UniquePtr<CountersReader>;

    }

    impl SharedPtr<CountersReader> {}
}

pub struct CountersReader {
    counters_reader: ffi::CountersReader,
}

impl CountersReader {
    #[inline]
    pub fn new(counters_reader: ffi::CountersReader) -> Self {
        Self {
            counters_reader
        }
    }

    #[inline]
    pub fn find_by_registration_id(&self, registration_id: i64) -> i32 {
        self.counters_reader.find_by_registration_id(registration_id)
    }

    #[inline]
    pub fn find_by_type_id_and_registration_id(&self, type_id: i32, registration_id: i64) -> i32 {
        self.counters_reader.find_by_type_id_and_registration_id(type_id, registration_id)
    }

    #[inline]
    pub fn max_counter_id(&self) -> i32 {
        self.counters_reader.max_counter_id()
    }

    #[inline]
    pub fn get_counter_value(&self, id: i32) -> i64 {
        self.counters_reader.get_counter_value(id)
    }

    #[inline]
    pub fn get_counter_registration_id(&self, id: i32) -> i64 {
        self.counters_reader.get_counter_registration_id(id)
    }

    #[inline]
    pub fn get_counter_owner_id(&self, id: i32) -> i64 {
        self.counters_reader.get_counter_owner_id(id)
    }

    #[inline]
    pub fn get_counter_state(&self, id: i32) -> i32 {
        self.counters_reader.get_counter_state(id)
    }

    #[inline]
    pub fn get_counter_type_id(&self, id: i32) -> i32 {
        self.counters_reader.get_counter_type_id(id)
    }

    #[inline]
    pub fn get_free_for_reuse_deadline(&self, id: i32) -> i64 {
        self.counters_reader.get_free_for_reuse_deadline(id)
    }


    #[inline]
    pub fn get_counter_label(&self, id: i32) -> String {
        ffi::get_counter_label(&self.counters_reader, id)
    }

    #[inline]
    pub fn get_values_buffer(&self, dest: & UniquePtr<AtomicBuffer>) {
        ffi::get_values_buffer(&self.counters_reader, dest);
    }

    #[inline]
    pub fn get_meta_data_buffer(&self, dest: & UniquePtr<AtomicBuffer>) {
        ffi::get_meta_data_buffer(&self.counters_reader, dest);
    }

    #[inline]
    pub fn for_each(&self, function: fn(id: i32, type_id: i32, buffer: &AtomicBuffer, label: &CxxString)) {
        ffi::for_each(&self.counters_reader, function);
    }
}


impl From <ffi::CountersReader> for CountersReader {
    fn from(counters_reader: ffi::CountersReader) -> Self{
        Self::new(counters_reader)
    }
}