const PROTO_DIR: &str = "proto";
const PROTOS: &[&str] = &[
    "proto/geo.proto",
    "proto/rate.proto",
    "proto/profile.proto",
    "proto/search.proto",
    "proto/khudib.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(PROTOS, &[PROTO_DIR])?;
    Ok(())
}
