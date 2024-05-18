use std::env::{args, Args};
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(EnumString, Debug)]
enum Action {
    CreateRandomShuffle
}

impl Action {
    fn execute(&self, mut arg_iter: Args) {
        match self {
            Action::CreateRandomShuffle => {
                let set_bits = arg_iter.next().expect("no set_bits given");
                let max_bits = arg_iter.next().expect("no max_bits given");
                let file_name = arg_iter.next().expect("no file_name given");

                if file_name == "--" {
                    println!("creating a roaring bitmap with {set_bits} bits set in {max_bits} and forward")
                } else {
                    let path = PathBuf::from_str(file_name.as_str());
                    println!("creating a roaring bitmap with {set_bits} bits set in {max_bits} and save to {:?}", path.unwrap())
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
