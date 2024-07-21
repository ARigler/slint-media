use slint::{invoke_from_event_loop, Timer, TimerMode, Weak as SlintWeak};
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
    currently_playing_duration: f32,
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

    let mut state = Rc::new(State{
        app_window: weak,
        sink: Rc::downgrade(&sink_rc),
        currently_playing_duration:playing_duration,
    });

    let mut state_copy = Rc::downgrade(&mut state.clone());
    let mut state_copy_1 = Rc::downgrade(&mut state.clone());
    let mut state_copy_2 = Rc::downgrade(&mut state.clone());
    let mut state_copy_3 = Rc::downgrade(&mut state.clone());

    let timer = Timer::default();
    timer.start(TimerMode::Repeated, std::time::Duration::from_secs(1), move | |{progress_music(&mut state_copy);});
    app.on_play_music(
       move | | {play_pause_music(&mut state_copy_1);}
    );
    app.on_seek_track( move | f: f32 | {seek_music(&mut state_copy_2, f)}
    );
    app.on_seek_vol( move | f:f32 |{ seek_volume(&mut state_copy_3, f) });
    app.run().unwrap();
}

pub fn progress_music(state: &mut Weak<State>){
    if let Some(unwrap_state) = state.upgrade(){
        let app_window = unwrap_state.app_window.unwrap();
        if app_window.get_playing() && app_window.get_current_max() >= app_window.get_current_pos(){
        app_window.set_current_pos(app_window.get_current_pos()+(1.0));
        }
    }
}

pub fn play_pause_music(state: &mut Weak<State>){
    if let Some(unwrap_state) = state.upgrade(){
        let app_window = unwrap_state.app_window.unwrap();
        if let Some(sink) = unwrap_state.sink.upgrade(){
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
       
}

pub fn seek_music(state: &mut Weak<State>, pos: f32){
    if let Some(unwrap_state) = state.upgrade(){    
        let app_window = unwrap_state.app_window.unwrap();
        if let Some(sink) = unwrap_state.sink.upgrade(){
            let pos_per = pos/app_window.get_current_max();
            let max_pos = unwrap_state.currently_playing_duration;
            let new_pos = pos_per * max_pos;
            sink.try_seek(Duration::from_secs_f32(new_pos)).unwrap();  
            app_window.set_current_pos(new_pos);  
        };
    }
}

pub fn seek_volume(state: &mut Weak<State>, pos: f32){
    if let Some(unwrap_state) = state.upgrade(){  
        let app_window = unwrap_state.app_window.unwrap();
        if let Some(sink) = unwrap_state.sink.upgrade(){
            sink.set_volume(pos/app_window.get_volmax());
        };
    }
}