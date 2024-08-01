use std::{env, error::Error, fs::File, io::BufReader, path::Path};

use image::{codecs::png::PngDecoder, DynamicImage, GenericImage, ImageResult};

pub const CLIENT_PATH: &str = env!("CLIENT_PATH");

fn convert_terrain(terrain_png: File) -> ImageResult<DynamicImage> {
    let mc_terrain = PngDecoder::new(BufReader::new(terrain_png))?;
    let mc_terrain = DynamicImage::from_decoder(mc_terrain)?;

    let mut terrain = DynamicImage::new_rgba8(64, 64);

    terrain.copy_from(&mc_terrain.crop_imm(0, 0, 16, 16), 0, 0)?; // grass

    Ok(terrain)
}

pub fn create_textures(output_path: &Path) -> Result<(), Box<dyn Error>> {
    let client_path = Path::new(CLIENT_PATH);

    convert_terrain(File::open(client_path.join("terrain.png"))?)?
        .save(output_path.join("terrain.png"))?;

    Ok(())
}
