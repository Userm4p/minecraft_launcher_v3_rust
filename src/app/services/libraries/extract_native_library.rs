use std::{
    fs::{self, File},
    io,
    path::Path,
};

use anyhow::Result;
use zip::ZipArchive;

pub fn extract_native_library(jar_path: &Path, natives_dir: &Path) -> Result<()> {
    let file = File::open(jar_path)?;

    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;

        let name = entry.name().to_string();

        if name.starts_with("META-INF") {
            continue;
        }

        let outpath = natives_dir.join(&name);

        if entry.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut outfile = File::create(&outpath)?;

            io::copy(&mut entry, &mut outfile)?;
        }
    }

    Ok(())
}
