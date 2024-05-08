fn main() {
    prost_build::compile_protos(&["message.proto"], &[""]).expect("failed to compile");
}