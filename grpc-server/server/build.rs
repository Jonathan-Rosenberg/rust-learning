fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/diff.proto").unwrap();
    // let proto_file = "../protos/diff.proto";
    // tonic_build::configure()
    //   .build_server(true)
    //   .out_dir(".")
    //   .proto_path("../protos")
    //   .compile(&[proto_file], &["."])
    //   .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

// println!("cargo:rerun-if-changed={}", proto_file);
    Ok(())
}