use atty::Stream;
use gif::{Frame, Decoder, Encoder, Reader, Repeat, SetParameter};
use std::env;
use std::fs::File;
use std::io::stdout;
use std::io::Write;
use std::mem;


fn get_file_reader(filename: &str) -> Reader<File> {
    let mut decoder = match File::open(filename) {
        Ok(file) => gif::Decoder::new(file),
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


fn interpolate(component1: u8, component2: u8, amount: f64) -> u8 {
    (((component2 as f64 - component1 as f64) * amount) + component1 as f64) as u8
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

    let args: Vec<_> = env::args().collect();
    let mut empty_img = get_file_reader(&args[1]);
    let mut full_img = get_file_reader(&args[2]);

    let width = empty_img.width();
    let height = empty_img.height();

    let width_px = width as usize;

    let frames = (width_px + ekg_width) / ekg_speed + 1;
    eprintln!("frames: {:?}", frames);

    let empty_img = empty_img.read_next_frame().unwrap().unwrap();
    let full_img = full_img.read_next_frame().unwrap().unwrap();

    let mut image = Vec::new();
    let mut encoder = Encoder::new(&mut image, width, height, &[]).unwrap();
    encoder.set(Repeat::Infinite).unwrap();

    for frame in 0..frames {
        let ekg_pos = frame * ekg_speed;
        eprintln!("{:?}", ekg_pos);
        let pixels = empty_img.buffer.iter().zip(full_img.buffer.iter());
        let pixels_x = pixels.enumerate().map(|(i, pixel_tuple)| (i / 4 % width_px, pixel_tuple));
        let mut buffer: Vec<u8> = pixels_x.map(|(x, (pixel1, pixel2))| {
            match ekg_pos.checked_sub(x) {
                Some(distance) if distance < ekg_width =>
                    interpolate(*pixel1, *pixel2, 1. - (distance as f64 / ekg_width as f64)),
                _ => *pixel1,
            }
        }).collect();

        let mut composite_frame = Frame::from_rgba(width, height, &mut buffer);
        composite_frame.delay = 2;
        encoder.write_frame(&composite_frame).unwrap();
    }

    mem::drop(encoder);
    stdout().write_all(&image).unwrap();
}
