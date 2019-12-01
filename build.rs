use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    error::Error,
};

const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/webapp/dist");

fn main() -> Result<(), Box<dyn Error>> {
    let dest = Path::new(&env::var("OUT_DIR")?).join("assets.rs");
    let mut fh = File::create(dest)?;
    writeln!(&mut fh, "{{")?;
    writeln!(&mut fh, "  let mut assets = std::collections::HashMap::new();")?;
    for asset in fs::read_dir(ASSETS_DIR)? {
        let asset = asset?;
        if asset.file_type()?.is_file() {
            writeln!(&mut fh,
                     r#"  assets.insert("/{name}", include_bytes!("{path}").to_vec()); "#,
                     name = asset.file_name().to_string_lossy(),
                     path = asset.path().display()
                     )?;
        }
    }
    writeln!(&mut fh, "  assets")?;
    writeln!(&mut fh, "}};")?;
    Ok(())
}
