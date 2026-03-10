use ptcov::PtImage;
#[cfg(feature = "export_coverage")]
use std::io::Write;
use std::{env, fs::File};

fn main() {
    // env_logger::init();

    let trace_path = env::var("XDC_TRACE").unwrap();
    let image_path = env::var("XDC_IMAGE").unwrap();
    let image_from = env::var("XDC_BASE").unwrap().parse::<u64>().unwrap();

    let trace_file = File::open(trace_path).unwrap();
    let trace = unsafe { memmap2::Mmap::map(&trace_file).unwrap() };

    let image_file = File::open(image_path).unwrap();
    let raw_image = unsafe { memmap2::Mmap::map(&image_file).unwrap() };
    let images = [PtImage::new(&raw_image, image_from)];

    let mut pt_cov = ptcov::PtCoverageDecoderBuilder::new()
        .images(&images)
        .build();
    let mut bitmap = vec![0u8; 0x10000];

    pt_cov.coverage(&trace, &mut bitmap).unwrap();

    #[cfg(feature = "export_coverage")]
    {
        let mut out_file = File::create("ptcov_bitmap.bin").unwrap();
        out_file.write_all(&bitmap).unwrap();
    }
}
