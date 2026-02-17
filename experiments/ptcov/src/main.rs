use ptcov::PtImage;
use std::{env, fs::File, io::Read};

fn main() {
    // env_logger::init();

    let trace_path = env::var("XDC_TRACE").unwrap();
    let image_path = env::var("XDC_IMAGE").unwrap();
    let image_from = env::var("XDC_BASE").unwrap().parse::<u64>().unwrap();
    let image_to = env::var("XDC_END").unwrap().parse::<u64>().unwrap();

    let trace_file = File::open(trace_path).unwrap();
    let trace = unsafe { memmap2::Mmap::map(&trace_file).unwrap() };

    let mut raw_image = vec![0; (image_to - image_from) as usize];
    let mut image_file = File::open(image_path).unwrap();
    image_file.read_exact(&mut raw_image).unwrap();
    let image = PtImage::new(raw_image, image_from);

    let mut pt_cov = ptcov::PtCoverageDecoderBuilder::new()
        .images(vec![image])
        .build();
    let mut bitmap = vec![0u8; 0x10000];
    pt_cov.coverage(&trace, &mut bitmap).unwrap();
}
