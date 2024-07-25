use std::{error::Error, path::Path};

use glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
    let src_path = Path::new("src");

    let mut capnp_command = capnpc::CompilerCommand::new();
    let mut capnp_command = capnp_command.src_prefix(&src_path);

    for entry in glob(src_path.join("**").join("*.capnp").to_str().unwrap())? {
        let entry = entry?;

        capnp_command = capnp_command.file(entry);
    }

    capnp_command.run()?;

    Ok(())
}
