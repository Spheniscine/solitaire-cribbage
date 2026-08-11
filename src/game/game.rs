use crate::game::{BitMask, Card, RANK_JACK, RankScore, ScoreBoard};

use super::StackScore;

pub fn stack_total(stack: &[Card]) -> u8 {
    stack.iter().map(|card| card.value()).sum()
}

impl ScoreBoard {
    pub fn score(&mut self, stack: &[Card]) {
        let total = stack_total(stack);
        if stack.len() == 1 && stack[0].rank == RANK_JACK {
            self.stack_score = Some(StackScore::Jack);
        } else if total == 15 {
            self.stack_score = Some(StackScore::Total15);
        } else if total == 31 {
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