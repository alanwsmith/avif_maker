use image::ImageReader;
use ravif::*;
use rgb::FromSlice;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    let debug = false;
    let input_dir = if !debug {
        PathBuf::from("/Users/alan/Documents/Neopoligen/alanwsmith.com/images")
    } else {
        PathBuf::from("/Users/alan/Documents/Neopoligen/alanwsmith.com/avif_test_images")
    };
    let max_widths = if !debug {
        // Sizes aren't used yet so just make a single value for now
        // to trigger the build of the fallback image.
        // vec![100, 200, 300, 400, 800, 1200, 1600, 2400, 3200]
        // TODO: Make the different sized images
        vec![2000]
    } else {
        // Sizes aren't used yet so just make a single value for now
        // to trigger the build of the fallback image.
        //vec![100, 200, 300, 2400]
        // TODO: Make the different sized images
        vec![200]
    };
    let output_root = PathBuf::from("/Users/alan/Documents/Neopoligen/alanwsmith.com/cache/images");
    let extensions = vec!["jpg", "png", "jpeg"];
    let source_files = get_files_with_extensions(&input_dir, &extensions);
    source_files.iter().for_each(|f| {
        let output_base_dir = output_root.join(f.file_stem().unwrap());
        max_widths.iter().for_each(|max_width| {
            let _ = make_avif(&f, &output_base_dir, *max_width);
        });
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

fn convert_rgb8(image_data: &image::DynamicImage, output_base_dir: &PathBuf) -> anyhow::Result<()> {
    let output_path = output_base_dir.join(format!(
        "{}.avif",
        output_base_dir.file_stem().unwrap().display()
    ));
    if !file_exists(&output_path) {
        println!("Making Image: {}", output_path.display());
        let img = Img::new(
            image_data.as_bytes().as_rgb(),
            image_data.width() as usize,
            image_data.height() as usize,
        );
        let res = Encoder::new()
            .with_quality(70.)
            .with_speed(4)
            .encode_rgb(img)?;
        std::fs::write(output_path, res.avif_file)?;
    } else {
        println!("Image Already Exists: {}", output_path.display());
    }
    Ok(())
}

fn convert_rgba8(
    image_data: &image::DynamicImage,
    output_base_dir: &PathBuf,
) -> anyhow::Result<()> {
    let output_path = output_base_dir.join(format!(
        "{}.avif",
        output_base_dir.file_stem().unwrap().display()
    ));
    if !file_exists(&output_path) {
        println!("Making Image: {}", output_path.display());
        let img = Img::new(
            image_data.as_bytes().as_rgba(),
            image_data.width() as usize,
            image_data.height() as usize,
        );
        let res = Encoder::new()
            .with_quality(70.)
            .with_speed(4)
            .encode_rgba(img)?;
        std::fs::write(output_path, res.avif_file)?;
    } else {
        println!("Image Already Exists: {}", output_path.display());
    }
    Ok(())
}

fn make_avif(
    input_path: &PathBuf,
    output_base_dir: &PathBuf,
    _max_width: u32,
) -> anyhow::Result<()> {
    match fs::create_dir_all(output_base_dir) {
        Ok(_) => {
            let image_data = ImageReader::open(input_path)?.decode()?;
            match image_data.color() {
                image::ColorType::Rgb8 => convert_rgb8(&image_data, output_base_dir),
                image::ColorType::Rgba8 => convert_rgba8(&image_data, output_base_dir),
                _ => {
                    println!("Hit Currently Unhandled ColorType - See Notes");
                    Ok(())
                }
            }

            // let image_data = ImageReader::open(input_path)?.decode()?.to_rgba8();
            //let image_data = ImageReader::open(input_path)?.decode()?;

            //// NOTE: it's a little inefficient to do the fallback
            //// process every time but it doesn't do anything
            //// if the file already exists so I'm not worried
            //// about it.
            //let fallback_path = output_base_dir.join(format!(
            //    "{}.avif",
            //    output_base_dir.file_stem().unwrap().display()
            //));
            //if !file_exists(&fallback_path) {
            //    println!("Making Fallback: {}", &fallback_path.display());
            //    let img = Img::new(
            //        //image_data.as_bytes().as_rgb(),
            //        image_data.as_raw().as_rgba(),
            //        image_data.width() as usize,
            //        image_data.height() as usize,
            //    );
            //    let res = Encoder::new()
            //        .with_quality(70.)
            //        .with_speed(4)
            //        .encode_rgba(img)?;
            //    std::fs::write(&fallback_path, res.avif_file)?;
            //} else {
            //    println!("Fallback Exists: {}", &fallback_path.display());
            //}

            // let output_width = if img_file.width() > max_width {
            //     max_width as u32
            // } else {
            //     img_file.width()
            // };
            // let output_height = img_file.height() * output_width / img_file.width();
            // let output_path = output_base_dir.join(format!("{}w.avif", output_width));
            // if !file_exists(&output_path) {
            //     println!("Making: {}", output_path.display());
            //     let resized_image = img_file.resize_to_fill(
            //         output_width,
            //         output_height,
            //         image::imageops::FilterType::Lanczos3,
            //     );
            //     let img = Img::new(
            //         resized_image.as_bytes().as_rgb(),
            //         resized_image.width() as usize,
            //         resized_image.height() as usize,
            //     );
            //     let res = Encoder::new()
            //         .with_quality(70.)
            //         .with_speed(4)
            //         .encode_rgb(img)?;
            //     std::fs::write(output_path, res.avif_file)?;
            // } else {
            //     println!("Exists: {}", output_path.display());
            // }
        }
        Err(e) => {
            println!("Error: {}", e);
            // this should be an error at some point.
            Ok(())
        }
    }
    //Ok(())
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
