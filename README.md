# Build
cargo build --release
# Run
## rnd
### help
```
> target/release/rnd --help
Creates a randomized roaring bitmap, with a set number of bits

Usage: rnd [OPTIONS] --set-bits <SET_BITS>

Options:
  -s, --set-bits <SET_BITS>   Number of bits to set in the bitmap
  -m, --max-bits <MAX_BITS>   max number of total bits, set or unset, in the bitmap [default: 1000000000]
  -d, --disable-progress-bar  display the progress bar as we build the randomized bitmap
  -h, --help                  Print help
  -V, --version               Print version
```
### create, with progress bar and send output to stdout
```
> target/release/rnd --set-bits 10                       
█████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████ 1000000000/1000000000:0
 ��T&8XZ\^`bdfhj        O��:��\��#]��8_�%  
```
### create, without progress bar and send to file > out.roar
```
> target/release/rnd --set-bits 10 --disable-progress-bar > out.roar
>
```
