use std::io::{stdout, Write};
use croaring::{Bitmap, Native, Portable};
use rand::Rng;
use clap::Parser;
use indicatif::ProgressBar;

/// Creates a randomized roaring bitmap, with a set number of bits
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of bits to set in the bitmap
    #[arg(short, long)]
    set_bits: u32,

    /// max number of total bits, set or unset, in the bitmap
    #[arg(short, long, default_value_t = 1_000_000_000)]
    max_bits: u32,

    /// disables displaying the progress bar as we build the randomized bitmap
    #[arg(short, long, default_value_t = false)]
    disable_progress_bar: bool,

    /// if enabled, serialization will be more efficient, but can't be imported with java/go
    #[arg(short, long, default_value_t = false)]
    native_serialization: bool
}

trait ProgressOutput {
    fn incr(&self);
    fn finish(&self);
}

struct ProgressBarOutput {
    pb : ProgressBar
}

impl ProgressOutput for ProgressBarOutput {
    fn incr(&self) {
        self.pb.inc(1);
    }
    fn finish(&self) {
        self.pb.finish();
    }
}

struct NoOpProgressOutput {
}

impl ProgressOutput for NoOpProgressOutput {
    fn incr(&self) {
    }
    fn finish(&self) {
    }
}

fn create_progress_output(args: &Args) -> Box<dyn ProgressOutput> {
    if args.disable_progress_bar {
        Box::new(NoOpProgressOutput {}) }
    else {
        Box::new(ProgressBarOutput {pb: ProgressBar::new(args.max_bits as u64)})
    }
}

fn main() {
    let args = Args::parse();

    // add 0 through set_bits-1
    let mut bitmap = Bitmap::from_range(0..args.set_bits);

    let progress_output = &*create_progress_output(&args);

    // randomly shuffle those values throughout the max_bits
    // https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
    let mut rng = rand::thread_rng();
    for i in (0..args.max_bits).rev() {
        let j = rng.gen_range(0..=i);
        let jb = bitmap.contains(j);
        let ib = bitmap.contains(i);
        if jb && !ib {
            bitmap.add(i);
            bitmap.remove(j);
        } else if ib && !jb {
            bitmap.add(j);
            bitmap.remove(i);
        }
        progress_output.incr();
    }

    progress_output.finish();
    bitmap.run_optimize();
    let size = if args.native_serialization {
        bitmap.get_serialized_size_in_bytes::<Native>()
    } else {
        bitmap.get_serialized_size_in_bytes::<Portable>()
    };
    let mut buffer = Vec::with_capacity(size);
    if args.native_serialization {
        bitmap.serialize_into::<Native>(&mut buffer);
    } else {
        bitmap.serialize_into::<Portable>(&mut buffer);
    }
    stdout().write_all(&*buffer).unwrap();
    std::process::exit(exit_code::SUCCESS);
}
