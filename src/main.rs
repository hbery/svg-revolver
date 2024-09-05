use std::path::Path;
use iced::{
    alignment::{
        Alignment,
        Horizontal,
        Vertical
    },
    keyboard::{
        self,
        key::Named
    },
    widget::{
        button,
        column,
        text,
        row,
        Svg
    },
    Application,
    Command,
    Length,
    Renderer,
    Settings,
    Theme,
};

mod constants;
mod dir;

use constants::{TITLE, SVG_DIR};
use crate::dir::safe_read_svg_dir;

#[derive(Clone, Debug)]
struct Revolver {
    numpics: usize,
    current: usize,
    svgs:    Vec<String>,
}

trait Swipable {
    fn left(&self) -> usize;
    fn first(&self) -> usize;
    fn right(&self) -> usize;
    fn last(&self) -> usize;
}

impl Swipable for Revolver {
    fn left(&self) -> usize {
        match self.current {
            x if x == 0 => return self.numpics - 1,
            _ => return self.current - 1,
        }
    }

    fn first(&self) -> usize {
        return 0
    }

    fn right(&self) -> usize {
        match self.current {
            x if x == (self.numpics - 1) => 0,
            _ => self.current + 1,
        }
    }

    fn last(&self) -> usize {
        return self.numpics - 1;
    }
}

#[derive(Clone, Copy, Debug)]
enum RevolverMessage {
    ChangeLeft,
    ChangeToStart,
    ChangeRight,
    ChangeToEnd,
    WindowClose,
}

impl Application for Revolver {
    type Message  = RevolverMessage;
    type Theme    = Theme;
    type Executor = iced::executor::Default;
    type Flags    = ();


    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let svg_array = safe_read_svg_dir(Path::new(SVG_DIR));

        return (
            Revolver{
                numpics: svg_array.len(),
                current: 0,
                svgs:    svg_array,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        return TITLE.to_string();
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        keyboard::on_key_press(|k, modifiers| match k {
            keyboard::Key::Named(Named::ArrowLeft)  => return Some(
                if modifiers.shift() { RevolverMessage::ChangeToStart }
                else                 { RevolverMessage::ChangeLeft    }
            ),
            keyboard::Key::Character(c) if c == "h" => return Some(
                if modifiers.shift() { RevolverMessage::ChangeToStart }
                else                 { RevolverMessage::ChangeLeft    }
            ),
            keyboard::Key::Named(Named::ArrowRight) => return Some(
                if modifiers.shift() { RevolverMessage::ChangeToEnd }
                else                 { RevolverMessage::ChangeRight }
            ),
            keyboard::Key::Character(c) if c == "l" => return Some(
                if modifiers.shift() { RevolverMessage::ChangeToEnd }
                else                 { RevolverMessage::ChangeRight }
            ),
            keyboard::Key::Named(Named::Escape)     => return Some(RevolverMessage::WindowClose),
            keyboard::Key::Character(c) if c == "q" => return Some(RevolverMessage::WindowClose),
            _ => return None,
        })
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, Renderer> {
        column![
            Svg::from_path(&self.svgs[self.current]).height(Length::FillPortion(17)),
            row![
                button(text("Left")
                        .vertical_alignment(Vertical::Center)
                        .horizontal_alignment(Horizontal::Center))
                    .height(Length::FillPortion(2))
                    .width(Length::FillPortion(1))
                    .on_press(RevolverMessage::ChangeLeft),
                button(text("Right")
                        .vertical_alignment(Vertical::Center)
                        .horizontal_alignment(Horizontal::Center))
                    .height(Length::FillPortion(2))
                    .width(Length::FillPortion(1))
                    .on_press(RevolverMessage::ChangeRight),
            ],
            text(format!(
            "{l_mod} + {l_arr}/{l_let} - Start | {l_arr}/{l_let} - Left | {r_arr}/{r_let} - Right | {r_mod} + {r_arr}/{r_let} - End | {q_esc}/{q_let} - Quit",
                l_mod = "<SHIFT>",
                r_mod = "<SHIFT>",
                l_arr = "=>",
                l_let = "<H>",
                r_arr = "<=",
                r_let = "<L>",
                q_esc = "<ESC>",
                q_let = "<Q>"
            ))
                .height(Length::FillPortion(1))
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Center)
        ]
        .padding(5)
        .align_items(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            RevolverMessage::ChangeLeft => {
                self.current = self.left();
                Command::none()
            }
            RevolverMessage::ChangeRight => {
                self.current = self.right();
                Command::none()
            }
            RevolverMessage::ChangeToStart => {
                self.current = self.first();
                Command::none()
            }
            RevolverMessage::ChangeToEnd => {
                self.current = self.last();
                Command::none()
            }
            RevolverMessage::WindowClose => {
                std::process::exit(0)
            }
        }
    }
}

fn main() -> iced::Result {
    return Revolver::run(Settings::default());
}
