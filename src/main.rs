use image::ImageReader;
use ravif::*;
use rgb::FromSlice;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    let debug = true;
    let input_dir = if debug {
        PathBuf::from("/Users/alan/Documents/Neopoligen/alanwsmith.com/avif_test_images")
    } else {
        PathBuf::from("/Users/alan/Documents/Neopoligen/alanwsmith.com/images")
    };
    let output_root = PathBuf::from("/Users/alan/Documents/Neopoligen/alanwsmith.com/cache/images");
    let extensions = vec!["jpg", "png", "jpeg"];
    let max_widths = vec![100, 200, 300, 400];
    let source_files = get_files_with_extensions(&input_dir, &extensions);
    source_files.iter().for_each(|f| {
        let output_base_dir = output_root.join(f.file_stem().unwrap());
        max_widths.iter().for_each(|max_width| {
            let _ = make_avif(&f, &output_base_dir, *max_width);
            // let output_path = output_base_dir.join(format!("{}w.avif", w));
            // println!("{}", output_path.display());
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

fn make_avif(
    input_path: &PathBuf,
    output_base_dir: &PathBuf,
    max_width: u32,
) -> anyhow::Result<()> {
    match fs::create_dir_all(output_base_dir) {
        Ok(_) => {
            let img_file = ImageReader::open(input_path)?.decode()?;
            let output_width = if img_file.width() > max_width {
                max_width as u32
            } else {
                img_file.width()
            };
            let output_height = img_file.height() * output_width / img_file.width();
            let output_path = output_base_dir.join(format!("{}w.avif", output_width));

            if !file_exists(&output_path) {
                println!("Making: {}", output_path.display());

                let resized_image = img_file.resize_to_fill(
                    output_width,
                    output_height,
                    image::imageops::FilterType::Lanczos3,
                );
                let img = Img::new(
                    resized_image.as_bytes().as_rgb(),
                    resized_image.width() as usize,
                    resized_image.height() as usize,
                );
                let res = Encoder::new()
                    .with_quality(70.)
                    .with_speed(4)
                    .encode_rgb(img)?;
                std::fs::write(output_path, res.avif_file)?;
            } else {
                println!("Exists: {}", output_path.display());
            }
        }
        Err(e) => {
            dbg!(e);
            ()
        }
    }

    // let img_file = ImageReader::open(input_path)?.decode()?;
    // let img = Img::new(
    //     img_file.as_bytes().as_rgba(),
    //     img_file.width() as usize,
    //     img_file.height() as usize,
    // );
    // let res = Encoder::new()
    //     .with_quality(70.)
    //     .with_speed(4)
    //     .encode_rgba(img)?;
    // std::fs::write(output_path, res.avif_file)?;

    Ok(())
}

fn file_exists(path: &PathBuf) -> bool {
    match path.try_exists() {
        Ok(exists) => {
            if exists == true {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
