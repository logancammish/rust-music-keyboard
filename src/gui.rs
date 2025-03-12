use iced::{alignment, border::Radius, font::Weight, widget::{self, button, checkbox, container, pick_list, slider, text, text_input, Space}, Border, Color, Font, Length, Theme};
use crate::{Message, Note, Program, Chord};
use std::{time::{Duration, Instant}};

use std::fmt;
use std::string::ToString;

fn button_style(_theme: &Theme, _status: button::Status, note_color: Color, in_scale: bool) -> button::Style {
    let color = if in_scale {
        note_color
    } else {
        Color {
            a: 0.5,
            ..note_color
        }
    };
    
    button::Style {
        background: Some(iced::Background::Color(color)),
        text_color: if note_color == Color::BLACK { Color::WHITE } else { Color::BLACK },
        border: Border { radius: Radius::from(2), ..Border::default() },
        ..button::Style::default()
    }
}

impl ToString for Note {
    fn to_string(&self) -> String {
        match self {
            Note::C => "C".to_string(),
            Note::Csharp => "C#".to_string(),
            Note::D => "D".to_string(),
            Note::Dsharp => "D#".to_string(),
            Note::E => "E".to_string(),
            Note::F => "F".to_string(),
            Note::Fsharp => "F#".to_string(),
            Note::G => "G".to_string(),
            Note::Gsharp => "G#".to_string(),
            Note::A => "A".to_string(),
            Note::Asharp => "A#".to_string(),
            Note::B => "B".to_string(),
            Note::None => "None".to_string(),
        }
    }
}

impl Program {
    fn is_note_in_scale(&self, note: Note) -> bool {
        match &self.selected_scale {
            None => true,
            Some(scale_root) => Chord::get_major_scale(*scale_root).contains(&note)
        }
    }

    pub fn get_ui_information(&self) -> iced::widget::Container<Message> {
        container(widget::column![
            widget::row!(
                text("Octave"),
                slider(
                    0.0..=8.0,
                    self.octave,
                    Message::OctaveChange
                ),
            ).spacing(10),

            widget::row!(
                text("BPM"),
                slider(
                    10.0..=300.0,
                    self.bpm, 
                    Message::BpmChange
                ),  
                
                text_input(format!("{}", &self.bpm).as_str(), &self.custom_bpm)
                    .on_input(Message::CustomBpmChange) 
                    .padding(2)
                    .width(Length::Fixed(50.0)),
            ).spacing(10),

            // widget::row!(
            //     text("Note length"),
            //     slider(
            //         0.0..=1.0,
            //         self.Note, 
            //         Message::NoteLengthChange
            //     ),  
                
            //     text_input(format!("{}", &self.bpm).as_str(), &self.custom_bpm)
            //         .on_input(Message::CustomBpmChange) 
            //         .padding(2)
            //         .width(Length::Fixed(50.0)),
            // ).spacing(10),

            widget::row!(
                checkbox("Play major scale triads", self.play_chords)
                    .on_toggle(|_| Message::PlayChords)
                    .spacing(10),
                
                checkbox("Play asynchronously", self.play_async)
                    .on_toggle(|_| Message::PlayAsync)
                    .spacing(10),
            ).spacing(20),

            widget::stack!(
                widget::row!(
                    button(text(format!("C{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::C))
                        .style(|theme, status| button_style(theme, status, Color::WHITE, self.is_note_in_scale(Note::C)))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("D{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::D))
                        .style(|theme, status| button_style(theme, status, Color::WHITE, self.is_note_in_scale(Note::D)))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("E{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::E))
                        .style(|theme, status| button_style(theme, status, Color::WHITE, self.is_note_in_scale(Note::E)))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("F{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::F))
                        .style(|theme, status| button_style(theme, status, Color::WHITE, self.is_note_in_scale(Note::F)))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("G{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::G))
                        .style(|theme, status| button_style(theme, status, Color::WHITE, self.is_note_in_scale(Note::G)))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("A{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::A))
                        .style(|theme, status| button_style(theme, status, Color::WHITE, self.is_note_in_scale(Note::A)))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("B{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::B))
                        .style(|theme, status| button_style(theme, status, Color::WHITE, self.is_note_in_scale(Note::B)))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                ).spacing(2).padding(5),
            
                widget::row!(
                    Space::with_width(59.5),
                    button(text(format!("Db{}\nC#{}", self.octave, self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::Csharp))
                        .style(|theme, status| button_style(theme, status, Color::BLACK, self.is_note_in_scale(Note::Csharp)))
                        .width(Length::Fixed(63.75))
                        .height(Length::Fixed(132.6))
                        .padding(5),
                    Space::with_width(34.0),
                    button(text(format!("Eb{}\nD#{}",  self.octave, self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::Dsharp))
                        .style(|theme, status| button_style(theme, status, Color::BLACK, self.is_note_in_scale(Note::Dsharp)))
                        .width(Length::Fixed(63.75))
                        .height(Length::Fixed(132.6))
                        .padding(5),
                    Space::with_width(93.5),
                    button(text(format!("Gb{}\nF#{}", self.octave, self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::Fsharp))
                        .style(|theme, status| button_style(theme, status, Color::BLACK, self.is_note_in_scale(Note::Fsharp)))
                        .width(Length::Fixed(63.75))
                        .height(Length::Fixed(132.6))
                        .padding(5),
                    Space::with_width(34.0),
                    button(text(format!("Ab{}\nG#{}", self.octave, self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::Gsharp))
                        .style(|theme, status| button_style(theme, status, Color::BLACK, self.is_note_in_scale(Note::Gsharp)))
                        .width(Length::Fixed(63.75))
                        .height(Length::Fixed(132.6))
                        .padding(5),
                    Space::with_width(34.0),
                    button(text(format!("Bb{}\nA#{}", self.octave, self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::Asharp))
                        .style(|theme, status| button_style(theme, status, Color::BLACK, self.is_note_in_scale(Note::Asharp)))
                        .width(Length::Fixed(63.75))
                        .height(Length::Fixed(132.6))
                        .padding(5),
                ).spacing(0),
            ),

            Space::with_height(50), 

            widget::column![
                widget::row!(
                    if self.is_recording {
                        button(text("Stop recording")).on_press(Message::ToggleRecoring)
                    } else {
                        button(text("Record")).on_press(Message::ToggleRecoring)
                    },
                    text(format!("Time recorded: {:.2}",  self.time_elapsed
                    )),
                ).spacing(10) 

            ].spacing(20),

            Space::with_height(50), 

            widget::column![
                widget::row!( 
                    text("Select Major Scale: "),
                    pick_list(
                        Note::ALL,
                        self.selected_scale.clone(),
                        Message::Scale
                    ).width(Length::Fixed(150.0)),  
                )
            ],
            
            ]
        ).into()
    }
}
