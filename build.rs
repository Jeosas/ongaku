use std::io::Result;

fn main() -> Result<()> {
    // Compile protobuf interface
    prost_build::compile_protos(&["src/db.proto"], &["src/"])?;
    Ok(())
}
