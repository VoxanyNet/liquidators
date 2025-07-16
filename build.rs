use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let asset_dir = Path::new("assets");
    let mut paths = Vec::new();

    fn collect_files(dir: &Path, base: &Path, paths: &mut Vec<String>) {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {

                if let Some(extension) = path.extension() {
                    if extension.eq_ignore_ascii_case("png") || extension.eq_ignore_ascii_case("wav") {
                        let string_path = path.to_string_lossy();
                        paths.push(string_path.into_owned());
                    }
                }
                
            } else if path.is_dir() {
                collect_files(&path, base, paths);
            }
        }
    }

    collect_files(&asset_dir, &asset_dir, &mut paths);

    let contents = format!(
        "pub static ASSET_PATHS: &[&str] = &{:?};",
        paths
    );

    fs::write(Path::new(&out_dir).join("assets.rs"), contents).unwrap();
}
