use glob::{glob, GlobError};
use std::{
    env,
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

fn create_dir(path: &Path, env_name: &str) -> io::Result<()> {
    if !path.is_dir() {
        fs::create_dir(path)?;
    }

    println!("cargo:rustc-env={}={}", env_name, path.to_str().unwrap());

    Ok(())
}

fn copy(in_dir: &Path, out_dir: &Path) -> Result<(), GlobError> {
    let mut pattern = PathBuf::from(in_dir);
    pattern.push("**/*");

    for entry in glob(pattern.to_str().unwrap()).expect("glob failed") {
        let entry = entry?;

        if !entry.is_file() {
            continue;
        }

        println!("cargo::rerun-if-changed={}", entry.to_str().unwrap());

        let stripped = entry.strip_prefix(in_dir).expect("failed to strip prefix");

        let mut new_path = PathBuf::from(out_dir);
        new_path.push(stripped);

        let parent = new_path.parent().unwrap();

        fs::create_dir_all(parent).expect("failed to recursively create directories");
        fs::copy(&entry, &new_path).expect("failed to copy file");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut target_dir: PathBuf = env::var("OUT_DIR").unwrap().into();

    // Get the target directory (currently no environment variable for this)
    target_dir.pop();
    target_dir.pop();
    target_dir.pop();
    target_dir.push("res");

    if !target_dir.is_dir() {
        fs::create_dir(&target_dir)?;
    }

    let texture_dir = &target_dir.join("texture");
    let shader_dir = &target_dir.join("shader");

    create_dir(texture_dir, "TEXTURE_DIR")?;
    create_dir(shader_dir, "SHADER_DIR")?;

    rubycave_mc_assets::create_textures(texture_dir)?;
    copy(&env::current_dir()?.join("res"), &target_dir)?;

    Ok(())
}
