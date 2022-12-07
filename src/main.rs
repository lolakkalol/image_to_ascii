use clap::Parser;
use image::{self, Luma};
use std::fs;

static ASCII: [(char, f32); 95] = 
[(' ', 0.0), ('`', 3.5*0.013212278995336058), ('_', 3.5*0.01438274111222182), 
('\'', 3.5*0.027089170771462445), ('-', 3.5*0.028414101912004996), 
('.', 3.5*0.030564602964964993), ('^', 3.5*0.04424317056919902), 
('~', 3.5*0.05034429730630052), (',', 3.5*0.05396611162668696), 
('"', 3.5*0.0545383292880236), (':', 3.5*0.061134950415235534), 
('/', 3.5*0.08319721319960016), ('\\', 3.5*0.08319732474300401), 
(';', 3.5*0.08453645907695218), ('!', 3.5*0.09193539331690759), 
('+', 3.5*0.09285661169807202), ('*', 3.5*0.09322381258328583), 
('=', 3.5*0.0946950700789938), ('|', 3.5*0.09920045688178114), 
('>', 3.5*0.10011291910574804), ('<', 3.5*0.10035807291666554), 
('1', 3.5*0.11400994446624724), ('(', 3.5*0.11414095219398501), 
(')', 3.5*0.11485464407243462), ('r', 3.5*0.11685798219469057), 
('j', 3.5*0.13278963360222917), ('v', 3.5*0.13385617444793463), 
(']', 3.5*0.1351765508994801), ('[', 3.5*0.13521280250571277), 
('i', 3.5*0.13699420643558158), ('L', 3.5*0.13918298546662977), 
('{', 3.5*0.14288102111293366), ('}', 3.5*0.14290781012040876), 
('?', 3.5*0.14710890651770225), ('T', 3.5*0.1475681678921551), 
('t', 3.5*0.1487603809727721), ('7', 3.5*0.14877250202265369), 
('l', 3.5*0.1488032322303905), ('f', 3.5*0.15268616965900528), 
('z', 3.5*0.15291587470849388), ('x', 3.5*0.15328854122049262), 
('c', 3.5*0.1542967076848929), ('J', 3.5*0.15545066137802044), 
('Y', 3.5*0.15634272974966357), ('I', 3.5*0.15892246841090185), 
('n', 3.5*0.16644867367455596), ('u', 3.5*0.16697307639681636), 
('y', 3.5*0.17599137918213503), ('s', 3.5*0.17674864735032214), 
('F', 3.5*0.17679586739125008), ('V', 3.5*0.1770127821304497), 
('o', 3.5*0.17930289834380364), ('k', 3.5*0.18639445614410366), 
('4', 3.5*0.18674540887349567), ('e', 3.5*0.19032024483033338), 
('h', 3.5*0.19040170869621087), ('%', 3.5*0.19336467331167906), 
('X', 3.5*0.1946353571768511), ('a', 3.5*0.19627586319721746), 
('w', 3.5*0.19927164016395404), ('Z', 3.5*0.20216042841589954), 
('C', 3.5*0.20333460864624897), ('P', 3.5*0.20352624021392274), 
('A', 3.5*0.21248477432538204), ('2', 3.5*0.21399774905410418), 
('U', 3.5*0.21433007403507198), ('m', 3.5*0.21458727453359605), 
('3', 3.5*0.2161586980058996), ('#', 3.5*0.21648072381257744), 
('5', 3.5*0.216991090656529), ('d', 3.5*0.21866078386754562), 
('b', 3.5*0.21893692815415378), ('q', 3.5*0.2190966768989091), 
('E', 3.5*0.21913937943198347), ('p', 3.5*0.2195357675078542), 
('0', 3.5*0.22511910976465954), ('K', 3.5*0.22512989229368374), 
('H', 3.5*0.22617763807585312), ('S', 3.5*0.2346966539953359), 
('O', 3.5*0.23617650033314264), ('@', 3.5*0.2377632052517612), 
('D', 3.5*0.24438685706381524), ('N', 3.5*0.2469354751154157), 
('R', 3.5*0.24748307886564636), ('g', 3.5*0.24810582568888284), 
('G', 3.5*0.25009049888159474), ('6', 3.5*0.2506571393727405), 
('9', 3.5*0.25093882364839126), ('$', 3.5*0.2510306424602571), 
('&', 3.5*0.2533401672258708), ('8', 3.5*0.2553505139920055), 
('W', 3.5*0.25954644221159395), ('Q', 3.5*0.2602397774188553), 
('B', 3.5*0.27656738931919145), ('M', 3.5*0.2788096905934704)];

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
    let image = image::open(args.input)?.into_luma8();
    let (width, height) = image.dimensions();

    print!("Image input size in pixels:       W:{} H:{}\n\r", width, height);
    print!("Output image size in characters:  W:{} H:{}\n\r", width/args.kernel_size, height/args.kernel_size);

    // Prints the ascii art into a file, overwriting the file in the process
    fs::write(&args.output, 
        // Convert image to a string of ascii chars
        image_to_ascii(&image, &args.kernel_size))?;

    print!("Image written to \"{}\"\n\r", args.output.to_string_lossy());
    
    Ok(())
}

// Will convert the Luma8 image to 
fn image_to_ascii(image: &impl image::GenericImageView<Pixel = Luma<u8>>, kernel_size: &u32) -> String {

    let mut ascii_art: String = String::new();
    let (width, height) = image.dimensions();

    let mut sum: f32;
    for y in 0..height/kernel_size {
        for x in 0..width/kernel_size {
            sum = sum_square(image, kernel_size, (x*kernel_size, y*kernel_size));
            ascii_art.push(ascii_representation(sum));
        }
        ascii_art.push('\n');
    }

    ascii_art
}

// Will find the representation of num in ascii. Num must be between 0 and 1
fn ascii_representation(num: f32) -> char {

    let mut previous_char = ' ';

    // itterate through the lookup table of ascii characters and one closest to the num value
    for (symbol, value) in ASCII {
        if value > num {
            return previous_char;
        }
        previous_char = symbol;
    }

    previous_char
}

fn sum_square(image: &impl image::GenericImageView<Pixel = Luma<u8>>, 
    kernel: &u32, (x, y): (u32, u32)) -> f32{
        let mut sum:f32 = 0.0;
        
        let (width, height) = image.dimensions();

        // Incase the kernel is outside the image, return 0, this will result in
        // some cutting of the image but wont be notisable
        if width < x+kernel || height < y+kernel {
            return 0.0;
        }
        //print!("----------------------\n\r");
        for i in x.. x+*kernel {
            for j in y..y+*kernel {
                //print!("i: {}  j:{}\n\r", i, j);
                // Get first elements of an array inside the first element of a tuple
                sum += image.get_pixel(i, j).0[0] as f32 / 255.0;
            }
        }
    sum /= *kernel as f32 * *kernel as f32;
    sum
}

/*
TODO
[x] Put code into seperate functions
[ ] Fix error handling
[ ] Put some of the code in seperate crates
*/