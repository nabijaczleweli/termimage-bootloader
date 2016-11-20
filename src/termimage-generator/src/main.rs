extern crate termimage_bootloader_generator;
extern crate termimage;

use std::process::exit;
use std::io::Write;
use std::fs::File;


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let opts = termimage_bootloader_generator::Options::parse();
    println!("{:#?}", opts);

    let fmt = try!(termimage::ops::guess_format(&opts.image_file).map_err(|e| e.exit_value()));
    let img = try!(termimage::ops::load_image(&opts.image_file, fmt).map_err(|e| e.exit_value()));
    let img = termimage_bootloader_generator::ops::resize(&img);

    let clr_tbl = termimage_bootloader_generator::ops::create_colourtable(&img, termimage_bootloader_generator::util::VGA_COLOURS);
    File::create(opts.out_file.1).map_err(|_| 3)?
        .write_all(&clr_tbl.into_iter().flat_map(|v| v.into_iter()).map(|c| c as u8).collect::<Vec<_>>())
        .map_err(|_| 3)?;

    Ok(())
}
