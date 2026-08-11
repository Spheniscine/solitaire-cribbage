use crate::game::{BitMask, Board, BoardPos, Card, DepotRole, RANK_JACK, RankScore, ScoreBoard};

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

impl Board {
    pub fn update_score(&mut self) {
        self.score_board.score(&self.depots[DepotRole::Stack.id(0)]);
    }
    pub fn is_playable(&self, pos: BoardPos) -> bool {
        let Some((role, index)) = DepotRole::role_and_subindex(pos.depot_index) else {return false};
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
