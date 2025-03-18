use iced::{alignment, border::Radius, font::Weight, widget::{self, button, checkbox, container, pick_list, slider, text, text_input, Space}, Border, Color, Font, Length, Theme};
use crate::{Message, Note, Program, Chord};
use std::{collections::HashMap, hash::Hash, time::{Duration, Instant}};

use std::string::ToString;



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
    fn button_style(&mut self, _theme: &Theme, _status: button::Status, note_color: Color, in_scale: bool, pressed: bool) -> button::Style {
        let color = if pressed { 
            Color {
                a: 0.2,
                ..note_color
            }
        } else if in_scale {
            note_color 
        } else {
            Color {
                a: 0.5,
                ..note_color
            }
        };
    
        self.buttons_pressed.get(&Note::C).replace(&false);
        
        button::Style {
            background: Some(iced::Background::Color(color)),
            text_color: if note_color == Color::BLACK { Color::WHITE } else { Color::BLACK },
            border: Border { radius: Radius::from(2), ..Border::default() },
            ..button::Style::default()
        }
    }


    fn is_note_in_scale(&self, note: Note) -> bool {
        match &self.selected_scale {
            None => true,
            Some(scale_root) => Chord::get_major_scale(*scale_root).contains(&note)
        }
    }

    pub fn get_ui_information(&self, buttons_pressed: HashMap<Note, bool>) -> iced::widget::Container<Message> {
        let buttons_pressed = buttons_pressed.clone(); // Clone once here
        container(widget::column![
            widget::row!(
                text("Octave:"),
                slider(
                    0.0..=7.0,
                    self.octave,
                    Message::OctaveChange
                ),
            ).spacing(10),

            widget::row!(
                text("Note Length"),
                slider(
                    1.0..=5.0,
                    self.note_length,
                    Message::NoteLengthChange
                ),
                text(format!("Length: {}", Self::get_note_length(self.note_length))),
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
                        .style({
                            move |theme, status| {
                                Self::button_style(&mut self, theme, status, Color::WHITE, self.is_note_in_scale(Note::C), true)
                            }
                        })
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("D{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::D))
                        .style({
                            move |theme, status| {
                                Self::button_style(&mut self, theme, status, Color::WHITE, self.is_note_in_scale(Note::C), true)
                            }
                        })
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("E{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::E))
                        .style({
                            move |theme, status| {
                                Self::button_style(&mut self, theme, status, Color::WHITE, self.is_note_in_scale(Note::C), true)
                            }
                        })
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("F{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::F))
                    .style({
                        move |theme, status| {
                            Self::button_style(&mut self, theme, status, Color::WHITE, self.is_note_in_scale(Note::C), true)
                        }
                    })
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("G{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::G))
                    .style({
                        move |theme, status| {
                            Self::button_style(&mut self, theme, status, Color::WHITE, self.is_note_in_scale(Note::C), true)
                        }
                    })
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("A{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::A))
                    .style({
                        move |theme, status| {
                            Self::button_style(&mut self, theme, status, Color::WHITE, self.is_note_in_scale(Note::C), true)
                        }
                    })
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("B{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::B))
                    .style({
                        move |theme, status| {
                            Self::button_style(&mut self, theme, status, Color::WHITE, self.is_note_in_scale(Note::C), true)
                        }
                    })
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
                    .style({
                        move |theme, status| {
                            Self::button_style(&mut self, theme, status, Color::BLACK, self.is_note_in_scale(Note::C), true)
                        }
                    })
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
                    .style({
                        move |theme, status| {
                            Self::button_style(&mut self, theme, status, Color::BLACK, self.is_note_in_scale(Note::C), true)
                        }
                    })
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
                        .style({
                            let buttons = buttons_pressed.clone();
                            move |theme, status| {
                                let is_pressed = *buttons.get(&Note::Fsharp).unwrap_or(&false);
                                button_style(theme, status, Color::BLACK, self.is_note_in_scale(Note::Fsharp), is_pressed)
                            }
                        })
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
                        .style({
                            let buttons = buttons_pressed.clone();
                            move |theme, status| {
                                let is_pressed = *buttons.get(&Note::Gsharp).unwrap_or(&false);
                                button_style(theme, status, Color::BLACK, self.is_note_in_scale(Note::Gsharp), is_pressed)
                            }
                        })
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
                        .style({
                            let buttons = buttons_pressed.clone();
                            move |theme, status| {
                                let is_pressed = *buttons.get(&Note::Asharp).unwrap_or(&false);
                                button_style(theme, status, Color::BLACK, self.is_note_in_scale(Note::Asharp), is_pressed)
                            }
                        })
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
                        button(text("Start recording")).on_press(Message::ToggleRecoring)
                    },
                    text(format!("Time recorded: {:.2}s",  self.time_elapsed)),
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

            widget::column![   
                widget::row!(
                    text("Volume:"),
                    slider(
                        0.0..=100.0,
                        self.volume,
                        Message::VolumeChange
                    ),
                    text(format!("{}%",  self.volume)),
                ).spacing(10),
            ]
            
            ]
        ).into()
    }
}