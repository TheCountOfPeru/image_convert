extern crate image;
use image::{GenericImageView};

pub fn handle_args(input: &str, output: &str){
    println!("Reading file!");
    println!("Input file: {}", input);
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(input).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img.save(output).unwrap();
}