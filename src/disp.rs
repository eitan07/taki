use once_cell::unsync::Lazy;
use regex::*;
use std::{collections::HashMap, fs, io::{self, ErrorKind}, path::Path};

use crate::Card;

const KEY_SECTION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[(.+)\]$").unwrap());

#[derive(Debug)]
pub struct CardSet {
    set: HashMap<String, String>,
}

impl CardSet {
    pub fn get_card_by_name(&self, name: &str) -> Result<String, io::Error> {
        if let Some(card) = self.set.get(name) {
            return Ok(card.clone())
        }

        Err(io::Error::new(ErrorKind::NotFound, "Card not found in this set! Are you sure it was loaded correctly with no typos?"))
    }
    pub fn get_card_by_instance(&self, card: &Card) -> Result<String, io::Error> {
        match card {
            Card::Numeric(1, _) => self.get_card_by_name("Card1"),
            Card::Numeric(2, _) => self.get_card_by_name("Card2"),
            Card::Numeric(3, _) => self.get_card_by_name("Card3"),
            Card::Numeric(4, _) => self.get_card_by_name("Card4"),
            Card::Numeric(5, _) => self.get_card_by_name("Card5"),
            Card::Numeric(6, _) => self.get_card_by_name("Card6"),
            Card::Numeric(7, _) => self.get_card_by_name("Card7"),
            Card::Numeric(8, _) => self.get_card_by_name("Card8"),
            Card::Numeric(9, _) => self.get_card_by_name("Card9"),
            Card::Stop(_) => self.get_card_by_name("Stop"),
            Card::ChangeDir(_) => self.get_card_by_name("ChangeDir"),
            Card::Plus(_) => self.get_card_by_name("Plus"),
            Card::Kah2(_) => self.get_card_by_name("Take2"),
            Card::Taki(_) => self.get_card_by_name("Taki"),
            Card::ChangeColor => self.get_card_by_name("ChangeCol"),
            Card::King => self.get_card_by_name("King"),
            Card::SuperTaki => self.get_card_by_name("SuperTaki"),
            _ => Err(io::Error::new(ErrorKind::NotFound, "Card not found in this set! Are you sure it was loaded correctly with no typos?"))
        }
    }
}

pub struct CardSetLoader;

impl CardSetLoader {
    pub fn load_from_file<F: AsRef<Path>>(f: F) -> Result<CardSet, io::Error> {
        let mut hm: HashMap<String, String> = HashMap::new();
        let binding = fs::read_to_string(f)?;
        let lines = binding.as_str().lines().collect::<Vec<&str>>();
        
        let mut val: String = String::new();
        let mut key = "";
        for i in 0..lines.len() {
            if let Some(cap) = KEY_SECTION_REGEX.captures(&lines[i]) {
                if let Some(cap) = cap.get(1) {                   
                    if key != "" {
                        hm.insert(key.to_string(), val);
                    }
                    key = cap.as_str();
                    val = String::new();
                }
            } else {
                val += &lines[i];
                val += "\n";
            }
        }

        Ok(CardSet { set: hm })
    }
}
