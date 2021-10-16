use image::*;
use rand::prelude::*;
use rand::distributions::Alphanumeric;
use rayon::prelude::*;
use std::iter;
use std::path::PathBuf;
use structopt::StructOpt;
use image::imageops::FilterType;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(long = "initial-size", default_value = "2")]
    initial_size: u32,

    #[structopt(short = "o", long = "output-dir", default_value = ".", parse(from_os_str))]
    output_directory: PathBuf,

    #[structopt(long = "output-size", default_value = "1024")]
    output_size: u32,

    #[structopt(long = "output-format", default_value = "jpg")]
    output_format: String,

    #[structopt(name = "count", default_value = "1")]
    count: u32,
}

fn main() {
    let options = Options::from_args();
    let count = options.count;

    println!("Generating {} random images", count);

    (0..count).into_par_iter().for_each(|i| {
        let mut rng = rand::thread_rng();
        let mut path = options.output_directory.clone();

        let filename = gen_filename(&mut rng, options.output_format.as_str());
        println!("  [{}/{}] {}", i + 1, count, filename);
        path.push(filename);

        let buf = gen_image(&mut rng, &options);

        buf.save(path).expect("Failed saving file")
    });
}

fn gen_filename(mut rng: impl RngCore, extension: &str) -> String {
    let length = 32;
    let mut name = String::with_capacity(length + 1 + extension.len());

    for c in iter::repeat(()).map(|_| rng.sample(Alphanumeric)).take(length) {
        name.push(c);
    }

    name.push_str(".");
    name.push_str(extension);

    name
}

fn gen_image(mut rng: impl RngCore, options: &Options) -> ImageBuffer<Rgb<u8>, Vec<u8>>  {
    let mut buf = image::ImageBuffer::new(options.initial_size, options.initial_size);

    for (_, _, pixel) in buf.enumerate_pixels_mut() {
        *pixel = image::Rgb([rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255)],);
    }

    image::imageops::resize(&buf, options.output_size, options.output_size, FilterType::Gaussian)
}
