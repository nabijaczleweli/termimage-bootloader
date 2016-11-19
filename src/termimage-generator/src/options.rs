//! Option parsing and management.
//!
//! Use the `Options::parse()` function to get the program's configuration,
//! as parsed from the commandline.
//!
//! # Examples
//!
//! ```no_run
//! # use termimage_bootloader_generator::Options;
//! let opts = Options::parse();
//! println!("{:#?}", opts);
//! ```


use self::super::ops::fname_to_8_3;
use clap::{AppSettings, App, Arg};
use std::path::PathBuf;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// The image file to convert.
    pub image_file: (String, PathBuf),
    /// The output file to convert. Default: 8.3-truncated `image_file` with `.BLI` extension
    pub out_file: (String, PathBuf),
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = App::new("termimage-bootloader-generator")
            .settings(&[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])
            .version(crate_version!())
            .author(crate_authors!())
            .about("Generator to display a textified image from a floppy, old-school style")
            .args(&[Arg::from_usage("<FILE> 'The image file to convert'")
                        .validator(Options::in_file_validator)
                        .required(true),
                    Arg::from_usage("-o [OUT_FILE] 'The output file. Default: truncated input file + .bli'")])
            .get_matches();

        Options {
            image_file: (matches.value_of("FILE").unwrap().to_string(), fs::canonicalize(matches.value_of("FILE").unwrap()).unwrap()),
            out_file: match matches.value_of("OUT_FILE") {
                Some(of) => (of.to_string(), fs::canonicalize(of).unwrap()),
                None => {
                    let mut pb = fs::canonicalize(matches.value_of("FILE").unwrap()).unwrap();
                    pb.set_extension("");
                    let orig_fname = pb.file_name().unwrap().to_str().unwrap().to_string();
                    pb.set_file_name(fname_to_8_3(&orig_fname));
                    pb.set_extension("BLI");

                    ({
                        let mut f = PathBuf::from(matches.value_of("FILE").unwrap());
                        f.set_file_name(pb.file_name().unwrap());
                        f.to_str().unwrap().to_string()
                    },
                     pb)
                }
            },
        }
    }

    fn in_file_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map_err(|_| format!("Image to convert \"{}\" not found", s)).and_then(|pb| if pb.is_file() {
            Ok(())
        } else {
            Err(format!("Image to convert \"{}\" is not a file", s))
        })
    }
}
