// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
// https://lib.rs/install/image
// https://docs.rs/clap/2.33.3/clap/index.html
extern crate clap;
//extern crate image;
use clap::{Arg, App};
//use image::{GenericImageView};

use image_convert;

fn main() {
    // // Use the open function to load an image from a Path.
    // // `open` returns a `DynamicImage` on success.
    // let img = image::open("sample_5184×3456.bmp").unwrap();

    // // The dimensions method returns the images width and height.
    // println!("dimensions {:?}", img.dimensions());

    // // The color method returns the image's `ColorType`.
    // println!("{:?}", img.color());

    // // Write the contents of this image to the Writer in PNG format.
    // img.save("test.png").unwrap();
    let matches = App::new("image_convert")
                          .version("0.1.0")
                          .author("Jonathan Yee <jonathan.yee16@gmail.com>")
                          .about("Does awesome things")
                        //   .arg(Arg::with_name("config")
                        //        .short("c")
                        //        .long("config")
                        //        .value_name("FILE")
                        //        .help("Sets a custom config file")
                        //        .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("OUTPUT")
                               .help("Sets the output file")
                               .required(true)
                               .index(2))
                        //   .arg(Arg::with_name("v")
                        //        .short("v")
                        //        .multiple(true)
                        //        .help("Sets the level of verbosity"))
                        //   .subcommand(SubCommand::with_name("test")
                        //               .about("controls testing features")
                        //               .version("1.3")
                        //               .author("Someone E. <someone_else@other.com>")
                        //               .arg(Arg::with_name("debug")
                        //                   .short("d")
                        //                   .help("print debug information verbosely")))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let input = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", input);

    let output = matches.value_of("OUTPUT").unwrap();
    println!("Using output file: {}", output);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    // more program logic goes here...
    image_convert::handle_args(&input, &output);
}