use std::{env, error::Error, fs::File, io, path::Path};

use zip::ZipArchive;

const MC_VERSION_JSON_URL: &str = "https://meta.prismlauncher.org/v1/net.minecraft/a1.1.2_01.json";

fn main() -> Result<(), Box<dyn Error>> {
    // todo!("Download Minecraft client, extract resources and make them accessible from the crate");

    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    let client_path = out_dir.join("../../minecraft_client.jar");

    let client_file = File::open(&client_path);
    let client_file = if client_file.is_err() {
        let json = reqwest::blocking::get(MC_VERSION_JSON_URL)?.text()?;
        let json: serde_json::Value = serde_json::from_str(json.as_str())?;

        let url = json["mainJar"]["downloads"]["artifact"]["url"]
            .as_str()
            .expect("failed to get minecraft client download url");

        let mut client_res = reqwest::blocking::get(url)?;
        let mut client_file = File::create(&client_path)?;

        io::copy(&mut client_res, &mut client_file)?;

        client_file
    } else {
        client_file?
    };

    let mut zip = ZipArchive::new(client_file)?;

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        println!("file: {}", file.name());
    }

    Ok(())
}
