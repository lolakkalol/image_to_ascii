use clap::Parser;
use std::fs;

mod aic;

/// Generates an ascii image of the input image
#[derive(Parser)]
struct Cli {
    /// Input image file
    input: std::path::PathBuf,

    /// Output text file
    output: std::path::PathBuf,

    /// Size of the kernel used when generating ascii image
    kernel_size: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // Open images as luma8 image (grayscale 8 bit values)
    let image = match image::open(args.input) {
        Ok(image) => image,
        Err(error) => {
            eprint!("Error while opening image: {}\n\r", error);
            return Ok(());
        },
    };
    

    let image = image.into_luma8();

    let (width, height) = image.dimensions();
    print!("Image input size in pixels:       W:{} H:{}\n\r", width, height);
    print!("Output image size in characters:  W:{} H:{}\n\r", width/args.kernel_size, height/args.kernel_size);

    // Prints the ascii art into a file, overwriting the file in the process
    let write_result = fs::write(&args.output, 
        // Convert image to a string of ascii chars
        aic::image_to_ascii(&image, &args.kernel_size));
    
    // Handle write errors
    match write_result {
        Ok(_) => (),
        Err(error) => {
            eprint!("An error occured while writing to file: {}\n\rError type: {}\n\r", error, error.kind() );
            return Ok(());
        },
    }

    print!("Image written to \"{}\"\n\r", args.output.to_string_lossy());
    
    Ok(())
}

/*
TODO
[x] Put code into seperate functions
[ ] Fix error handling
[x] Put some of the code in seperate modules
*/