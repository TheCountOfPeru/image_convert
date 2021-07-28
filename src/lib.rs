// use std::env;
// pub struct Config {
//     pub file_extension: String,
//     pub filename: String,
// }
// impl Config {
//     pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
//         args.next();

//         let file_extension = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a file extension string"),
//         };

//         let filename = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a file name"),
//         };

//         Ok(Config {
//             file_extension,
//             filename,
//         })
//     }
// }