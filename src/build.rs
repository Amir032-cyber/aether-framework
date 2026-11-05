fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/aether.proto");
    println!("cargo:rerun-if-changed=proto/modules.proto");
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/aether.proto", "proto/modules.proto"], &["proto"])?;
    Ok(())
}
