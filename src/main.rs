use core::f64;
use std::{fs::File, io::Write, path::PathBuf};

const RIFF: &[u8] = &[0x52, 0x49, 0x46, 0x46]; // `RIFF`

const WAVE: &[u8] = &[0x57, 0x41, 0x56, 0x45]; // `WAVE`

const FMT: &[u8] = &[
    0x66, 0x6d, 0x74, 0x20, 0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02, 0x00,
]; // 'fmt 'ascii, 16u32 (Subchunk size), 01u16 (1->PCM), 02u16 (number of channels)

const ALIGN_AND_BPS: &[u8] = &[0x04, 0x00, 0x10, 0x00]; // 04u16 (Block size in bytes, channels*bytes/sample), 16u16 (bits per sample)

const DATA: &[u8] = &[0x64, 0x61, 0x74, 0x61]; //`data`ascii

use clap::Parser;

// A clap command line tool; output a wav file!
#[derive(Parser)]
struct ClArgs {
    path: PathBuf,
    samples: u32,
    sample_rate: u32,
    frequency: u32,
    ms_taper: u32,
}

fn main() {
    let args = ClArgs::parse();
    let chunk_size = args.samples * 4 + 36;
    let subchunk_size = args.samples * 4;

    let sample_rate = args.sample_rate;

    let taper_samples = ((sample_rate as f64 / 1000.0) * args.ms_taper as f64) as u32;

    let byte_rate = 4 * sample_rate;

    let mut file = File::create(args.path).unwrap();

    let header_buf = [
        RIFF,
        &chunk_size.to_le_bytes(),
        WAVE,
        FMT,
        &sample_rate.to_le_bytes(),
        &byte_rate.to_le_bytes(),
        ALIGN_AND_BPS,
        DATA,
        &subchunk_size.to_le_bytes(),
    ]
    .concat();

    file.write_all(&header_buf)
        .expect("Failed to write headers");

    for i in 1..args.samples {
        let t = i as f64 / (sample_rate as f64);
        let x = (t * std::f64::consts::TAU * (args.frequency as f64)).sin();

        let taper = if i < args.samples - taper_samples {
            1.0
        } else {
            (args.samples - i) as f64 / (taper_samples as f64)
        };

        let k = (x * taper * (i16::MAX as f64 / 2.0)) as i16;
        let block = [k.to_le_bytes(), k.to_le_bytes()].concat(); // One block, consisting of one sample for each channel.

        file.write_all(&block).expect("Failed to write sample");
    }
}
