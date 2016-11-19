extern crate termimage_bootloader_generator;

use std::process::exit;


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let opts = termimage_bootloader_generator::Options::parse();
    println!("{:#?}", opts);

    Ok(())
}
