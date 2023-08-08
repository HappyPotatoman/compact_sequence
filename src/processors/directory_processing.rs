use std::fs;
use std::path::{Path, PathBuf};
use std::io::Result as IoResult;
use walkdir::WalkDir;
use rayon::prelude::*;

use crate::Mode;

pub fn compress_directory(input_dir: &str, output_dir: &str, mode: &Mode) -> IoResult<()> {
    let input_path = Path::new(input_dir);
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
        if let Err(err) = crate::compress_to_file(file.to_str().unwrap(), output_file_path.to_str().unwrap(), mode) {
            eprintln!("Error processing file: {}: {}", file.display(), err);
        }
    });

    Ok(())
}

pub fn unpack_directory(input_dir: &str, output_dir: &str, mode: &Mode) -> std::io::Result<()> {
    let input_path = Path::new(input_dir);
    std::fs::create_dir_all(&output_dir)?;

    let files: Vec<PathBuf> = WalkDir::new(input_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_owned())
        .collect();

    files.par_iter().for_each(|file| {
        let file_stem = file.file_stem().and_then(|f| f.to_str()).unwrap_or("output");
        let format = file.extension().and_then(|ext| ext.to_str()).unwrap_or("txt");
        let output_file_path = Path::new(&output_dir).join(format!("{}_unpacked.{}", file_stem, format));
        if let Err(err) = crate::unpack_from_file(file.to_str().unwrap(), output_file_path.to_str().unwrap(), mode) {
            eprintln!("Error processing file: {}: {}", file.display(), err);
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    const SAMPLE_DNA_SEQUENCE: &str  = "AAGGCCTTNN";
    const SAMPLE_RNA_SEQUENCE: &str  = "AAGGCCUUNN";
    
    #[test]
    fn test_dna_compress_and_unpack_directory() -> std::io::Result<()> {
        const TEST_MODE: Mode = Mode::DNA;
        let temp_input_dir = tempfile::tempdir()?;
        let input_dir_str = temp_input_dir.path().to_str().unwrap();

        let temp_output_dir_str = format!("{}_outputs", input_dir_str);
        let temp_unpacked_dir_str = format!("{}_unpacked_outputs", input_dir_str);

        let file_path = temp_input_dir.path().join("sample.txt");
        let mut file = File::create(&file_path)?;
        writeln!(file, "{}", SAMPLE_DNA_SEQUENCE)?;

        compress_directory(input_dir_str, &temp_output_dir_str, &TEST_MODE)?;

        assert!(Path::new(&temp_output_dir_str).join("sample_output.txt").exists());

        unpack_directory(&temp_output_dir_str, &temp_unpacked_dir_str, &TEST_MODE)?;

        let unpacked_file_path = Path::new(&temp_unpacked_dir_str).join("sample_output_unpacked.txt");
        assert!(unpacked_file_path.exists());
        let unpacked_content = std::fs::read_to_string(unpacked_file_path)?;
        assert_eq!(unpacked_content.trim(), SAMPLE_DNA_SEQUENCE);

        Ok(())
    }


    #[test]
    fn test_rna_compress_and_unpack_directory() -> std::io::Result<()> {
        const TEST_MODE: Mode = Mode::RNA;
        let temp_input_dir = tempfile::tempdir()?;
        let input_dir_str = temp_input_dir.path().to_str().unwrap();

        let temp_output_dir_str = format!("{}_outputs", input_dir_str);
        let temp_unpacked_dir_str = format!("{}_unpacked_outputs", input_dir_str);

        let file_path = temp_input_dir.path().join("sample.txt");
        let mut file = File::create(&file_path)?;
        writeln!(file, "{}", SAMPLE_RNA_SEQUENCE)?;

        compress_directory(input_dir_str, &temp_output_dir_str, &TEST_MODE)?;

        assert!(Path::new(&temp_output_dir_str).join("sample_output.txt").exists());

        unpack_directory(&temp_output_dir_str, &temp_unpacked_dir_str, &TEST_MODE)?;

        let unpacked_file_path = Path::new(&temp_unpacked_dir_str).join("sample_output_unpacked.txt");
        assert!(unpacked_file_path.exists());
        let unpacked_content = std::fs::read_to_string(unpacked_file_path)?;
        assert_eq!(unpacked_content.trim(), SAMPLE_RNA_SEQUENCE);

        Ok(())
    }

}
