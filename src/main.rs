use atty::Stream;
use gif::{Frame, Decoder, Encoder, Reader, Repeat, SetParameter};
use std::env;
use std::fs::File;
use std::io::stdout;
use std::io::Write;
use std::mem;


fn get_file_reader(filename: &str) -> Reader<File> {
    let mut decoder = match File::open(filename) {
        Ok(file) => Decoder::new(file),
        Err(msg) => {
            eprintln!("{} - {}", filename, msg);
            std::process::exit(1)
        }
    };
    // Configure the decoder such that it will expand the image to RGBA.
    decoder.set(gif::ColorOutput::RGBA);
    // Read the file header
    decoder.read_info().unwrap()
}


fn interpolate(component1: u8, component2: u8, mut amount: f64) -> u8 {
    if amount > 0.9 {
        return component2;
    }
    amount *= 0.8;
    (((f64::from(component2) - f64::from(component1)) * amount) + f64::from(component1)) as u8
}

/// Compute (a - b) % m, but using the modulo instead of the remainder.
/// The distinction is in the negatives: when `a - b` is negative the `remainder` will be negative,
/// but the modulo will be `remainder + m`
fn sub_modulo(a: usize, b: usize, m: usize) -> usize {
    if a >= b {
        (a - b) % m
    } else {
        (m - (b - a) % m) % m
    }
}


fn main() {
    if atty::is(Stream::Stdout) {
        println!(
            "This command returns an image file. Your terminal likely cannot display it directly!"
        );
        println!(
            "Redirect output to a file or `imgcat`. If you really want to see the bytes \
            in your terminal pipe the result to `cat`."
        );
        std::process::exit(1);
    }
    // TODO: parametrize?
    let ekg_width = 400;
    let ekg_speed = 20;
    let ekg_frame_overlap = 5;

    let args: Vec<_> = env::args().collect();
    let mut empty_img = get_file_reader(&args[1]);
    let mut full_img = get_file_reader(&args[2]);

    let width = empty_img.width();
    let height = empty_img.height();

    let width_px = width as usize;

    let frames = (width_px + ekg_width) / ekg_speed - ekg_frame_overlap;
    let cycle_width = frames * ekg_speed;

    let empty_img = empty_img.read_next_frame().unwrap().unwrap();
    let full_img = full_img.read_next_frame().unwrap().unwrap();

    let mut image = Vec::new();
    let mut encoder = Encoder::new(&mut image, width, height, &[]).unwrap();
    encoder.set(Repeat::Infinite).unwrap();

    eprintln!();
    for frame in 0..frames {
        let ekg_pos = frame * ekg_speed;
        eprint!("\r{}/{}", frame, frames);
        let pixels = empty_img.buffer.iter().zip(full_img.buffer.iter());
        let pixels_x = pixels.enumerate().map(|(i, pixel_tuple)| {
            (i / 4 % width_px, pixel_tuple)
        });
        let mut buffer: Vec<u8> = pixels_x
            .map(|(x, (pixel1, pixel2))| {
                let distance = sub_modulo(ekg_pos, x, cycle_width);
                if distance < ekg_width {
                    interpolate(*pixel1, *pixel2, 1. - (distance as f64 / ekg_width as f64))
                } else {
                    *pixel1
                }
            })
            .collect();

        let mut composite_frame = Frame::from_rgba(width, height, &mut buffer);
        composite_frame.delay = 4;
        encoder.write_frame(&composite_frame).unwrap();
    }
    eprintln!("\r{}/{}", frames, frames);

    mem::drop(encoder);
    stdout().write_all(&image).unwrap();
}
