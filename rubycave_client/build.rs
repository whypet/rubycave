use glob::glob;
use std::{env, error::Error, fs, path::PathBuf};

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

    let mut res_dir = env::current_dir()?;
    res_dir.push("res");

    let mut pattern = res_dir.clone();
    pattern.push("**/*");

    for entry in glob(pattern.to_str().unwrap())
        .expect(&format!("glob failed for pattern: '{}'", pattern.display()))
    {
        let entry = entry?;

        if !entry.is_file() {
            continue;
        }

        println!("cargo::rerun-if-changed={}", entry.to_str().unwrap());

        let stripped = entry.strip_prefix(&res_dir).expect(&format!(
            "failed to strip prefix '{}' from '{}'",
            entry.display(),
            res_dir.display()
        ));

        let mut new_path = target_dir.clone();
        new_path.push(stripped);

        let parent = new_path.parent().unwrap();

        fs::create_dir_all(parent).expect(&format!(
            "failed to recursively create directories for: '{}'",
            parent.display()
        ));
        fs::copy(&entry, &new_path).expect(&format!(
            "failed to copy file '{}' to '{}'",
            entry.display(),
            new_path.display()
        ));
    }

    Ok(())
}
