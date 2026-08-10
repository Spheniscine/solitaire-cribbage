use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, FromRepr};

use crate::game::Suit;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default)]
pub enum RankSkin {
    #[default]
    Traditional,
}

pub const RANK_ACE: u8 = 1;
pub const RANK_JACK: u8 = 11;
pub const RANK_QUEEN: u8 = 12;
pub const RANK_KING: u8 = 13;

impl RankSkin {
    pub fn rank_text(self, rank: u8) -> String {
        match self {
            RankSkin::Traditional => {
                match rank {
                    RANK_ACE => String::from("A"),
                    RANK_JACK => String::from("J"),
                    RANK_QUEEN => String::from("Q"),
                    RANK_KING => String::from("K"),
                    _ => rank.to_string(),
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum SuitSkin {
    #[default]
    Animals,
    Shapes,
    Traditional,
}

impl SuitSkin {
    pub fn suit_symbol(self, suit: Suit) -> &'static str {
        match self {
            SuitSkin::Animals => {
                match suit {
                    Suit::Clubs => "🐰",
                    Suit::Diamonds => "🦁",
                    Suit::Hearts => "🦊",
                    Suit::Spades => "🐧",
                }
            },
            SuitSkin::Shapes => {
                match suit {
                    Suit::Clubs => "▲",
                    Suit::Diamonds => "⬥",
                    Suit::Hearts => "●",
                    Suit::Spades => "★",
                }
            }
            SuitSkin::Traditional => {
                match suit {
                    Suit::Clubs => "♣",
                    Suit::Diamonds => "♦︎",
                    Suit::Hearts => "♥",
                    Suit::Spades => "♠",
                }
            }
        }
    }

    pub fn font(self) -> &'static str {
        match self {
            SuitSkin::Animals => "'Noto Color Emoji'",
            SuitSkin::Shapes => "'Noto Sans Symbols 2'",
            SuitSkin::Traditional => "KaTeX_Suits", // links to custom version of Katex/MLModern that has filled card suits
        }
    }
}

const COLOR_AMBER: [&str; 2] = ["#b70", "#ffb433"];
const COLOR_GREEN: [&str; 2] = ["#062", "#00ff55"];
const COLOR_RED: [&str; 2] = ["#f00", "#ff8888"];
const COLOR_BLUE: [&str; 2] = ["#00d", "#aaaaff"];
const COLOR_BLACK: [&str; 2] = ["#000", "#fff"];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorMode {
    #[default] Dark, Light
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorSkin {
    #[default]
    #[strum(to_string = "One color")]
    OneColor,
    #[strum(to_string = "Two colors")]
    TwoColor,
    #[strum(to_string = "Four colors")]
    FourColor,
}

impl ColorSkin {
    pub fn color(self, suit: Suit, mode: ColorMode) -> &'static str {
        let res = match self {
            ColorSkin::FourColor => {
                // Use Spectrum Bridge colors - better distinction between reddish/warm and blackish/cool colors for
                // solitaires that care about that
                match suit {
                    Suit::Clubs => COLOR_GREEN,
                    Suit::Diamonds => COLOR_AMBER,
                    Suit::Hearts => COLOR_RED,
                    Suit::Spades => COLOR_BLUE,
                }
            },
            ColorSkin::TwoColor => {
                match suit {
                    Suit::Clubs | Suit::Spades => COLOR_BLACK,
                    Suit::Diamonds | Suit::Hearts => COLOR_RED,
                }
            },
            ColorSkin::OneColor => COLOR_BLACK,
        };
        res[mode as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub struct Skin {
    #[serde(skip)]
    pub ranks: RankSkin,
    pub suits: SuitSkin,
    pub colors: ColorSkin,
}