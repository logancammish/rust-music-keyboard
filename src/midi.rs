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

    pub fn bpm_to_microseconds_per_beat(bpm: f32) -> u24 {   
        u24::from((60_000_000.0 / bpm) as u32)
    }
    
    pub fn midi_file_create(song: Song) {
        let header = Header::new(
            Format::SingleTrack,
            Timing::Metrical(480.into()) 
        );
        let mut smf = Smf::new(header);

        let mut track: Vec<TrackEvent<'_>> = Track::new();
        let tempo = MetaMessage::Tempo(Self::bpm_to_microseconds_per_beat(song.bpm));
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: midly::TrackEventKind::Meta(tempo),
        });

        for (k, i) in song.notes {
            let current_note = k.unwrap();
            let note_on = MidiMessage::NoteOn {
                key: Self::note_to_midi(current_note.clone(), i.0),  
                vel: u7::new(64)   
            };
            let note_off = MidiMessage::NoteOff {
                key: Self::note_to_midi(current_note.clone(), i.0),  
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
        }

        smf.tracks.push(track);

        let mut buffer = Vec::new();
        smf.write(&mut buffer).expect("Failed to write to buffer");
        File::create("output.mid")
            .expect("Failed to create file")
            .write_all(&buffer)
            .expect("Failed to write to file");
    }
}