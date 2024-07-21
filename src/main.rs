use slint::Weak as SlintWeak;
use slint_generatedAppWindow::InnerAppWindow;
use std::{cell::RefCell, fs::File, rc::Rc, rc::Weak, sync::{Arc, Mutex}};
use core::time::Duration;
use std::io::BufReader;
use rodio::{source::{Pausable, Source}, Decoder, OutputStream, Sink};

slint::include_modules!();

#[derive(Clone)]
pub struct State{
    app_window: SlintWeak<AppWindow>,
    sink: Weak<Sink>,
    currently_playing_duration: f32
}

fn main() {
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink: Sink = Sink::try_new(&stream_handle).unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("./guiltyglaser.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    let playing_duration = source.total_duration().unwrap().as_secs_f32();
    // Play the sound directly on the device
    sink.append(source);
    sink.pause();
    
    let app = AppWindow::new().unwrap();
    app.set_current_max(playing_duration);

    let weak: SlintWeak<AppWindow> = app.as_weak();
    let sink_rc = Rc::new(sink);

    let state = State{
        app_window: weak,
        sink: Rc::downgrade(&sink_rc),
        currently_playing_duration:playing_duration,
    };

    let state_copy = state.clone();
    let state_copy_2 = state.clone();
    let state_copy_3 = state.clone();

    app.on_play_music(
       move | | {play_pause_music(&state_copy)}
    );
    app.on_seek_track( move | f: f32 | {seek_music(&state_copy_2, f)}
    );
    app.on_seek_vol( move | f:f32 |{ seek_volume(&state_copy_3, f) });
    app.run().unwrap();
}

pub fn play_pause_music(state: &State){
    let app_window = state.app_window.unwrap();
    if let Some(sink) = state.sink.upgrade(){
        if !app_window.get_playing(){
            sink.play();
            app_window.set_playing(true);
        }
        else{
            sink.pause();
            app_window.set_playing(false);
        }
    };
       
}

pub fn seek_music(state: &State, pos: f32){
    let app_window = state.app_window.unwrap();
    if let Some(sink) = state.sink.upgrade(){
        let pos_per = pos/app_window.get_current_max();
        let max_pos = state.currently_playing_duration;
        let new_pos = pos_per * max_pos;
        sink.try_seek(Duration::from_secs_f32(new_pos)).unwrap();    
    };
}

pub fn seek_volume(state: &State, pos: f32){
    let app_window = state.app_window.unwrap();
    if let Some(sink) = state.sink.upgrade(){
        sink.set_volume(pos/app_window.get_volmax());
    };
}