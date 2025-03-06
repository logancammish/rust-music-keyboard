use midly::{Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent};
use midly::num::{u28, u24, u7, u4};
use std::fs::File;
use std::io::Write;
use crate::{Note, Song};

pub struct Midi {} 

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
        };
        let octave_difference = (octave - 4.0).round() as u32;
        let midi_note = 12 * octave_difference + note_index;
    
        u7::new(midi_note as u8)
    }

    // Remember song: Some(Note) / Octave / Time played at
    fn midi_file_create(song: Song) {
        todo!("NOT FOR PRODUCTION USE:  Private, Change to pub");

        let header = Header::new(
            Format::SingleTrack,
            Timing::Metrical(480.into()) 
        );
        let mut smf = Smf::new(header);

        let mut track: Vec<TrackEvent<'_>> = Track::new();
        // Set the tempo (500,000 microseconds per beat = 120 BPM)
        let tempo = MetaMessage::Tempo(u24::from(500_000));
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: midly::TrackEventKind::Meta(tempo),
        });

        let note_on = MidiMessage::NoteOn {
            key: u7::new(60),  // Middle C
            vel: u7::new(64)   // Velocity
        };
        let note_off = MidiMessage::NoteOff {
            key: u7::new(60),
            vel: u7::new(64)
        };

        track.push(TrackEvent {
            delta: u28::new(0),
            kind: midly::TrackEventKind::Midi {
                channel: u4::new(0),
                message: note_on
            },
        });

        track.push(TrackEvent {
            delta: u28::new(48),
            kind: midly::TrackEventKind::Midi {
                channel: u4::new(0),
                message: note_off
            },
        });

        smf.tracks.push(track);

        let mut buffer = Vec::new();
        smf.write(&mut buffer).expect("Failed to write to buffer");

        // Write buffer to file
        File::create("output.mid")
            .expect("Failed to create file")
            .write_all(&buffer)
            .expect("Failed to write to file");

        println!("MIDI file created successfully!");
    }
}