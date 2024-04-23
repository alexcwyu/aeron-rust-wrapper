fn main() {
    // source files
    let rust_cxx_wrapper_dir = "cxx_wrapper";
    let client_dir = "aeron/aeron-client/src/main/cpp";
    let aeron_client_path = std::path::PathBuf::from(client_dir);
    let aeron_generated_path = std::path::PathBuf::from("aeron/cppbuild/Release/generated");
    let aeron_lib_path = std::path::PathBuf::from("aeron/cppbuild/Release/lib");

    let aeron_client_cpps = [
        "Aeron.h",
        "Context.h",
        "ExclusivePublication.h",
        "Image.h",
        "LogBuffers.h",
        "LogBuffers.h",
        "Publication.h",
        "Subscription.h",
        "concurrent/AtomicBuffer.h",
        "concurrent/logbuffer/Header.h",
        "concurrent/logbuffer/BufferClaim.h",
        "concurrent/logbuffer/FrameDescriptor.h",
    ].into_iter().map(|src|std::path::PathBuf::from(client_dir.to_owned() + "/" + src));

    let aeron_wrapper_cpps = [
        "Aeron.cpp",
        "Context.cpp",
        "ExclusivePublication.cpp",
        "Image.cpp",
        "LogBuffers.cpp",
        "LogBuffers.cpp",
        "Publication.cpp",
        "Subscription.cpp",
        "concurrent/AtomicBuffer.cpp",
        "concurrent/logbuffer/Header.cpp",
        "concurrent/logbuffer/BufferClaim.cpp",
    ].into_iter().map(|src|std::path::PathBuf::from(rust_cxx_wrapper_dir.to_owned() + "/" + src));


    let bridge_files = [
        "src/aeron/aeron.rs",
        "src/aeron/context.rs",
        "src/aeron/exclusive_publication.rs",
        "src/aeron/image.rs",
        "src/aeron/log_buffers.rs",
        "src/aeron/publication.rs",
        "src/aeron/subscription.rs",
        "src/aeron/concurrent/atomic_buffer.rs",
        "src/aeron/concurrent/logbuffer/header.rs",
        "src/aeron/concurrent/logbuffer/buffer_claim.rs",
    ].into_iter().map(|src|std::path::PathBuf::from(src));

    cxx_build::bridges(bridge_files.clone())
        .includes(&[
            &aeron_client_path,
            &aeron_generated_path,
            &aeron_lib_path,
        ])
        .files(aeron_client_cpps)
        .files(aeron_wrapper_cpps.clone())
        .std("c++14")
        .compile("aeron-rust-wrapper");

    bridge_files.for_each(|src| {
        println!("cargo:rerun-if-changed={}", src.to_str().unwrap());
    });

    aeron_wrapper_cpps.for_each(|src| {
        println!("cargo:rerun-if-changed={}", src.to_str().unwrap());
    });
    println!("cargo:rerun-if-changed=aeron/aeron-client/src/main/cpp");
    println!("cargo:rerun-if-changed=aeron/cppbuild/Release/generated");
    println!("cargo:rerun-if-changed=aeron/cppbuild/Release/lib");
}
