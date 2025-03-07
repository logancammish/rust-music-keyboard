#![windows_subsystem = "windows"]
// use other files inside this project
mod gui;
mod chord;
mod midi;

// use dependencies     
use iced::{Theme, Element, Subscription};
use once_cell::sync::Lazy;
use rodio::{self, OutputStream, Sink, Source};
use std::{collections::HashMap, sync::{Arc, Mutex}, time::Duration};
use threadpool::ThreadPool;
use num_cpus;
use midly::TrackEvent;

// playable trait to implement polymorphism
// for structs RealNote and Chord
trait Playable {
    fn play(&self, bpm: f32, is_recording: bool);
}


static RECORDED_NOTES: Lazy<Arc<Mutex<HashMap<Note, Vec<(f32, f32, f32)>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});
static RECORDING_START_TIME: Lazy<Arc<Mutex<Option<std::time::Instant>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

// Note enum defines all notes in Western music
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Note { 
    A, Asharp, B, C, Csharp, D, Dsharp, E, F, Fsharp, G, Gsharp
}

// NoteLength enum defines the length of a note
// to be calculated according to beats per minute
#[derive(Debug, Clone)]
enum NoteLength {  
    Whole, Half, Quarter, Eighth, Sixteenth
}

// implement the NoteLength enum
// functions: 
//
// 1. duration_in_seconds -> calculates the time 
//                           in seconds that a note should last
// 2. check_bpm           -> checks if the bpm is valid
//                           (not below of equal to 0 or above 300)
impl NoteLength { 
    pub fn duration_in_seconds(&self, bpm: f32) -> f32 {
        match self {
            NoteLength::Whole => (60.0 / bpm) * 4.0,      
            NoteLength::Half => (60.0 / bpm) * 2.0,        
            NoteLength::Quarter => 60.0 / bpm,            
            NoteLength::Eighth => (60.0 / bpm) * 0.5,      
            NoteLength::Sixteenth => (60.0 / bpm) * 0.25,  
        }
    }
    pub fn check_bpm(bpm: f32) -> bool { 
        if (bpm <= 0.0) || (bpm > 300.0) {
            return false
        }

        true
    }
}

// RealNote struct, used for playing sounds according
// to their length and octave
// fields: 
// 1. note           -> Relevant Note enum 
// 2. length         -> Relevant NoteLength enum
// 3. octave         -> The octave that music should be played at
#[derive(Debug, Clone)]
struct RealNote { 
    note: Note, 
    length: NoteLength, 
    octave: f32,
}

// implement the RealNote struct
// functions:
// 1. base_frequencies     -> Determine the octave 0 frequency for the relevant Note
// 2. play_sound           -> Plays the note sound in the current thread
// 3. play_async           -> Plays the note sound in another thread asynchronously
// also implements:
// 1. play (from Playable) -> The same as play_sound
impl RealNote { 
    pub fn base_frequencies(note: Note) -> f32 { 
        match note {
            Note::C => 16.35,
            Note::Csharp => 17.32,
            Note::D => 18.35,
            Note::Dsharp => 19.45,
            Note::E => 20.60,
            Note::F => 21.83,
            Note::Fsharp => 23.12,
            Note::G => 24.50,
            Note::Gsharp => 25.96,
            Note::A => 27.50,   
            Note::Asharp => 29.14,
            Note::B => 30.87,
        }
    }

    fn play_sound(&self, bpm: f32, is_recording: bool) {  
        let time = NoteLength::duration_in_seconds(&self.length, bpm);
        let frequency = Self::base_frequencies(self.note.clone()) * 2_f32.powf(self.octave);
        
        let source = rodio::source::SineWave::new(frequency)
            .amplify(0.1)
            .take_duration(Duration::from_secs_f32(time));

        if is_recording {
            let recording_start_guard = RECORDING_START_TIME.lock().unwrap();
            if let Some(start_time) = &*recording_start_guard {
                let elapsed = start_time.elapsed().as_secs_f32();
                let mut recorded_notes = RECORDED_NOTES.lock().unwrap();
                recorded_notes.entry(self.note.clone())
                    .or_insert_with(Vec::new)
                    .push((self.octave, elapsed, time)); // (octave, start_time, duration)
            }
        }

        let (_stream, handle) = OutputStream::try_default().expect("Failed to create output stream");
        let sink = Sink::try_new(&handle).expect("Failed to create sink");
        sink.append(source);
        sink.play(); 
        sink.sleep_until_end();
    }

    fn play_async(&self, bpm: f32, is_recording: bool) { 
        let notes = vec![self.clone()];
        async_play_note(&notes, bpm, is_recording);
    }
}

// implement Playable trait for RealNote 
impl Playable for RealNote { 
    fn play(&self, bpm: f32, is_recording: bool) {
        self.play_sound(bpm, is_recording);
    }
}

// Chord struct, which is used to play multiple notes at once
// and calculations musically relevant to this concept
struct Chord { 
    notes: Vec<RealNote>
}

// implement the Chord struct
// functions:
// 1. triad_from_note   -> Calculates the major triad 
//                         (the 1st, 3rd and 5th notes of the major scale)
//                         and returns it as a function
// also implements:
// 1. play (from Playable) -> The same as plays the chord asynchronously using
//                            async_play_note
impl Chord {
    fn triad_from_note(note: &RealNote) -> Chord {
        let scale = Self::get_major_scale(note.note.clone());
        return Chord{
            notes: vec![
                RealNote { note: scale[0].clone(), length: note.length.clone(), octave: note.octave },
                RealNote { note: scale[2].clone(), length: note.length.clone(), octave: note.octave },
                RealNote { note: scale[4].clone(), length: note.length.clone(), octave: note.octave }
            ]
        }
    }
}

// implement Playable trait for Chord
impl Playable for Chord { 
    fn play(&self, bpm: f32, is_recording: bool) {
        async_play_note(&self.notes, bpm, is_recording);
    }
}

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

fn async_play_note(notes: &[RealNote], bpm: f32, is_recording: bool) {
    let pool = ThreadPool::new(num_cpus::get());
    for note in notes {
        let note = note.clone();
        pool.execute(move || note.play_sound(bpm, is_recording));
    }
}

// Message enum, which is used to communicate changes to the GUI
#[derive(Debug, Clone)]
enum Message { 
    OctaveChange(f32),
    BpmChange(f32),
    CustomBpmChange(String),
    Play(Note),
    PlayChords,
    PlayAsync,
    ToggleRecoring
}

// Program struct, which stores the current information the program may need
// fields:
// 1. octave        -> The current octave the program is using
// 2. bpm           -> The current beats per minute the program is using
// 3. custom_bpm    -> String representation of the bpm, required for iced
// 4. play_chords   -> Whether or not the play triad button is selected
// 5. play_async    -> Whether or not to play notes asynchronously 
struct Program { 
    octave: f32,
    bpm: f32,
    custom_bpm: String,
    play_chords: bool,
    play_async: bool,
    is_recording: bool
}

// implement the Program struct
// functions: 
// 1. update_bpm    -> check and update the bpm
// 2. view          -> display gui
// 3. update        -> update Program
// 4. subscription  -> sets the iced subscription
impl Program { 
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

    fn view<'a>(&'a self) -> Element<'a, Message> {
        Self::get_ui_information(self).into()
    }    
    
    fn update(&mut self, message: Message) { 
        match message { 
            Message::ToggleRecoring => {
                if self.is_recording == false{
                    self.start_recording();
                } else { 
                    let song = self.stop_recording();
                    midi::Midi::midi_file_create(song);
                }
                //self.is_recording = !self.is_recording;
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

            Message::Play(note) => {
                let note_duration = NoteLength::duration_in_seconds(&NoteLength::Whole, self.bpm);  // Adjusted to use actual duration
                let real_note = RealNote {
                    note: note,
                    length: NoteLength::Whole,  // Use the relevant length or adjust based on actual duration
                    octave: self.octave,
                };

                if self.play_chords == false && self.play_async == false {  
                    real_note.play(self.bpm, self.is_recording);
                } else if self.play_chords == true { 
                    let chord = Chord::triad_from_note(&real_note);
                    chord.play(self.bpm, self.is_recording);
                } else if self.play_async == true {               
                    real_note.play_async(self.bpm, self.is_recording);
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

// changing Default for Program
impl Default for Program { 
    fn default() -> Self {
        Self {
            octave: 2.0,
            bpm: 120.0,
            custom_bpm: "120".to_string(),
            play_chords: false,
            play_async: false,
            is_recording: false
        }
    }
}

// main function
pub fn main() -> iced::Result {
    let (stream, handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&handle).expect("Failed to create sink");
    std::mem::forget(stream);
    
    iced::application("Rust Music Keyboard (c) 2025 Logan Cammish", Program::update, Program::view) 
        .subscription(Program::subscription)
        .theme(|_| Theme::TokyoNight)
        .run()
}