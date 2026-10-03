fn main() {
    reboot::build::compile_protos_with_runtime(
        &["proto/family_tasks/v1/task.proto"],
        &["proto"],
        "crate::proto",
        "reboot",
    )
    .expect("Family Tasks protobuf and Reboot adapter generation must succeed");
}
