fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/") // you can change the generated code's location
        .compile(
            &["protos/zkp_protocol.proto"],
            &["protos/"], // specify the root location to search proto dependencies
        )
        .unwrap();
}
