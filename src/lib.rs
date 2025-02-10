pub mod disp;

use disp::CardSet;
use rand::{seq::SliceRandom, thread_rng, Rng};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::Widget
};
use std::{
    collections::{vec_deque::IntoIter, VecDeque},
    fmt::Debug
};

#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub cards: Stack<Card>,
    pub cards_count: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            id: thread_rng().gen_range(0..=99),
            cards: Stack::new(),
            cards_count: 0usize,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Card {
    Numeric(u8, CardColor),
    Taki(CardColor),
    Plus(CardColor),
    ChangeDir(CardColor),
    Stop(CardColor),
    Kah2(CardColor),
    ChangeColor,
    SuperTaki,
    King,
    CardsBack,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CardColor {
    Red,
    Green,
    Blue,
    Yellow,
}

impl CardColor {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..4) {
            1 => Self::Red,
            2 => Self::Green,
            3 => Self::Blue,
            4 => Self::Yellow,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Stack<T>(VecDeque<T>, usize);

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// impl<T> Iterator for Stack<T>
// where T: Clone + Copy {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(t) = self.0.iter().next() {
//             Some(*t)
//         } else {
//             None
//         }
//     }
// }

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(VecDeque::new(), 0)
    }

    pub fn push(&mut self, val: T) {
        self.1 += 1;
        self.0.push_front(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.1 -= 1;
        self.0.pop_front()
    }

    pub fn shuffle(&mut self, times: usize)
    where
        T: Clone,
    {
        let mut v: Vec<T> = self.0.clone().into();

        for _ in 0..times {
            v.shuffle(&mut thread_rng());
        }

        self.0 = v.into();
    }

    pub fn push_times(&mut self, val: T, times: usize)
    where
        T: Copy,
    {
        for _ in 0..times {
            self.1 += 1;
            self.0.push_front(val);
        }
    }

    pub fn len(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Clone)]
enum CardStyle {
    NormalStyle(Style),
    Rainbow([Style; 6]),
    BoldRainbow([Style; 6]),
}

#[derive(Debug, Clone)]
pub struct CardWidget {
    card: Vec<String>,
    style: CardStyle,
}

impl<'a> CardWidget {
    pub fn new(card_set: &CardSet, card: &Card) -> Self {
        let style = Style::new();
        let style = match *card {
            Card::Numeric(_, CardColor::Red) => CardStyle::NormalStyle(style.fg(Color::Red)),
            Card::Numeric(_, CardColor::Green) => CardStyle::NormalStyle(style.fg(Color::Green)),
            Card::Numeric(_, CardColor::Blue) => CardStyle::NormalStyle(style.fg(Color::Blue)),
            Card::Numeric(_, CardColor::Yellow) => CardStyle::NormalStyle(style.fg(Color::Yellow)),
            Card::ChangeDir(CardColor::Red) => CardStyle::NormalStyle(style.fg(Color::Red)),
            Card::ChangeDir(CardColor::Green) => CardStyle::NormalStyle(style.fg(Color::Green)),
            Card::ChangeDir(CardColor::Blue) => CardStyle::NormalStyle(style.fg(Color::Blue)),
            Card::ChangeDir(CardColor::Yellow) => CardStyle::NormalStyle(style.fg(Color::Yellow)),

            Card::Kah2(CardColor::Red) => CardStyle::NormalStyle(style.fg(Color::Red)),
            Card::Kah2(CardColor::Green) => CardStyle::NormalStyle(style.fg(Color::Green)),
            Card::Kah2(CardColor::Blue) => CardStyle::NormalStyle(style.fg(Color::Blue)),
            Card::Kah2(CardColor::Yellow) => CardStyle::NormalStyle(style.fg(Color::Yellow)),

            Card::Plus(CardColor::Red) => CardStyle::NormalStyle(style.fg(Color::Red)),
            Card::Plus(CardColor::Green) => CardStyle::NormalStyle(style.fg(Color::Green)),
            Card::Plus(CardColor::Blue) => CardStyle::NormalStyle(style.fg(Color::Blue)),
            Card::Plus(CardColor::Yellow) => CardStyle::NormalStyle(style.fg(Color::Yellow)),
            Card::Taki(CardColor::Red) => CardStyle::NormalStyle(style.fg(Color::Red)),
            Card::Taki(CardColor::Green) => CardStyle::NormalStyle(style.fg(Color::Green)),
            Card::Taki(CardColor::Blue) => CardStyle::NormalStyle(style.fg(Color::Blue)),
            Card::Taki(CardColor::Yellow) => CardStyle::NormalStyle(style.fg(Color::Yellow)),

            Card::Stop(CardColor::Red) => CardStyle::NormalStyle(style.fg(Color::Red)),
            Card::Stop(CardColor::Green) => CardStyle::NormalStyle(style.fg(Color::Green)),
            Card::Stop(CardColor::Blue) => CardStyle::NormalStyle(style.fg(Color::Blue)),
            Card::Stop(CardColor::Yellow) => CardStyle::NormalStyle(style.fg(Color::Yellow)),

            Card::King => CardStyle::NormalStyle(style.fg(Color::Rgb(255, 127, 0))),

            Card::ChangeColor => {
                let mut colors = [Style::reset(); 6];
                for i in 0..6 {
                    let mut rng = thread_rng();
                    colors[i] = Color::Rgb(
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                    )
                    .into()
                }

                CardStyle::Rainbow(colors)
            }

            Card::SuperTaki => {
                let mut colors = [Style::reset(); 6];
                for i in 0..6 {
                    let mut rng = thread_rng();
                    colors[i] = Style::new().bold().fg(Color::Rgb(
                        rng.gen_range(127..255),
                        rng.gen_range(127..255),
                        rng.gen_range(127..255),
                    ))
                }

                CardStyle::BoldRainbow(colors)
            }

            Card::CardsBack => CardStyle::NormalStyle(Style::new().bg(Color::Gray)),
        };

        let card = card_set.get_card_by_instance(card).unwrap();
        let card_lines = card
            .split("\n")
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>();

        Self {
            card: card_lines,
            style,
        }
    }
}

impl<'a> Widget for CardWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.style {
            CardStyle::Rainbow(colors) => {
                for i in 0..self.card.len() {
                    buf.set_string(area.x, area.y + i as u16, self.card[i].clone(), colors[i]);
                }
            }

            CardStyle::BoldRainbow(colors) => {
                for i in 0..self.card.len() {
                    buf.set_string(area.x, area.y + i as u16, self.card[i].clone(), colors[i]);
                }
            }

            CardStyle::NormalStyle(style) => {
                for i in 0..self.card.len() {
                    buf.set_string(area.x, area.y + i as u16, self.card[i].clone(), style);
                }
            }
        }
    }
}