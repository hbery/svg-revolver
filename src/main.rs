use std::path::Path;
use iced::{
    alignment::{
        Alignment,
        Horizontal,
        Vertical
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
    fn right(&self) -> usize;
}

impl Swipable for Revolver {
    fn left(&self) -> usize {
        match self.current {
            x if x == 0 => return self.numpics - 1,
            _ => return self.current - 1,
        }
    }

    fn right(&self) -> usize {
        match self.current {
            x if x == (self.numpics - 1) => 0,
            _ => self.current + 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum RevolverMessage {
    ChangeLeft,
    ChangeRight,
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

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, Renderer> {
        column![
            Svg::from_path(&self.svgs[self.current]).height(Length::FillPortion(10)),
            row![
                button(text("Left")
                        .vertical_alignment(Vertical::Center)
                        .horizontal_alignment(Horizontal::Center))
                    .height(Length::FillPortion(1))
                    .width(Length::FillPortion(1))
                    .on_press(RevolverMessage::ChangeLeft),
                button(text("Right")
                        .vertical_alignment(Vertical::Center)
                        .horizontal_alignment(Horizontal::Center))
                    .height(Length::FillPortion(1))
                    .width(Length::FillPortion(1))
                    .on_press(RevolverMessage::ChangeRight)
            ],
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
        }
    }
}

fn main() -> iced::Result {
    return Revolver::run(Settings::default());
}
