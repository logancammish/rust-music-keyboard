//#![windows_subsystem = "windows"]
// use other files inside this project
mod gui;
mod chord;
mod midi;
mod note;
use gui::{*};
use chord::{*};
use note::{*};


// use dependencies     
use iced::{keyboard::{self}, Element, Size, Subscription, Theme};
use once_cell::sync::Lazy;
use rodio::{self, OutputStream, OutputStreamHandle, Sink, Source};
use std::{fs, io::Read};
use std::{thread, collections::HashMap, fs::File,  sync::{Arc, Mutex}, time::Duration};
use iced::futures::{self, Stream};
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::stream::StreamExt;
use iced_native::subscription::Recipe;
use serde_json;

#[derive(Clone)]
struct SoundRequest {
    frequency: u32,
    duration: f32,
    volume: f32,
    real_note: RealNote,
    bpm: f32
}

// playable trait to implement polymorphism
// for structs RealNote and Chordf
trait Playable {
    fn play(&self, bpm: f32, is_recording: bool, volume: f32);
}

// Mutually exclusive, thread-safe static variables for storing important 
// information which needs to be used throughout the program
static RECORDED_NOTES: Lazy<Arc<Mutex<HashMap<Note, Vec<(f32, f32, f32)>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});
static RECORDING_START_TIME: Lazy<Arc<Mutex<Option<std::time::Instant>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});
const THREAD_POOL: Lazy<Arc<Mutex<rayon::ThreadPool>>> = Lazy::new(|| {
    Arc::new(Mutex::new(rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap()))
});




#[derive(Debug, Clone)]
struct Song {
    notes: Vec<(Note, f32, f32, f32)>, // Note, octave, start_time, duration
    bpm: f32,
}

impl Default for Song { 
    fn default() -> Self {
        Self {
            bpm: 120.0,
            notes: vec![]
        }
    }
}

pub fn async_play_note(notes: &[RealNote], bpm: f32, is_recording: bool, volume: f32) {
    for note in notes {
        let note = note.clone();
        //THREAD_POOL.lock().unwrap().execute(move || note.play_sound(bpm, is_recording, sink));

        THREAD_POOL.lock().unwrap().spawn(move || {
            note.play(bpm, is_recording, volume);
        });
        // thread::spawn(move || {
        //     note.play(bpm, is_recording, volume);
        // });
    }
}

pub fn record_history(real_note: RealNote, time: f32) { 
    let recording_start_guard = RECORDING_START_TIME.lock().unwrap();
    if let Some(start_time) = &*recording_start_guard {
        let elapsed = start_time.elapsed().as_secs_f32();
        let mut recorded_notes = RECORDED_NOTES.lock().unwrap();
        recorded_notes.entry(real_note.note.clone())
            .or_insert_with(Vec::new)
            .push((real_note.octave, elapsed, time)); // (octave, start_time, duration)
    }
}

// Message enum 
#[derive(Debug, Clone, PartialEq)]
enum Message { 
    Scale(Note), 
    OctaveChange(f32),
    BpmChange(f32),
    CustomBpmChange(String),
    Play(Note, bool), // True if played with gui
    EndPlaying(Note),
    KeyPressed(iced::keyboard::Key),
    KeyReleased(iced::keyboard::Key),
    PlayChords,
    PlayAsync,
    ToggleRecoring,
    NoteLengthChange(f32),
    VolumeChange(f32),
    ToggleHelpGUI,
    Tick
}

#[derive(Clone)]
// Program struct, which stores the current information the program may need
// fields:
// 1. octave           -> The current octave the program is using
// 2. bpm              -> The current beats per minute the program is using
// 3. custom_bpm       -> String representation of the bpm, required for iced
// 4. play_chords      -> Whether or not the play triad button is selected
// 5. play_async       -> Whether or not to play notes asynchronously 
// 6. is_recording     -> Whether or not the program is currently recording
// 7. selected_scale   -> The scale that the program is currently using
// 8. time_elapsed     -> The time elapsed since recording started
// 9. note_length      -> The length of the note
// 10. volume          -> The volume of the note
// 11. buttons_pressed -> The buttons that are currently pressed
struct Program { 
    octave: f32,
    bpm: f32,
    custom_bpm: String,
    play_chords: bool,
    play_async: bool,
    is_recording: bool,
    selected_scale: Option<Note>,  
    time_elapsed: f32,
    note_length: f32,
    volume: f32,
    buttons_pressed: HashMap<Note, bool>,
    sound_channel: Arc<Mutex<(std::sync::mpsc::Sender<SoundRequest>, std::sync::mpsc::Receiver<SoundRequest>)>>,
    current_menu: CurrentMenu
}

// implement the Program struct
// functions: 
// 1. update_bpm      -> check and update the bpm
// 2. view            -> display gui
// 3. update          -> update Program
// 4. subscription    -> sets the iced subscription
// 5. start_recording -> begin recording midi file
// 6. stop_recording  -> stop recording midi
// 7. get_note_length -> get the NoteLength from a float
// 8. match_keyboard_key -> match the keyboard key to a Note
impl Program { 
    pub fn get_note_length(length: f32) -> NoteLength { 
        return match length {
            5.0 => NoteLength::Whole,
            4.0 => NoteLength::Half,
            3.0 => NoteLength::Quarter,
            2.0 => NoteLength::Eighth,
            1.0 => NoteLength::Sixteenth,
            _ =>  NoteLength::Whole
        }; 
    }

    pub fn start_recording(&mut self) {
        self.is_recording = true;
        *RECORDING_START_TIME.lock().unwrap() = Some(std::time::Instant::now());
        RECORDED_NOTES.lock().unwrap().clear();  
    }
    
    pub fn stop_recording(&mut self) -> Song {
        self.is_recording = false;
        let recorded_notes = RECORDED_NOTES.lock().unwrap().clone();
    
        let mut song = Song {
            notes: vec![],
            bpm: self.bpm,
        };
    
        for (note, data) in recorded_notes {
            for (octave, start_time, duration) in data {
                song.notes.push((note.clone(), octave, start_time, duration));
            }
        }
        song
    }
    
    pub fn update_bpm(&mut self, value: f32) {
        if NoteLength::check_bpm(value) {
            self.bpm = value;
            self.custom_bpm = value.to_string();
        } else {
            self.bpm = 60.0;
            self.custom_bpm = "60".to_string();
        }
    }

    fn view(&self) -> Element<Message> {
        Self::get_ui_information(self, Arc::new(Mutex::new(self.buttons_pressed.clone()))).into()
    }

    fn match_keyboard_key(key: keyboard::Key) -> Option<Note> {
        match key {
            keyboard::Key::Character(c) => {
                return match c.as_str() {
                    "a" => Some(Note::C),
                    "w" => Some(Note::Csharp),
                    "s" => Some(Note::D),
                    "r" => Some(Note::Dsharp),
                    "d" => Some(Note::E),
                    "f" => Some(Note::F),
                    "t" => Some(Note::Fsharp),
                    "g" => Some(Note::G),
                    "y" => Some(Note::Gsharp),
                    "h" => Some(Note::A),
                    "j" => Some(Note::B),
                    "u" => Some(Note::Asharp),
                    _ => None
                };
            },
            _ => {None}
        }
    }
    
    fn update(&mut self, message: Message) { 
        match message { 
            Message::ToggleHelpGUI => {
                if self.current_menu == CurrentMenu::Help { 
                    self.current_menu = CurrentMenu::Standard
                } else {
                    self.current_menu = CurrentMenu::Help
                }
            }

            Message::NoteLengthChange(value) => {
                self.note_length = value;
            }

            Message::VolumeChange(value) => {
                self.volume = value;
            }

            Message::Tick => {
                if self.is_recording {
                    let now = std::time::Instant::now();
                    self.time_elapsed = now.duration_since(*RECORDING_START_TIME.lock().unwrap().as_ref().unwrap()).as_secs_f32();
                } else {
                    self.time_elapsed = 0.0;
                }

                // if let Ok(sound_request) = self.sound_channel.lock().unwrap().1.try_recv() {
                //     let real_note = sound_request.real_note;
                //     let time = NoteLength::duration_in_seconds(&real_note.length, sound_request.bpm);
                //     let frequency = RealNote::base_frequencies(real_note.note.clone()) * 2_f32.powf(self.octave);
                //     let source = rodio::source::SineWave::new(frequency)
                //         .amplify(0.1)
                //         .take_duration(Duration::from_secs_f32(time));
                //     let (_stream, handle) = OutputStream::try_default().expect("Failed to create output stream");
                //     let sink = Sink::try_new(&handle).expect("Failed to create sink");

                //     sink.append(source);
                //     sink.play(); 
                //     sink.set_volume(sound_request.volume / 10.0);
                //     sink.sleep_until_end();                    
                // }
            }

            Message::Scale(note) => {
                self.selected_scale = Some(note); 
            }

            Message::KeyPressed(key) => {
                let note = Self::match_keyboard_key(key);

                if let Some(note) = note {
                    self.buttons_pressed.insert(note, true); 
                    self.update(Message::Play(note, false));
                }
            },

            Message::KeyReleased(key) => {
                let note = Self::match_keyboard_key(key);
                
                if let Some(note) = note {
                    self.buttons_pressed.insert(note, false); 
                }
            },

            Message::ToggleRecoring => {
                if self.is_recording == false{
                    self.start_recording();
                } else { 
                    let song = self.stop_recording();
                    midi::Midi::midi_file_create(song);
                }
            },

           
            Message::PlayChords => {
                self.play_chords = !self.play_chords;
            }

            Message::PlayAsync => {
                self.play_async = !self.play_async;
            }

            Message::OctaveChange(value) => {
                self.octave = value;
            }

            Message::CustomBpmChange(value) => {
                if let Ok(value) = value.parse::<f32>() {
                    Self::update_bpm(self, value);
                } 
            }

            Message::BpmChange(value) => {
                Self::update_bpm(self, value);
            }

            Message::EndPlaying(note) => {
                self.buttons_pressed.insert(note, false); // Update pressed state
            }

            Message::Play(note, _gui) => {
                self.buttons_pressed.insert(note, true); // Update pressed state

                if note == Note::None {
                    return;
                }

                let note_length: NoteLength = match self.note_length {
                    5.0 => NoteLength::Whole,
                    4.0 => NoteLength::Half,
                    3.0 => NoteLength::Quarter,
                    2.0 => NoteLength::Eighth,
                    1.0 => NoteLength::Sixteenth,
                    _ =>  NoteLength::Whole
                };

                let real_note = RealNote {
                    note: note,
                    length: note_length, 
                    octave: self.octave,
                };

                if self.play_chords == false && self.play_async == false {  
                    real_note.play(self.bpm, self.is_recording, self.volume);
                } else if self.play_chords == true { 
                    let chord = Chord::triad_from_note(&real_note);
                    chord.play(self.bpm, self.is_recording, self.volume);
                } else if self.play_async == true {               
                    real_note.play_async(self.bpm, self.is_recording, self.volume);
                }
            }
        }
    }

    
    fn subscription(&self) -> Subscription<Message> {
        struct Timer;
        impl<H: std::hash::Hasher, E> Recipe<H, E> for Timer {            
            type Output = Message;
            fn hash(&self, state: &mut H) {
                use std::hash::Hash;
                "timer".hash(state);
            }

            fn stream(self: Box<Self>, _: futures::stream::BoxStream<'static, E>) -> futures::stream::BoxStream<'static, Self::Output> {
                futures::stream::unfold((), |_| async {
                    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                    Some((Message::Tick, ()))
                }).boxed()
            }
        }

        impl Stream for Timer {
            type Item = Message;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                cx.waker().wake_by_ref();
                Poll::Ready(Some(Message::Tick))
            }
        }
        
        Subscription::batch(vec![
            keyboard::on_key_press(|key, _modifiers| Some(Message::KeyPressed(key))),
            keyboard::on_key_release(|key, _modifiers| Some(Message::KeyReleased(key))),
            Subscription::run_with_id("timer", Timer)
        ])
    }
}

// changing Default for Program
impl Default for Program { 
    fn default() -> Self {
        let mut buttons_pressed = HashMap::new();
        for note in [
            Note::C, Note::Csharp, Note::D, Note::Dsharp, 
            Note::E, Note::F, Note::Fsharp, Note::G, 
            Note::Gsharp, Note::A, Note::Asharp, Note::B
        ].iter() {
            buttons_pressed.insert(*note, false);
        }

        // Reading settings.json
        let settings = match fs::read_to_string("./config/settings.json") {
            Ok(dp) => dp, 
            Err(_e) => {
                println!("An error occured reading settings"); 
                "[]".to_string()
            }
        };
        let settings_hmap: HashMap<String, bool> = match serde_json::from_str(&settings) {
            Ok(sp) => sp, 
            Err(_e) => {
                println!("An error occured reading settings (bad format)"); 
                HashMap::from([
                    ("info_popup".to_string(), false)
                ])
            }
        };

        let current_menu = if *settings_hmap.get("info_popup").unwrap_or(&false) {
            CurrentMenu::Help
        } else {
            CurrentMenu::Standard
        };

        Self {
            note_length: 2.0, 
            selected_scale: None,  
            octave: 4.0,
            bpm: 120.0,
            custom_bpm: "120".to_string(),
            play_chords: false,
            play_async: true,
            is_recording: false,
            time_elapsed: 0.0,
            volume: 30.0,
            buttons_pressed: buttons_pressed,
            sound_channel: Arc::new(Mutex::new(
                std::sync::mpsc::channel::<SoundRequest>()
            )),
            current_menu: current_menu
        }
    }
}


// main function
pub fn main() -> iced::Result {
    let mut icon_bytes = Vec::new();
    let mut file = match File::open("./assets/icon.ico") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open the icon file: {}", e);
            return Ok(()); 
        }
    };

    if let Err(e) = file.read_to_end(&mut icon_bytes) {
        eprintln!("Failed to read the icon file: {}", e);
        return Ok(()); 
    }

    let icon = match image::ImageReader::open("./assets/icon.ico") {
        Ok(image_reader) => {
            match image_reader.decode() {
                Ok(img) => {
                    let rgba_image = img.into_rgba8();
                    let (width, height) = rgba_image.dimensions();
                    
                    match iced::window::icon::from_rgba(rgba_image.into_raw(), width, height) {
                        Ok(icon) => Some(icon),
                        Err(e) => {
                            eprintln!("Failed to create icon: {}", e);
                            None
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to decode the image: {}", e);
                    None
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to open the icon file: {}", e);
            None
        }
    };

    let window_settings = iced::window::Settings {
        icon,
        ..iced::window::Settings::default()
    };


    iced::application("Rust Music Keyboard", Program::update, Program::view)
        .window_size(Size::new(700.0, 720.0))
        .subscription(Program::subscription)
        .theme(|_| Theme::TokyoNight)
    .window(window_settings)
        .run()
}