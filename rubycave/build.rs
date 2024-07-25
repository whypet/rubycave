use std::{error::Error, path::Path};

use glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
    let src_path = Path::new("src");
    let capnp_path = &src_path.join("capnp");

    println!("cargo::rerun-if-changed={}", capnp_path.to_str().unwrap());

    let mut capnp_command = capnpc::CompilerCommand::new();
    let mut capnp_command = capnp_command
        .src_prefix(&src_path)
        .default_parent_module(vec!["protocol".into()]);

    for entry in glob(src_path.join("**").join("*.capnp").to_str().unwrap())? {
        let entry = entry?;

        println!("cargo::rerun-if-changed={}", entry.to_str().unwrap());

        capnp_command = capnp_command.file(entry);
    }

    capnp_command.run()?;

    Ok(())
}
