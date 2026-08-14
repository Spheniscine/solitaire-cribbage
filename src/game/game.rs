use std::time::Duration;

use rand::{Rng, seq::SliceRandom};
use strum::IntoEnumIterator;
use serde::{Deserialize, Serialize};

use crate::game::{BitMask, Board, BoardPos, Card, DECK_SIZE, DepotRole, RANK_JACK, RANKS, RankScore, ScoreBoard, Skin, Suit};

use super::StackScore;

pub fn stack_total(stack: &[Card]) -> u8 {
    stack.iter().map(|card| card.value()).sum()
}

impl ScoreBoard {
    pub fn score(&mut self, stack: &[Card]) {
        self.total = stack_total(stack);
        if stack.len() == 1 && stack[0].rank == RANK_JACK {
            self.stack_score = Some(StackScore::Jack);
        } else if self.total == 15 {
            self.stack_score = Some(StackScore::Total15);
        } else if self.total == 31 {
            self.stack_score = Some(StackScore::Total31);
        } else {
            self.stack_score = None;
        }
        if let Some(s) = self.stack_score { self.score += s.value(); }

        self.rank_score = None;
        if stack.len() >= 2 {
            let top_rank = stack[stack.len() - 1].rank;
            let kind = stack.iter().rev().take_while(|card| card.rank == top_rank).count();
            if kind >= 2 { self.rank_score = Some(RankScore::Kind(kind)); }
            else {
                let mut lo = top_rank as usize;
                let mut hi = lo;
                let mut mask = BitMask::single(lo);
                for i in 2..=stack.len() {
                    let rank = stack[stack.len() - i].rank as usize;
                    if mask.contains(rank) { break; }
                    mask = mask.flip(rank);
                    lo = lo.min(rank);
                    hi = hi.max(rank);

                    if i >= 3 && hi + 1 - lo == i {
                        self.rank_score = Some(RankScore::Run(i))
                    }
                }
            }
        }
        if let Some(s) = self.rank_score { self.score += s.value(); }
    }
}

pub const STACK_LIMIT: u8 = 31;
pub const SCORE_GOAL: i32 = 61;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameStatus {
    pub is_won: bool,
    pub is_playable: bool,
}

impl Board {
    pub fn update_score(&mut self) {
        self.score_board.score(&self.depots[DepotRole::Stack.id(0)]);
    }
    pub fn is_playable(&self, pos: BoardPos) -> bool {
        let Some(role) = DepotRole::role(pos.depot_index) else {return false};
        match role {
            DepotRole::Tableau => {
                let depot = &self.depots[pos.depot_index];
                !depot.is_empty() && pos.card_index == depot.len() - 1 && 
                    self.score_board.total + depot[pos.card_index].value() <= STACK_LIMIT
            },
            DepotRole::Stack => false,
            DepotRole::Discard => DepotRole::Tableau.range().any(|i| !self.depots[i].is_empty()) && 
                DepotRole::Tableau.range().all(|i| !self.is_playable(self.last_pos(i))),
        }
    }
}

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ActionRecord {
    pos1: BoardPos,
    pos2: BoardPos,
    score_board: ScoreBoard,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ScreenState {
    #[default] Game, 
    Settings, Help,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub undo_stack: Vec<usize>,
    pub already_won: bool,
    pub num_wins: i32,
    pub high_score: i32,

    pub screen_state: ScreenState,

    pub allow_undo: bool,
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS {
            for suit in Suit::iter() {
                deck.push(Card { rank, suit });
            }
        }

        deck.shuffle(rng);
        deck
    }

    pub fn init() -> Self {
        let mut res = Self {
            board: Board::empty(),
            deal: vec![],
            animation_key: 0,
            history: vec![],
            undo_stack: vec![],
            already_won: false,
            num_wins: 0,
            high_score: 0,
            screen_state: ScreenState::Game,
            allow_undo: true,
            skin: Skin::default(),
        };

        res.new_game();
        res
    }

    pub fn new_game(&mut self) {
        let deal = Self::new_deal(&mut rand::rng());
        self.board = Board::from_deal(&deal);
        self.deal = deal;
        self.history.clear();
        self.undo_stack.clear();
        self.already_won = false;

        // // test for display of stack
        // for i in 1..=13 {
        //     let card = Card { rank: i, suit: Suit::Spades };
        //     self.board.depots[DepotRole::Stack.id(0)].push(card);
        // }

        // if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.undo_stack.is_empty()
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        self.board.update_score();
        self.high_score = self.high_score.max(self.board.score_board.score);

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            // self.check_auto_moves();
        }
        
        // if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn game_status(&self) -> GameStatus {
        GameStatus {
            is_won: self.is_won(),
            is_playable: DepotRole::Tableau.range().any(|i| !self.board.depots[i].is_empty()),
        }
    }

    pub fn is_won(&self) -> bool {
        self.board.score_board.score >= SCORE_GOAL
    }

    fn do_move_raw(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.board.do_move(pos1, pos2);
        self.history.push(ActionRecord { pos1, pos2, score_board: self.board.score_board })
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if !self.board.is_playable(pos) { return; }

        let history_len = self.history.len();
        match DepotRole::role(pos.depot_index).unwrap() {
            DepotRole::Tableau => {
                self.do_move_raw(pos, self.board.top_pos(DepotRole::Stack.id(0)));
            },
            DepotRole::Stack => { return; },
            DepotRole::Discard => {
                self.do_move_raw(BoardPos::new(DepotRole::Stack.id(0), 0), 
                    self.board.top_pos(DepotRole::Discard.id(0)));
            },
        }

        self.undo_stack.push(history_len);
    }
}