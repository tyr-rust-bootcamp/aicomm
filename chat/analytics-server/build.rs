use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["../../protos/messages.proto"], &["../../protos"])
        .unwrap();
    Ok(())
}
