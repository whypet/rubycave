use glob::glob;
use std::{env, fs, path::PathBuf};

fn main() {
    let mut target_dir: PathBuf = env::var("OUT_DIR").unwrap().into();

    // Get the target directory (currently no environment variable for this)
    target_dir.pop();
    target_dir.pop();
    target_dir.pop();

    let mut res_dir = env::current_dir().expect("failed to get current directory");
    res_dir.push("res");

    // let cond = out_dir.ends_with('/') || out_dir.ends_with(path::MAIN_SEPARATOR);
    // let pattern = &(out_dir + if cond { "**/*" } else { "/**/*" });

    let mut pattern = res_dir.clone();
    pattern.push("**/*");

    for entry in glob(pattern.to_str().unwrap())
        .expect(&format!("glob failed for pattern: '{}'", pattern.display()))
    {
        let entry_path = entry.expect("entry failure");

        if !entry_path.is_file() {
            continue;
        }

        let stripped = entry_path.strip_prefix(&res_dir).expect(&format!(
            "failed to strip prefix '{}' from '{}'",
            entry_path.display(),
            res_dir.display()
        ));

        let mut new_path = target_dir.clone();
        new_path.push(stripped);

        let parent = new_path.parent().unwrap();

        fs::create_dir_all(parent).expect(&format!(
            "failed to recursively create directories for: '{}'",
            parent.display()
        ));
        fs::copy(&entry_path, &new_path).expect(&format!(
            "failed to copy file '{}' to '{}'",
            entry_path.display(),
            new_path.display()
        ));
    }
}
