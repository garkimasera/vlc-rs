
// This file is an example for vlc-rs, licensed under CC0.
// https://creativecommons.org/publicdomain/zero/1.0/deed

extern crate vlc;

use std::sync::mpsc::channel;

use vlc::{Instance, Media, MediaPlayer, Event, EventType, State};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let path = match args.get(1) {
        Some(s) => s,
        None => {
            println!("Usage: cli_audio_player path_to_a_media_file");
            return;
        }
    };
    let instance = Instance::new().unwrap();
    
    let md = Media::new_path(&instance, path).unwrap();    
    let mdp = MediaPlayer::new(&instance).unwrap();
    
    let (tx, rx) = channel::<()>();
    
    let em = md.event_manager();
    let _ = em.attach(EventType::MediaStateChanged, move |e, _| {
        match e {
            Event::MediaStateChanged(s) => {
                println!("State : {:?}", s);
                if s == State::Ended || s == State::Error {
                    tx.send(()).unwrap();
                }
            },
            _ => (),
        }
    });
    
    mdp.set_media(&md);
    
    // Start playing
    mdp.play().unwrap();
    
    // Wait for end state
    rx.recv().unwrap();    
}


