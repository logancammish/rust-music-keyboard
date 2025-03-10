use iced::{alignment, border::Radius, font::Weight, widget::{self, button, checkbox, container, slider, text, text_input, Space}, Background, Border, Color, Font, Length, Theme}; // Add Font
use crate::{Message, Note, Program};

fn button_style(_theme: &Theme, _status: button::Status, note_color: Color) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(note_color)),
        text_color: if note_color == Color::BLACK { Color::WHITE } else { Color::BLACK },
        border: Border { radius: Radius::from(2), ..Border::default() }, // Add border with radius
        ..button::Style::default()
    }
}

impl Program {
    pub fn get_ui_information(&self) -> iced::widget::Container<'static, Message> {
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
                checkbox("Play triad ascending appregios", self.play_chords)
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
                        .style(|theme, status| button_style(theme, status, Color::WHITE))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("D{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::D))
                        .style(|theme, status| button_style(theme, status, Color::WHITE))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("E{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::E))
                        .style(|theme, status| button_style(theme, status, Color::WHITE))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("F{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::F))
                        .style(|theme, status| button_style(theme, status, Color::WHITE))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("G{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::G))
                        .style(|theme, status| button_style(theme, status, Color::WHITE))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("A{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::A))
                        .style(|theme, status| button_style(theme, status, Color::WHITE))
                        .width(Length::Fixed(85.0))
                        .height(Length::Fixed(255.0))
                        .padding(5),
                    button(text(format!("B{}", self.octave))
                        .size(24)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                        .font(Font { weight: Weight::Bold, ..Default::default() })
                    ).on_press(Message::Play(Note::B))
                        .style(|theme, status| button_style(theme, status, Color::WHITE))
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
                        .style(|theme, status| button_style(theme, status, Color::BLACK))
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
                        .style(|theme, status| button_style(theme, status, Color::BLACK))
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
                        .style(|theme, status| button_style(theme, status, Color::BLACK))
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
                        .style(|theme, status| button_style(theme, status, Color::BLACK))
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
                        .style(|theme, status| button_style(theme, status, Color::BLACK))
                        .width(Length::Fixed(63.75))
                        .height(Length::Fixed(132.6))
                        .padding(5),
                ).spacing(0),
            ),

            Space::with_height(50), // Add vertical space

            widget::column![
                widget::row!(
                    if self.is_recording {
                        button(text("Stop recording")).on_press(Message::ToggleRecoring)
                    } else {
                        button(text("Record")).on_press(Message::ToggleRecoring)
                    }
                ).spacing(10) // Add spacing between buttons
            ].spacing(20)
        ]).into() // Add missing closing delimiters
    }
}
