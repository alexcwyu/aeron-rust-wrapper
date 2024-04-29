
#[cxx::bridge(namespace = "aeron::concurrent")]
pub mod ffi {

    unsafe extern "C++" {
        #[namespace = "aeron::concurrent"]
        type CxxAtomicBuffer = crate::aeron::concurrent::atomic_buffer::ffi::CxxAtomicBuffer;


        include!("aeron-rust-wrapper/aeron/aeron-client/src/main/cpp/concurrent/AgentRunner.h");

        #[rust_name = "CxxAgentRunner"]
        type AgentRunner;

        // fn name(self: &AgentRunner) -> &CxxString;
        //
        // #[rust_name = "is_started"]
        // fn isStarted(self: &AgentRunner) -> bool;
        //
        // #[rust_name = "is_running"]
        // fn isRunning(self: &AgentRunner) -> bool;
        //
        // #[rust_name = "is_closed"]
        // fn isClosed(self: &AgentRunner) -> bool;
        //
        // fn start(self: &AgentRunner) -> bool;
        //
        // fn run(self: &AgentRunner) -> bool;
        //
        // fn close(self: &AgentRunner) -> bool;

    }

    // impl SharedPtr<AgentRunner> {}
}