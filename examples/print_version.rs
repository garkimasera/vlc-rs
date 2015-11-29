
// This file is an example for vlc-rs, licensed under CC0.
// https://creativecommons.org/publicdomain/zero/1.0/deed

extern crate vlc;

fn main() {
    println!("Version : {}", vlc::version());
    println!("Compiler : {}", vlc::compiler());
}
