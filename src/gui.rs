use iced::{alignment, border::Radius, font::Weight, widget::{self, MouseArea, button, checkbox, container, pick_list, slider, text, text_input, Space}, Border, Color, Font, Length, Theme};
use crate::{Message, Note, Program, Chord};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::string::ToString;

// allows Note to be converted to String
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
    fn button_style(_theme: &Theme, _status: button::Status, note_color: Color, in_scale: bool, pressed: bool) -> button::Style {
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

        button::Style {
            background: Some(iced::Background::Color(color)),
            text_color: if note_color == Color::BLACK { Color::WHITE } else { Color::BLACK },
            border: Border { radius: Radius::from(2), ..Border::default() },
            ..button::Style::default()
        }
    }

    pub fn get_ui_information(&self, buttons_pressed: Arc<Mutex<HashMap<Note, bool>>>) -> iced::widget::Container<Message> {
        let accidental_height = 132.6;
        let accidental_width = 63.75;
        let natural_height = 255.0;
        let natural_width = 85.0;

        container(widget::column![
            widget::row!(
                text("Note Length"),
                slider(
                    1.0..=5.0,
                    self.note_length,
                    Message::NoteLengthChange
                ),
                text(format!("Length: {}", Self::get_note_length(self.note_length))),
            ).spacing(10).padding(5),

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
            ).spacing(10).padding(5),

            widget::row!(
                checkbox("Play major scale triads", self.play_chords)
                    .on_toggle(|_| Message::PlayChords)
                    .spacing(10),
                
                checkbox("Play asynchronously", self.play_async)
                    .on_toggle(|_| Message::PlayAsync)
                    .spacing(10),
            ).spacing(20).padding(5),

            Space::with_height(20), 

            widget::row!(
                text("Octave:"),
                button("+").on_press(Message::OctaveChange((self.octave + 1.0).min(6.0))),
                button("-").on_press(Message::OctaveChange((self.octave - 1.0).max(0.0))),
            ).spacing(10).padding(5),

            Space::with_height(50), 

            widget::row!(
                widget::stack!(
                    widget::row!(
                        MouseArea::new(
                            button(text(format!("C{}", self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::C).unwrap_or(&false);
                                
                                    Self::button_style(theme, status, Color::WHITE, Chord::is_note_in_scale(&self, Note::C), is_pressed)
                                }
                            })
                            .width(Length::Fixed(natural_width))
                            .height(Length::Fixed(natural_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::C, true))
                        .on_release(Message::EndPlaying(Note::C))
                        .on_exit(Message::EndPlaying(Note::C)),

                        MouseArea::new(
                            button(text(format!("D{}", self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({                            
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::D).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::WHITE, Chord::is_note_in_scale(&self, Note::D), is_pressed)
                                }
                            })
                            .width(Length::Fixed(natural_width))
                            .height(Length::Fixed(natural_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::D, true))
                        .on_release(Message::EndPlaying(Note::D))
                        .on_exit(Message::EndPlaying(Note::D)),


                        MouseArea::new(
                            button(text(format!("E{}", self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::E).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::WHITE, Chord::is_note_in_scale(&self, Note::E), is_pressed)
                                }
                            })
                            .width(Length::Fixed(natural_width))
                            .height(Length::Fixed(natural_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::E, true))
                        .on_release(Message::EndPlaying(Note::E))
                        .on_exit(Message::EndPlaying(Note::E)),

                        MouseArea::new(
                            button(text(format!("F{}", self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::F).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::WHITE, Chord::is_note_in_scale(&self, Note::F), is_pressed)
                                }
                            })
                            .width(Length::Fixed(natural_width))
                            .height(Length::Fixed(natural_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::F, true))
                        .on_release(Message::EndPlaying(Note::F))
                        .on_exit(Message::EndPlaying(Note::F)),

                        MouseArea::new(
                            button(text(format!("G{}", self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::G).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::WHITE, Chord::is_note_in_scale(&self, Note::G), is_pressed)
                                }
                            })
                            .width(Length::Fixed(natural_width))
                            .height(Length::Fixed(natural_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::G, true))
                        .on_release(Message::EndPlaying(Note::G))
                        .on_exit(Message::EndPlaying(Note::G)),

                        MouseArea::new(
                            button(text(format!("A{}", self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::A).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::WHITE, Chord::is_note_in_scale(&self, Note::A), is_pressed)
                                }
                            })
                            .width(Length::Fixed(natural_width))
                            .height(Length::Fixed(natural_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::A, true))
                        .on_release(Message::EndPlaying(Note::A))
                        .on_exit(Message::EndPlaying(Note::A)),

                        MouseArea::new(
                            button(text(format!("B{}", self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::B).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::WHITE, Chord::is_note_in_scale(&self, Note::B), is_pressed)
                                }
                            })
                            .width(Length::Fixed(natural_width))
                            .height(Length::Fixed(natural_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::B, true))
                        .on_release(Message::EndPlaying(Note::B))
                        .on_exit(Message::EndPlaying(Note::B)),
                                    
                                    ).spacing(2).padding(5),
                
                    widget::row!(
                        Space::with_width(59.5),
                        MouseArea::new(
                            button(text(format!("Db{}\nC#{}", self.octave, self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::Csharp).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::BLACK, Chord::is_note_in_scale(&self, Note::Csharp), is_pressed)
                                }
                            })
                            .width(Length::Fixed(accidental_width))
                            .height(Length::Fixed(accidental_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::Csharp, true))
                        .on_release(Message::EndPlaying(Note::Csharp))
                        .on_exit(Message::EndPlaying(Note::Csharp)),

                        Space::with_width(34.0),
                        MouseArea::new(
                            button(text(format!("Eb{}\nD#{}", self.octave, self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::Dsharp).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::BLACK, Chord::is_note_in_scale(&self, Note::Dsharp), is_pressed)
                                }
                            })
                            .width(Length::Fixed(accidental_width))
                            .height(Length::Fixed(accidental_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::Dsharp, true))
                        .on_release(Message::EndPlaying(Note::Dsharp))
                        .on_exit(Message::EndPlaying(Note::Dsharp)),

                        Space::with_width(93.5),
                        MouseArea::new(
                            button(text(format!("Gb{}\nF#{}", self.octave, self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::Fsharp).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::BLACK, Chord::is_note_in_scale(&self, Note::Fsharp), is_pressed)
                                }
                            })
                            .width(Length::Fixed(accidental_width))
                            .height(Length::Fixed(accidental_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::Fsharp, true))
                        .on_release(Message::EndPlaying(Note::Fsharp))
                        .on_exit(Message::EndPlaying(Note::Fsharp)),

                        Space::with_width(34.0),
                        MouseArea::new(
                            button(text(format!("Ab{}\nG#{}", self.octave, self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::Gsharp).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::BLACK, Chord::is_note_in_scale(&self, Note::Gsharp), is_pressed)
                                }
                            })
                            .width(Length::Fixed(accidental_width))
                            .height(Length::Fixed(accidental_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::Gsharp, true))
                        .on_release(Message::EndPlaying(Note::Gsharp))
                        .on_exit(Message::EndPlaying(Note::Gsharp)),

                        Space::with_width(34.0),
                        MouseArea::new(
                            button(text(format!("Bb{}\nA#{}", self.octave, self.octave))
                                .size(24)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Bottom)
                                .font(Font { weight: Weight::Bold, ..Default::default() })
                            )
                            .style({
                                let buttons = buttons_pressed.clone();
                                move |theme, status| {
                                    let is_pressed = *buttons.lock().unwrap().get(&Note::Asharp).unwrap_or(&false);
                                    Self::button_style(theme, status, Color::BLACK, Chord::is_note_in_scale(&self, Note::Asharp), is_pressed)
                                }
                            })
                            .width(Length::Fixed(accidental_width))
                            .height(Length::Fixed(accidental_height))
                            .padding(5)
                        )
                        .on_press(Message::Play(Note::Asharp, true))
                        .on_release(Message::EndPlaying(Note::Asharp))
                        .on_exit(Message::EndPlaying(Note::Asharp)),
                    ).spacing(0),
                ),
                Space::with_width(10), 
                widget::column! [ 
                    text("How do I use this?")
                        .width(Length::Fill)
                        .align_x(alignment::Horizontal::Center)
                        .size(28),
                    text("1. Selecting your Note Length, this can be any value from Whole to Sixteenth\n2. Selecting your BPM, or the amount of beats per minute, this can be any value from 1 to 300\n3. Play notes; you can either press the note on the screen or use the keyboard keys to control it\n4. (optional) Record; you can press the record button to export the music as a midi file")
                        .width(Length::Fill)
                        .align_x(alignment::Horizontal::Center)
                        .size(16),
                ]
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

            ].spacing(20).padding(5),

            Space::with_height(20), 

            widget::column![
                widget::row!( 
                    text("Select Major Scale: "),
                    pick_list(
                        Note::ALL,
                        self.selected_scale.clone(),
                        Message::Scale
                    ).width(Length::Fixed(150.0)),  
                )
            ].spacing(20).padding(5),

            Space::with_height(20), 

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
            ].spacing(20).padding(5)
            
            ]
        ).into()
    }
}