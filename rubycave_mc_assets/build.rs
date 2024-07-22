use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, BufReader},
    path::Path,
};

use image::{codecs::png::PngDecoder, DynamicImage, GenericImage, ImageResult};
use zip::ZipArchive;

const MC_VERSION_JSON_URL: &str = "https://meta.prismlauncher.org/v1/net.minecraft/a1.1.2_01.json";

fn convert_terrain(terrain_png: File) -> ImageResult<DynamicImage> {
    let mc_terrain = PngDecoder::new(BufReader::new(terrain_png))?;
    let mc_terrain = DynamicImage::from_decoder(mc_terrain)?;

    let mut terrain = DynamicImage::new_rgba8(64, 64);

    terrain.copy_from(&mc_terrain.crop_imm(0, 0, 16, 16), 0, 0)?; // grass

    Ok(terrain)
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    let client_dir = out_dir.join("../../minecraft_client");

    if !client_dir.is_dir() {
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
        zip.extract(&client_dir)?;
    }

    let res_dir = out_dir.join("../../../res");
    let tex_dir = res_dir.join("texture");

    if !tex_dir.is_dir() {
        fs::create_dir_all(&tex_dir)?;
    }

    convert_terrain(File::open(&client_dir.join("terrain.png"))?)?
        .save(tex_dir.join("terrain.png"))?;

    Ok(())
}
