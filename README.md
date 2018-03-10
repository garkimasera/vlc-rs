# vlc-rs
Rust bindings for libVLC media framework.

## Status
Many missing functions and wrappers.

## Use
Please add the following dependencies to your Cargo.toml.

```Toml
[dependencies]
vlc-rs = "0.2"
```

Or:

```Toml
[dependencies.vlc-rs]
git = "https://github.com/garkimasera/vlc-rs.git"
```

## Example
Play for 10 seconds from a media file.
```Rust
extern crate vlc;
use vlc::{Instance, Media, MediaPlayer};
use std::thread;

fn main() {
    // Create an instance
    let instance = Instance::new().unwrap();
    // Create a media from a file
    let md = Media::new_path(&instance, "path_to_a_media_file.ogg").unwrap();
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();

    // Wait for 10 seconds
    thread::sleep(::std::time::Duration::from_secs(10));
}
```

Other examples are in the examples directory.

## License
MIT (Examples are licensed under CC0)
