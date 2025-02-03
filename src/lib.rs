pub mod disp;

use rand::{seq::SliceRandom, thread_rng, Rng};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use std::{
    collections::VecDeque,
    fmt::Debug
};

#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub cards: Stack<Card>
}

impl Player {
    pub fn new() -> Self {
        Self {
            id: thread_rng().gen_range(0..=99),
            cards: Stack::new(),
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
    King
}

// impl Display for Card {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self {
//             Card::Numeric(u8, CardColor) => {}
//             Card::Taki(CardColor) => {}
//             Card::Plus(CardColor) => {}
//             Card::ChangeDir(CardColor) => {}
//             Card::Stop(CardColor) => {}
//             Card::Kah2(CardColor) => {}
//             Card::ChangeColor => {}
//             Card::SuperTaki => {}
//             Card::King => {}
//         }

//         todo!()
//     }
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CardColor {
    Red,
    Green,
    Blue,
    Yellow
}

impl CardColor {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..4) {
            1 => Self::Red,
            2 => Self::Green,
            3 => Self::Blue,
            4 => Self::Yellow,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Stack<T>(VecDeque<T>, usize);

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
        T: Clone
    {
        let mut v: Vec<T> = self.0.clone().into();

        for _ in 0..times {
            v.shuffle(&mut thread_rng());
        }

        self.0 = v.into();
    }

    pub fn pop_nth(&mut self, n: usize) -> Vec<Option<T>> {
        let mut v: Vec<Option<T>> = Vec::new();

        for _ in 0..n {
            v.push(self.pop());
        }

        v
    }

    pub fn push_times(&mut self, val: T, times: usize)
    where
        T: Copy
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

pub struct CardWidget<'a> {
    pub card: &'a Card
}

impl<'a> Widget for CardWidget<'a> {
    fn render(self, _area: Rect, _buf: &mut Buffer)
    where
        Self: Sized
    {
    }
}