use std::ffi::CString;
use std::pin::Pin;
use std::slice;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

use cxx::CxxString;
use lazy_static::lazy_static;

use aeron_rs::concurrent::strategies::{SleepingIdleStrategy, Strategy};
use aeron_rs::example_config::{DEFAULT_CHANNEL, DEFAULT_STREAM_ID};
use aeron_rust_wrapper::aeron::aeron::Aeron;
use aeron_rust_wrapper::aeron::concurrent::atomic_buffer;
use aeron_rust_wrapper::aeron::concurrent::logbuffer::header;
use aeron_rust_wrapper::aeron::context;
use aeron_rust_wrapper::aeron::image;

lazy_static! {
    pub static ref RUNNING: AtomicBool = AtomicBool::from(true);
    pub static ref SUBSCRIPTION_ID: AtomicI64 = AtomicI64::new(-1);
}

fn sig_int_handler() {
    RUNNING.store(false, Ordering::SeqCst);
}

#[derive(Clone)]
struct Settings {
    dir_prefix: String,
    channel: String,
    stream_id: i32,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            dir_prefix: String::new(),
            channel: String::from(DEFAULT_CHANNEL),
            stream_id: DEFAULT_STREAM_ID.parse().unwrap(),
        }
    }
}

fn parse_cmd_line() -> Settings {
    Settings::new()
}

fn available_image_handler(image: Pin<& mut image::ffi::CxxImage>) {
    println!(
        "Available image correlation_id={} session_id={} at position={}",
        image.correlation_id(),
        image.session_id(),
        image.position(),
    );
}

fn unavailable_image_handler(image: Pin<& mut image::ffi::CxxImage>) {
    println!(
        "Unavailable image correlation_id={} session_id={} at position={}",
        image.correlation_id(),
        image.session_id(),
        image.position(),
    );
}


fn str_to_c(val: &str) -> CString {
    CString::new(val).expect("Error converting str to CString")
}

fn on_new_fragment(buffer: &atomic_buffer::ffi::CxxAtomicBuffer, offset: i32, length: i32, header: & header::ffi::CxxHeader){
    unsafe {
        // println!(
        //     "Message to stream {} from session {} ({}@{})",
        //     header.stream_id(),
        //     header.session_id(),
        //     length,
        //     offset
        // );
        let slice_msg = slice::from_raw_parts_mut(buffer.buffer().offset(offset as isize), length as usize);
        let msg = CString::new(slice_msg).unwrap();
        println!(
            "Message to stream {} from session {} ({}@{}): <<{}>>",
            header.stream_id(),
            header.session_id(),
            length,
            offset,
            msg.to_str().unwrap()
        );
    }
}

fn main() {
    pretty_env_logger::init();
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        sig_int_handler();
    })
    .expect("Error setting Ctrl-C handler");

    let settings = parse_cmd_line();

    println!("Subscribing Pong at {} on Stream ID {}", settings.channel, settings.stream_id);

    let mut context = context::Context::new_instance("test");
    //let mut context = aeron_rust_wrapper::aeron::context::ffi::new_instance("test".to_string());


    println!("setting prefix");
    if !settings.dir_prefix.is_empty() {
        context.set_aeron_dir(&settings.dir_prefix);
    }

    println!("Using CnC file: {}", context.cnc_file_name());
    println!("client name: {}",context.client_name());


    println!("new_subscription_handler");
    context.new_subscription_handler(|channel: &CxxString, stream_id: i32, correlation_id: i64| {
        println!("Subscription: {} {} {}", channel, stream_id, correlation_id)
    });

    println!("available_image_handler");
    context.available_image_handler(available_image_handler);

    println!("unavailable_image_handler");
    context.unavailable_image_handler(unavailable_image_handler);

    println!("pre_touch_mapped_memory");
    context.pre_touch_mapped_memory(true);


    println!("setting aeron");
    let mut aeron = Aeron::connect_with_context(context.as_mut());


    println!("add_subscription");
    let subscription_id = aeron.add_subscription(settings.channel, settings.stream_id);

    SUBSCRIPTION_ID.store(subscription_id, Ordering::SeqCst);

    println!("find_subscription");
    let mut subscription = aeron.find_subscription(subscription_id);

    while subscription.is_null() ||!subscription.is_connected() {
        std::thread::yield_now();
        subscription = aeron.find_subscription(subscription_id);
    }


    println!("poll");
    let mut idle_strategy = SleepingIdleStrategy::new(1000);

    while RUNNING.load(Ordering::SeqCst) {
        let fragments_read = subscription.poll(on_new_fragment, 10);
        idle_strategy.idle_opt(fragments_read);
    }
}
