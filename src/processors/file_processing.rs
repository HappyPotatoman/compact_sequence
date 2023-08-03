use std::fs;
use std::path::{Path, PathBuf};
use std::io::Result as IoResult;
use walkdir::WalkDir;
use rayon::prelude::*;

pub fn process_directory(input_dir: &str) -> IoResult<()> {
    let input_path = Path::new(input_dir);
    let output_dir = format!("{}_outputs", input_path.display());
    fs::create_dir_all(&output_dir)?;

    let files: Vec<PathBuf> = WalkDir::new(input_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_owned())
        .collect();

    files.par_iter().for_each(|file| {
        let file_stem = file.file_stem().and_then(|f| f.to_str()).unwrap_or("output");
        let format = file.extension().and_then(|ext| ext.to_str()).unwrap_or("txt");
        let output_file_path = Path::new(&output_dir).join(format!("{}_output.{}", file_stem, format));
        if let Err(err) = crate::compress_to_file(file.to_str().unwrap(), output_file_path.to_str().unwrap()) {
            eprintln!("Error processing file: {}: {}", file.display(), err);
        }
    });

    Ok(())
}