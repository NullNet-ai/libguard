const DATASTORE_PROTOBUF_PATH: &str = "./proto/datastore.proto";
const PROTOBUF_DIR_PATH: &str = "./proto";

fn main() {
    tonic_build::configure()
        .out_dir("./src")
        .compile_protos(&[DATASTORE_PROTOBUF_PATH], &[PROTOBUF_DIR_PATH])
        .expect("Protobuf files generation failed");
}
