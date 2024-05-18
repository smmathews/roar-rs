use std::env::{args, Args};
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::EnumString;
use roaring::RoaringBitmap;
use rand::Rng;

#[derive(EnumString, Debug)]
enum Action {
    CreateRandomShuffle
}

impl Action {
    fn execute(&self, mut arg_iter: Args) {
        match self {
            Action::CreateRandomShuffle => {
                let set_bits = u32::from_str(arg_iter.next().expect("no set_bits given").as_str()).unwrap();
                let max_bits = u32::from_str(arg_iter.next().expect("no max_bits given").as_str()).unwrap();
                let file_name = arg_iter.next().expect("no file_name given");



                let mut bitmap = RoaringBitmap::new();
                // add 0 through set_bits-1
                bitmap.insert_range(0..set_bits);

                // randomly shuffle those values throughout the max_bits
                // https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
                let mut rng = rand::thread_rng();
                for i in (0..max_bits).rev() {
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
                }

                if file_name == "|" {
                    bitmap.serialize_into(io::stdout()).unwrap();
                } else {
                    let path = PathBuf::from_str(file_name.as_str()).unwrap();
                    bitmap.serialize_into(File::create(path).unwrap()).unwrap();
                }
            }
        }
    }
}

fn main() {
    let mut arg_iter = args().into_iter();
    arg_iter.next();// disregard the executable name
    let action = Action::from_str(arg_iter.next().expect("no action given").as_str()).unwrap();
    action.execute(arg_iter);
}
