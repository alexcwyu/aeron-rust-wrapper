//
// use std::ffi::CString;
// use std::pin::Pin;
// use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;
//
// use aeron_rs::concurrent::atomic_buffer::{AlignedBuffer};
// use aeron_rs::example_config::{DEFAULT_CHANNEL, DEFAULT_STREAM_ID};
// use aeron_rust_wrapper::aeron::aeron;
// use aeron_rust_wrapper::aeron::aeron::Aeron;
// use aeron_rust_wrapper::aeron::concurrent::atomic_buffer;
// use aeron_rust_wrapper::aeron::concurrent::atomic_buffer::AtomicBuffer;
// use aeron_rust_wrapper::aeron::context;
// use aeron_rust_wrapper::aeron::image;
// use clap::Parser;
// use cxx::CxxString;
// use lazy_static::lazy_static;
// use aeron_rs::concurrent::strategies::{BusySpinIdleStrategy, Strategy};
// use aeron_rs::fragment_assembler::FragmentAssembler;
// use aeron_rs::utils::rate_reporter::RateReporter;
// use aeron_rs::utils::types::Index;
//
// lazy_static! {
//     pub static ref RUNNING: AtomicBool = AtomicBool::from(true);
//     pub static ref PRINTING_ACTIVE: AtomicBool = AtomicBool::from(true);
//     pub static ref SUBSCRIPTION_ID: AtomicI64 = AtomicI64::new(-1);
//     pub static ref PUBLICATION_ID: AtomicI64 = AtomicI64::new(-1);
// }
//
// fn sig_int_handler() {
//     RUNNING.store(false, Ordering::SeqCst);
// }
//
// #[derive(Parser, Clone, Debug)]
// #[command(name = "Aeron throughput measurement tool")]
// struct CmdOpts {
//     #[arg(short = 'p', long = "dir", default_value = "", help = "Prefix directory for aeron driver")]
//     dir_prefix: String,
//
//     #[arg(short = 'c', long = "channel", default_value = DEFAULT_CHANNEL, help = "Channel")]
//     channel: String,
//
//     #[arg(short = 's', long = "stream", default_value = DEFAULT_STREAM_ID, help = "Stream ID")]
//     stream_id: i32,
//
//     #[arg(short = 'm', long, default_value = DEFAULT_NUMBER_OF_MESSAGES, help = "Number of messages")]
//     number_of_messages: i64,
//
//     #[arg(short = 'L', long, default_value = DEFAULT_MESSAGE_LENGTH, help = "Message length")]
//     message_length: i32,
//
//     #[arg(short = 'l', long, default_value = DEFAULT_LINGER_TIMEOUT_MS, help = "Linger timeout")]
//     linger_timeout_ms: i32,
//
//     #[arg(short = 'f', long, default_value = DEFAULT_FRAGMENT_COUNT_LIMIT, help = "Fragment Count Limit")]
//     fragment_count_limit: i32,
//
//     #[arg(short = 'P', long, help = "Show publication progress")]
//     progress: bool,
// }
//
// fn print_rate(messages_per_sec: f64, bytes_per_sec: f64, total_fragments: u64, total_bytes: u64) {
//     if PRINTING_ACTIVE.load(Ordering::SeqCst) {
//         println!(
//             "{messages_per_sec:.4} msgs/sec, {bytes_per_sec:.4} bytes/sec, totals {total_fragments} messages {} MB payloads\n",
//             total_bytes / (1024 * 1024)
//         );
//     }
// }
//
// fn on_new_subscription_handler(channel: &CxxString, stream_id: i32, correlation_id: i64) {
//     println!("Subscription: {} {stream_id} {correlation_id}", channel.to_str().unwrap());
// }
//
// fn on_new_publication_handler(channel: &CxxString, stream_id: i32, session_id: i32, correlation_id: i64) {
//     println!(
//         "Publication: {} {stream_id} {session_id} {correlation_id}",
//         channel.to_str().unwrap(),
//     );
// }
//
// fn available_image_handler(image: Pin<&mut image::ffi::Image>) {
//     println!(
//         "Available image correlation_id={} session_id={} at position={} from {}",
//         image.correlation_id(),
//         image.session_id(),
//         image.position(),
//         image.source_identity().to_str().unwrap()
//     );
// }
//
// fn unavailable_image_handler(image: Pin<&mut image::ffi::Image>) {
//     println!(
//         "Unavailable image correlation_id={} session_id={} at position={} from {}",
//         image.correlation_id(),
//         image.session_id(),
//         image.position(),
//         image.source_identity().to_str().unwrap()
//     );
// }
//
// // fn error_handler(error: AeronError) {
// //     println!("Error: {error:?}");
// // }
//
// fn str_to_c(val: &str) -> CString {
//     CString::new(val).expect("Error converting str to CString")
// }
//
// fn main() {
//     pretty_env_logger::init();
//     ctrlc::set_handler(move || {
//         println!("received Ctrl+C!");
//         sig_int_handler();
//     })
//         .expect("Error setting Ctrl-C handler");
//
//     let settings = CmdOpts::parse();
//
//     println!(
//         "Subscribing to channel {} on Stream ID {}",
//         settings.channel, settings.stream_id
//     );
//
//     println!(
//         "Streaming {} messages of payload length {} bytes to {} on stream ID {}",
//         settings.number_of_messages, settings.message_length, settings.channel, settings.stream_id
//     );
//
//     let mut context = context::Context::new_instance("test");
//
//     if !settings.dir_prefix.is_empty() {
//         context.set_aeron_dir(&settings.dir_prefix);
//     }
//
//     println!("Using CnC file: {}", context.cnc_file_name());
//     println!("client name: {}",context.client_name());
//
//     context.new_subscription_handler(on_new_subscription_handler);
//     context.new_publication_handler(on_new_publication_handler);
//     context.available_image_handler(available_image_handler);
//     context.unavailable_image_handler(unavailable_image_handler);
//     // context.set_error_handler(Box::new(error_handler));
//     context.set_pre_touch_mapped_memory(true);
//     //context.set_use_conductor_agent_invoker(true); // start it in one thread for debugging
//
//
//     println!("setting aeron");
//     let mut aeron = Aeron::connect_with_context(context.as_mut());
//
//
//     let subscription_id = aeron
//         .add_subscription(settings.channel.clone(), settings.stream_id)
//         .expect("Error adding subscription");
//     let publication_id = aeron
//         .add_publication(settings.channel.clone(), settings.stream_id)
//         .expect("Error adding publication");
//
//     SUBSCRIPTION_ID.store(subscription_id, Ordering::SeqCst);
//     PUBLICATION_ID.store(publication_id, Ordering::SeqCst);
//
//     let mut subscription = aeron.find_subscription(subscription_id);
//     while subscription.is_null() || !subscription.is_connected() {
//         std::thread::yield_now();
//         subscription = aeron.find_subscription(subscription_id);
//     }
//
//     let mut publication = aeron.find_publication(publication_id);
//     while publication.is_null()|| !publication.is_connected() {
//         thread::yield_now();
//         publication = aeron.find_publication(publication_id);
//     }
//
//
//     let mut offer_idle_strategy = BusySpinIdleStrategy::default();
//     let mut poll_idle_strategy = BusySpinIdleStrategy::default();
//
//     let rate_reporter = Arc::new(Mutex::new(RateReporter::new(1_000_000, print_rate)));
//
//     let rate_reporter_thread: Option<thread::JoinHandle<()>> = None;
//
//     if settings.progress {
//         /*
//         rate_reporter_thread = Some(thread::Builder::new().name(String::from("Reporter thread")).spawn(move || {
//             rate_reporter.run();
//         }).expect("Can't start reporter thread"));
//         */
//     }
//
//     let rate_reporter_for_poll_thread = rate_reporter.clone();
//     let fragment_count_limit = settings.fragment_count_limit;
//     let poll_thread = thread::Builder::new()
//         .name(String::from("Poll thread"))
//         .spawn(move || {
//             let mut rate_reporter_handler = move |_buffer: &AtomicBuffer, _offset: Index, length: Index, _header: &Header| {
//                 let mut reporter = rate_reporter_for_poll_thread.lock().unwrap();
//                 reporter.on_message(1, length as u64);
//             };
//
//             let mut fragment_assembler = FragmentAssembler::new(&mut rate_reporter_handler, None);
//             let mut fragment_handler = fragment_assembler.handler();
//
//             while RUNNING.load(Ordering::SeqCst) {
//                 let fragments_read = subscription.lock().unwrap().poll(&mut fragment_handler, fragment_count_limit);
//
//                 poll_idle_strategy.idle_opt(fragments_read);
//             }
//         })
//         .expect("Can't start poll thread");
//
//     while RUNNING.load(Ordering::SeqCst) {
//         let mut buffer_claim = BufferClaim::default();
//         let mut back_pressure_count = 0;
//
//         PRINTING_ACTIVE.store(true, Ordering::SeqCst);
//
//         if rate_reporter_thread.is_none() {
//             rate_reporter.lock().unwrap().reset();
//         }
//
//         for i in 0..settings.number_of_messages {
//             if !RUNNING.load(Ordering::SeqCst) {
//                 break;
//             }
//
//             offer_idle_strategy.reset();
//
//             while let Err(AeronError::BackPressured) = publication
//                 .lock()
//                 .unwrap()
//                 .try_claim(settings.message_length, &mut buffer_claim)
//             {
//                 back_pressure_count += 1;
//                 offer_idle_strategy.idle();
//             }
//
//             buffer_claim.buffer().put::<i64>(buffer_claim.offset(), i);
//             buffer_claim.commit();
//         }
//
//         if rate_reporter_thread.is_none() {
//             // Don't have dedicated reporting thread thus report here
//             rate_reporter.lock().unwrap().report();
//         }
//
//         println!(
//             "Done streaming. Back pressure ratio {}",
//             back_pressure_count / settings.number_of_messages
//         );
//
//         if RUNNING.load(Ordering::SeqCst) && settings.linger_timeout_ms > 0 {
//             println!("Lingering for {} milliseconds.", settings.linger_timeout_ms);
//             thread::sleep(Duration::from_millis(settings.linger_timeout_ms as u64));
//         }
//
//         PRINTING_ACTIVE.store(false, Ordering::SeqCst);
//     }
//
//     RUNNING.store(false, Ordering::SeqCst);
//
//     rate_reporter.lock().unwrap().halt();
//     poll_thread.join().ok();
//
//     if let Some(handle) = rate_reporter_thread {
//         handle.join().ok();
//     }
// }
