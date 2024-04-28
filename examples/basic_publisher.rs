use std::ffi::CString;
use std::io::{stdout, Write};
use std::pin::pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use cxx::{CxxString, let_cxx_string};
use lazy_static::lazy_static;

use aeron_rs::concurrent::atomic_buffer::AlignedBuffer;
use aeron_rs::example_config::{DEFAULT_CHANNEL, DEFAULT_STREAM_ID};
use aeron_rust_wrapper::aeron::aeron;
use aeron_rust_wrapper::aeron::concurrent::atomic_buffer;
use aeron_rust_wrapper::aeron::context;

lazy_static! {
    pub static ref RUNNING: AtomicBool = AtomicBool::from(true);
}

fn sig_int_handler() {
    RUNNING.store(false, Ordering::SeqCst);
}

#[derive(Clone)]
struct Settings {
    dir_prefix: String,
    channel: String,
    stream_id: i32,
    number_of_messages: i64,
    linger_timeout_ms: u64,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            dir_prefix: String::new(),
            channel: String::from(DEFAULT_CHANNEL),
            stream_id: DEFAULT_STREAM_ID.parse().unwrap(),
            number_of_messages: 10,
            linger_timeout_ms: 10000,
        }
    }
}

fn parse_cmd_line() -> Settings {
    Settings::new()
}


fn on_new_publication_handler(channel: &CxxString, stream_id: i32, session_id: i32, correlation_id: i64) {
    println!(
        "Publication: {} {stream_id} {session_id} {correlation_id}",
        channel.to_str().unwrap(),
    );
}

fn str_to_c(val: &str) -> CString {
    CString::new(val).expect("Error converting str to CString")
}

fn convert_mut<T>(reference: &T) -> &mut T {
    unsafe {
        let const_ptr = reference as *const T;
        let mut_ptr = const_ptr as *mut T;
        &mut *mut_ptr
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

    println!(
        "Publishing to channel {} on Stream ID {}",
        settings.channel, settings.stream_id
    );


    let mut context = aeron_rust_wrapper::aeron::context::ffi::new_instance();

    if !settings.dir_prefix.is_empty() {
        let_cxx_string!(dir_prefix = settings.dir_prefix);
        context.as_mut().unwrap().set_aeron_dir(&dir_prefix);
    }


    println!("Using CnC file: {}", context.cnc_file_name());

    context::ffi::new_publication_handler(context.pin_mut(), on_new_publication_handler);
    context.set_pre_touch_mapped_memory(true);

    let mut aeron = aeron::ffi::connect_with_context(context.pin_mut());

    let_cxx_string!(channel = settings.channel);
    let publication_id = aeron.as_mut().unwrap().add_publication(&channel, settings.stream_id);


    let mut publication = aeron.as_mut().unwrap().find_publication(publication_id);
    while publication.is_null() || !publication.is_connected() {
        thread::yield_now();
        publication = aeron.as_mut().unwrap().find_publication(publication_id);
    }

    let publication = convert_mut(publication.as_ref().unwrap());

    let channel_status = publication.channel_status();

    println!(
        "Publication channel status {}",
        channel_status,
    );

    let buffer = AlignedBuffer::with_capacity(256);
    let mut src_buffer = unsafe { atomic_buffer::ffi::new_instance(buffer.ptr, buffer.len as usize) };

    for i in 0..settings.number_of_messages {
        if !RUNNING.load(Ordering::SeqCst) {
            break;
        }

        let str_msg = format!("Basic publisher msg #{}", i);

        let_cxx_string!(c_str_msg = str_msg);

        src_buffer.as_mut().unwrap().put_string(0, &c_str_msg);

        println!("offering {}/{}", i + 1, settings.number_of_messages);
        stdout().flush().ok();

        let publication = convert_mut(publication);
        let result = pin!(publication).offer_part(src_buffer.as_ref().unwrap(), 0, c_str_msg.len() as i32);

        println!("Sent with len {}!", result);


        if !publication.is_connected() {
            println!("No active subscribers detected");
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    println!("Done sending.");

    if settings.linger_timeout_ms > 0 {
        println!("Lingering for {} milliseconds.", settings.linger_timeout_ms);
        std::thread::sleep(Duration::from_millis(settings.linger_timeout_ms));
    }
}
