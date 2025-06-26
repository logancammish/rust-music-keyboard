use std::time::Duration;

use strum_macros::Display;
use rodio::{self, OutputStream, Sink, Source};
use crate::{Playable, async_play_note, record_history};


// Note enum defines all notes in Western music
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Note { 
    A, Asharp, B, C, Csharp, D, Dsharp, E, F, Fsharp, G, Gsharp, None
}

impl Note {
    pub const ALL: [Note; 13] = [
        Note::C, Note::Csharp, Note::D, Note::Dsharp, Note::E, Note::F, Note::Fsharp, Note::G, Note::Gsharp, Note::A, Note::Asharp, Note::B, Note::None
    ];
}

// NoteLength enum defines the length of a note
// to be calculated according to beats per minute
#[derive(Debug, Clone, Copy, Display)]
pub enum NoteLength {  
    Whole, Half, Quarter, Eighth, Sixteenth
}

// implement the NoteLength enum
// functions: 
//
// 1. duration_in_seconds -> calculates the time 
//                           in seconds that a note should last
// 2. check_bpm           -> checks if the bpm is valid
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
        if (bpm <= 0.1) || (bpm > 300.0) {
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
pub struct RealNote { 
    pub note: Note, 
    pub length: NoteLength, 
    pub octave: f32,
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
            Note::None => 0.0,
        }
    }

    fn play_sound(&self, bpm: f32, is_recording: bool, volume: f32) {  
        let time = NoteLength::duration_in_seconds(&self.length, bpm);
        let frequency = Self::base_frequencies(self.note.clone()) * 2_f32.powf(self.octave);
        let source = rodio::source::SineWave::new(frequency)
            .amplify(0.1)
            .take_duration(Duration::from_secs_f32(time));
        let (_stream, handle) = OutputStream::try_default().expect("Failed to create output stream");
        let sink = Sink::try_new(&handle).expect("Failed to create sink");

        if is_recording {
            record_history(self.clone(), time);
        }
        sink.append(source);
        sink.play(); 
        sink.set_volume(volume / 10.0);
        sink.sleep_until_end();
    }

    pub fn play_async(&self, bpm: f32, is_recording: bool, volume: f32) { 
        let notes = vec![self.clone()];
        async_play_note(&notes, bpm, is_recording, volume);
    }
}

// implement Playable trait for RealNote 
impl Playable for RealNote { 
    fn play(&self, bpm: f32, is_recording: bool, volume: f32) {
        self.play_sound(bpm, is_recording, volume);
    }
}