use midly::{Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent};
use midly::num::{u28, u24, u7, u4};
use std::io::Write;
use std::env;
use std::fs::{self, File};
use std::path::PathBuf;
use crate::{Note, Song};

pub struct Midi {}

// impliment for Midi
// functions: 
// 1. note_to_midi  -> converts note to u7 midi value
// 2. bpm_to_microseconds_per_beat  -> converts bpm to u24 microseconds per beat
// 3. midi_file_create -> creates a midi file with the valid info
impl Midi {
    pub fn note_to_midi(note: Note, octave: f32) -> u7 {
        let note_index = match note {
            Note::C => 0,
            Note::Csharp => 1,
            Note::D => 2,
            Note::Dsharp => 3,
            Note::E => 4,
            Note::F => 5,
            Note::Fsharp => 6,
            Note::G => 7,
            Note::Gsharp => 8,
            Note::A => 9,
            Note::Asharp => 10,
            Note::B => 11,
            Note::None => 0, // Default to C when Note is None
        };
        let midi_note = 12 * (octave as i32 + 1) + note_index;

        u7::new(midi_note as u8)
    }

    pub fn bpm_to_microseconds_per_beat(bpm: f32) -> u24 {
        u24::from((60_000_000.0 / bpm) as u32)
    }

    pub fn midi_file_create(song: Song) {
        let header = Header::new(Format::SingleTrack, Timing::Metrical(480.into()));
        let mut smf = Smf::new(header);
    
        let mut track: Vec<TrackEvent<'_>> = Track::new();
        let tempo = MetaMessage::Tempo(Self::bpm_to_microseconds_per_beat(song.bpm));
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: midly::TrackEventKind::Meta(tempo),
        });
    
        let mut events = Vec::new();
        
        for (note, octave, start_time, duration) in &song.notes {
            // Skip Note::None entries
            if *note == Note::None {
                continue;
            }
            
            let midi_note = Self::note_to_midi(note.clone(), *octave);
            let beats_per_second = song.bpm / 60.0;
            let start_ticks = (start_time * beats_per_second * 480.0).round() as u32;
            let duration_ticks = (duration * beats_per_second * 480.0).round() as u32;
    
            events.push((
                start_ticks,
                midly::TrackEventKind::Midi {
                    channel: u4::new(0),
                    message: MidiMessage::NoteOn {
                        key: midi_note,
                        vel: u7::new(64),
                    },
                },
            ));
    
            events.push((
                start_ticks + duration_ticks,
                midly::TrackEventKind::Midi {
                    channel: u4::new(0),
                    message: MidiMessage::NoteOff {
                        key: midi_note,
                        vel: u7::new(64),
                    },
                },
            ));
        }
    
        events.sort_by_key(|(time, _)| *time);
    
        let mut last_time = 0;
        for (time, event) in events {
            track.push(TrackEvent {
                delta: u28::new(time - last_time),
                kind: event,
            });
            last_time = time;
        }
    
        smf.tracks.push(track);
        
        let output_file: PathBuf;

        if cfg!(target_os = "windows") {
            let username = env::var("USERNAME").expect("Failed to get USERNAME environment variable");
            let mut output_dir = PathBuf::from("C:\\Users");
            output_dir.push(username);
            output_dir.push("Documents\\RustMusicKeyboard");
            fs::create_dir_all(&output_dir).expect("Failed to create directory");
            output_file = output_dir.join("output.mid");
    
        } else if cfg!(target_os = "linux") {
            output_file = PathBuf::from("/tmp/output.mid");
    
        } else {
            output_file = PathBuf::from("output.mid");
        }
    

        let buffer = Vec::new();
        // smf.write(&mut buffer).expect("Failed to write to buffer"); // Uncomment if using `smf`
        File::create(&output_file)
            .expect("Failed to create file")
            .write_all(&buffer)
            .expect("Failed to write to file");

        println!("MIDI file saved at: {:?}", output_file);
    }
}