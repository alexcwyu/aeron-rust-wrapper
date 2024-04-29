use std::ffi::CString;
use std::io::{stdout, Write};
use std::pin::pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{slice, thread};
use std::ops::Deref;
use std::time::Duration;

use cxx::{CxxString, let_cxx_string};
use lazy_static::lazy_static;
use nix::NixPath;

use aeron_rs::concurrent::atomic_buffer::{AlignedBuffer};
use aeron_rs::example_config::{DEFAULT_CHANNEL, DEFAULT_STREAM_ID};
use aeron_rust_wrapper::aeron::aeron;
use aeron_rust_wrapper::aeron::aeron::Aeron;
use aeron_rust_wrapper::aeron::concurrent::atomic_buffer;
use aeron_rust_wrapper::aeron::concurrent::atomic_buffer::AtomicBuffer;
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

    let mut context = context::Context::new_instance("test");

    println!("setting prefix");
    if !settings.dir_prefix.is_empty() {
        context.set_aeron_dir(&settings.dir_prefix);
    }



    println!("Using CnC file: {}", context.cnc_file_name());
    println!("client name: {}",context.client_name());

    context.new_publication_handler(on_new_publication_handler);
    context.pre_touch_mapped_memory(true);


    println!("setting aeron");
    let mut aeron = Aeron::connect_with_context(context.as_mut());

    println!("add_subscription");
    let publication_id = aeron.add_publication(settings.channel, settings.stream_id);


    let mut publication = aeron.find_publication(publication_id);
    while publication.is_null() || !publication.is_connected() {
        thread::yield_now();
        publication = aeron.find_publication(publication_id);
    }


    let channel_status = publication.channel_status();

    println!(
        "Publication channel status {}",
        channel_status,
    );

    let buffer = AlignedBuffer::with_capacity(256);
    let mut src_buffer = unsafe { AtomicBuffer::wrap_bytes(buffer.ptr, buffer.len as usize) };

    for i in 0..settings.number_of_messages {
        if !RUNNING.load(Ordering::SeqCst) {
            break;
        }

        let str_msg = format!("Basic publisher msg #{}", i);

        let_cxx_string!(c_str_msg = str_msg);

        let str_msg = format!("Basic publisher msg #{}", i);
        let c_str_msg = CString::new(str_msg.clone()).unwrap();
        let src = c_str_msg.as_bytes();
        let len = src.len();
        unsafe {
            src_buffer.put_bytes(0, src.as_ptr(), src.len() as i32);

            let slice_msg = slice::from_raw_parts_mut(src_buffer.buffer().offset(0), len as usize);
            let msg = CString::new(slice_msg).unwrap();

            println!("offering {}/{}, input str={}, input str len={}, src len={}, output={}", i + 1, settings.number_of_messages, str_msg, str_msg.len(), len, msg.to_str().unwrap());
        }



        stdout().flush().ok();

        loop {
            let result = publication.offer_part(&src_buffer, 0, c_str_msg.len() as i32);



            if result > 0 {
                println!("Sent with code {}!", result);
                break;
            }
            else
            {
                println!("resending... {}!", result);
                std::thread::sleep(Duration::from_millis(1));
            }
        }

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
