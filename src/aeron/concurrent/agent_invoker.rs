
#[cxx::bridge(namespace = "aeron::concurrent")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type AtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::AtomicBuffer;


        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/AgentInvoker.h");
        type AgentInvoker;

        // #[rust_name = "is_start"]
        // fn isStart(self: &AgentInvoker) -> bool;
        //
        //
        // #[rust_name = "is_running"]
        // fn isRunning(self: &AgentInvoker) -> bool;
        //
        //
        // #[rust_name = "is_closed"]
        // fn isClosed(self: &AgentInvoker) -> bool;
        //
        //
        // fn start(self: &AgentInvoker) -> bool;
        //
        // fn invoke(self: &AgentInvoker) -> bool;
        //
        // fn close(self: &AgentInvoker) -> bool;
    }

    // impl SharedPtr<AgentInvoker> {}
}