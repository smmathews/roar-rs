use std::io;
use roaring::RoaringBitmap;
use rand::Rng;
use clap::Parser;
use indicatif::ProgressBar;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Number of bits to set in the bitmap
    #[arg(short, long)]
    set_bits: u32,

    // max number of total bits, set or unset, in the bitmap
    #[arg(short, long, default_value_t = 1_000_000_000)]
    max_bits: u32
}

fn main() {
    let args = Args::parse();

    let mut bitmap = RoaringBitmap::new();
    // add 0 through set_bits-1
    bitmap.insert_range(0..args.set_bits);

    let pb = ProgressBar::new(args.max_bits as u64);

    // randomly shuffle those values throughout the max_bits
    // https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
    let mut rng = rand::thread_rng();
    for i in (0..args.max_bits).rev() {
        let j = rng.gen_range(0..=i);
        let jb = bitmap.contains(j);
        let ib = bitmap.contains(i);
        if jb && !ib {
            bitmap.insert(i);
            bitmap.remove(j);
        } else if ib && !jb {
            bitmap.insert(j);
            bitmap.remove(i);
        }
        pb.inc(1);
    }

    bitmap.serialize_into(io::stdout()).unwrap();

    std::process::exit(exit_code::SUCCESS);
}
