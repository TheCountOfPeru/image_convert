extern crate image;
use std::process;

pub fn handle_args(input: &str, output: &str){
    println!("Input file: {}", input);
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(input).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    // Write the contents of this image to the Writer in PNG format.
    img.save(output).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}