// use other files inside this project
use crate::{Program, Note, RealNote, Playable, async_play_note};

// use dependencies
use std::collections::HashMap;

// Chord struct, which is used to play multiple notes at once
// and calculations musically relevant to this concept
pub struct Chord { 
    pub notes: Vec<RealNote>
}

// implement the Chord struct
// functions:
// 1. triad_from_note   -> Calculates the major triad 
//                         (the 1st, 3rd and 5th notes of the major scale)
//                         and returns it as a function
// 2. get_major_scale  -> returns the major scale of the relevant Note
//
// also implements:
// 1. play (from Playable) -> The same as plays the chord asynchronously using
//                            async_play_note

impl Chord {
    pub fn is_note_in_scale(program: &Program, note: Note) -> bool {
        match &program.selected_scale {
            None => true,
            Some(scale_root) => Chord::get_major_scale(*scale_root).contains(&note)
        }
    }

    pub fn triad_from_note(note: &RealNote) -> Chord {
        let scale = Self::get_major_scale(note.note.clone());
        return Chord{
            notes: vec![
                RealNote { note: scale[0].clone(), length: note.length.clone(), octave: note.octave },
                RealNote { note: scale[2].clone(), length: note.length.clone(), octave: note.octave },
                RealNote { note: scale[4].clone(), length: note.length.clone(), octave: note.octave }
            ]
        }
    }
     pub fn get_major_scale(note: Note) -> Vec<Note> { 
        let mut major_scales: HashMap<Note, Vec<Note>> = HashMap::new();
        major_scales.insert(Note::A, vec![
            Note::A, Note::B, Note::Csharp, Note::D, Note::E, Note::Fsharp, Note::Gsharp
        ]);
        major_scales.insert(Note::Asharp, vec![
            Note::Asharp, Note::C, Note::D, Note::Dsharp, Note::F, Note::G, Note::A
        ]);
        major_scales.insert(Note::B, vec![
            Note::B, Note::Csharp, Note::Dsharp, Note::E, Note::Fsharp, Note::Gsharp, Note::Asharp
        ]);
        major_scales.insert(Note::C, vec![
            Note::C, Note::D, Note::E, Note::F, Note::G, Note::A, Note::B
        ]);
        major_scales.insert(Note::Csharp, vec![
            Note::Csharp, Note::Dsharp, Note::F, Note::Fsharp, Note::Gsharp, Note::Asharp, Note::C
        ]);
        major_scales.insert(Note::D, vec![
            Note::D, Note::E, Note::Fsharp, Note::G, Note::A, Note::B, Note::Csharp
        ]);
        major_scales.insert(Note::Dsharp, vec![
            Note::Dsharp, Note::F, Note::G, Note::Gsharp, Note::Asharp, Note::C, Note::D
        ]);
        major_scales.insert(Note::E, vec![
            Note::E, Note::Fsharp, Note::Gsharp, Note::A, Note::B, Note::Csharp, Note::Dsharp
        ]);
        major_scales.insert(Note::F, vec![
            Note::F, Note::G, Note::A, Note::Asharp, Note::C, Note::D, Note::E
        ]);
        major_scales.insert(Note::Fsharp, vec![
            Note::Fsharp, Note::Gsharp, Note::Asharp, Note::B, Note::Csharp, Note::Dsharp, Note::F
        ]);
        major_scales.insert(Note::G, vec![
            Note::G, Note::A, Note::B, Note::C, Note::D, Note::E, Note::Fsharp
        ]);
        major_scales.insert(Note::Gsharp, vec![
            Note::Gsharp, Note::Asharp, Note::C, Note::Csharp, Note::Dsharp, Note::F, Note::G
        ]);
        major_scales.insert(Note::None, Note::ALL.to_vec());

        major_scales.get(&note).expect("Not a valid scale").clone()
    }
}

// implement Playable trait for Chord
impl Playable for Chord { 
    fn play(&self, bpm: f32, is_recording: bool, volume: f32) {
        async_play_note(&self.notes, bpm, is_recording, volume);
    }
}