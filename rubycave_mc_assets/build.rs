use std::{env, error::Error, fs::File, io, path::Path};

use zip::ZipArchive;

const MC_VERSION_JSON_URL: &str = "https://meta.prismlauncher.org/v1/net.minecraft/a1.1.2_01.json";

pub fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    let client_jar = out_dir.join("minecraft_client.jar");

    let json = reqwest::blocking::get(MC_VERSION_JSON_URL)?.text()?;
    let json: serde_json::Value = serde_json::from_str(json.as_str())?;

    let url = json["mainJar"]["downloads"]["artifact"]["url"]
        .as_str()
        .expect("failed to get minecraft client download url");

    let mut client_res = reqwest::blocking::get(url)?;
    let mut client_file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&client_jar)?;

    io::copy(&mut client_res, &mut client_file)?;

    let client_dir = out_dir.join("minecraft_client");

    let mut zip = ZipArchive::new(client_file)?;
    zip.extract(&client_dir)?;

    println!(
        "cargo:rustc-env=CLIENT_PATH={}",
        client_dir.to_str().unwrap()
    );

    Ok(())
}
