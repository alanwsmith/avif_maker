use image::ImageReader;
use ravif::*;
use rgb::FromSlice;
use std::{fmt::format, path::PathBuf};
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    // let input_dir = PathBuf::from("/Users/alan/Documents/Neopoligen/alanwsmith.com/images");
    let input_dir =
        PathBuf::from("/Users/alan/Documents/Neopoligen/alanwsmith.com/avif_test_images");
    let output_root = PathBuf::from("/Users/alan/Documents/Neopoligen/alanwsmith.com/cache/images");
    let extensions = vec!["jpg", "png", "jpeg"];
    let max_widths = vec![100, 200];
    let source_files = get_files_with_extensions(&input_dir, &extensions);
    source_files.iter().for_each(|f| {
        let output_base_dir = output_root.join(f.file_stem().unwrap());
        max_widths.iter().for_each(|w| {
            let output_path = output_base_dir.join(format!("{}w.avif", w));
            println!("{}", output_path.display());
        });

        // println!("Input File: {}", f.display());
        // println!("Output Base: {}", output_base_dir.to_string_lossy());

        // println!("{}", f.file_name().unwrap().to_string_lossy());
        // println!("{}", f.file_stem().unwrap().to_string_lossy());
    });
    Ok(())
}

fn get_files_with_extensions(dir: &PathBuf, extensions: &Vec<&str>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| {
            let path = e.as_ref().unwrap().path();
            match path.extension() {
                Some(ext) => {
                    if extensions.contains(&ext.to_str().unwrap()) {
                        Some(path.to_path_buf())
                    } else {
                        None
                    }
                }
                None => None,
            }
        })
        .collect()
}

fn make_avif(input_path: &PathBuf, output_path: &PathBuf) -> anyhow::Result<()> {
    let img_file = ImageReader::open(input_path)?.decode()?;
    let img = Img::new(
        img_file.as_bytes().as_rgba(),
        img_file.width() as usize,
        img_file.height() as usize,
    );
    let res = Encoder::new()
        .with_quality(70.)
        .with_speed(4)
        .encode_rgba(img)?;
    std::fs::write(output_path, res.avif_file)?;
    Ok(())
}
