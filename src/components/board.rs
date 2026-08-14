use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_BORDER_RADIUS_RATIO, CARD_HEIGHT_RATIO, CardComponent, CardFrame, Emoji, Movement, rem}, game::{AnimationAct, AnimationKey, Board, BoardPos, Card, DECK_SIZE, DepotRole, GameStatus, NUM_DEPOTS, RankScore, SCORE_GOAL, ScoreBoard, Skin, StackScore}};

#[component]
pub fn ScoreBoardComponent(
    position: Vec2,
    width: f32,
    score_board: ScoreBoard,
    high_score: i32,
) -> Element {
    let run_length = match score_board.rank_score {
        Some(RankScore::Run(x)) => Some(x),
        _ => None
    };

    let run_length_text = || rsx! {
        if let Some(x) = run_length {
            span {
                color: "#0f0",
                "{x}",
            }
        } else {
            "X"
        }
    };

    rsx! {
        table {
            class: "scoreboard",
            position: "absolute",
            left: rem(position.x),
            top: rem(position.y),
            width: rem(width),

            tr {
                td {
                    "Stack Total: {score_board.total}"
                }
            }

            tr {
                td {
                    " "
                }
            }

            tr {
                class: if score_board.stack_score == Some(StackScore::Jack) {"highlight"},
                td {
                    "Jack First"
                }
                td {
                    "+2"
                }
            }

            tr {
                class: if score_board.stack_score == Some(StackScore::Total15) {"highlight"},
                td {
                    "Stack Total = 15"
                }
                td {
                    "+2"
                }
            }

            tr {
                class: if score_board.stack_score == Some(StackScore::Total31) {"highlight"},
                td {
                    "Stack Total = 31"
                }
                td {
                    "+2"
                }
            }

            tr {
                class: if score_board.rank_score == Some(RankScore::Kind(2)) {"highlight"},
                td {
                    "Pair"
                }
                td {
                    "+2"
                }
            }

            tr {
                class: if score_board.rank_score == Some(RankScore::Kind(3)) {"highlight"},
                td {
                    "Triple"
                }
                td {
                    "+6"
                }
            }

            tr {
                class: if score_board.rank_score == Some(RankScore::Kind(4)) {"highlight"},
                td {
                    "Quad"
                }
                td {
                    "+12"
                }
            }

            tr {
                class: if run_length.is_some() {"highlight"},
                td {
                    "Run (length ", {run_length_text()}, " ≥ 3)"
                }
                td {
                    "+", {run_length_text()}
                }
            }

            tr {
                td {
                    " "
                }
            }

            tr {
                class: "highlight",
                td {
                    "Score"
                }
                td {
                    "{score_board.score}"
                }
            }
            tr {
                class: "highlight",
                td {
                    "Goal"
                }
                td {
                    "{SCORE_GOAL}"
                }
            }
            tr {
                td {
                    "High score"
                }
                td {
                    "{high_score}"
                }
            }
        }
    }
}

#[component]
pub fn BoardComponent(
    position: Vec2,
    board: Board,
    skin: Skin,
    #[props(default)]
    onclick: EventHandler<BoardPos>,
    #[props(default)]
    animation_key: AnimationKey,
    game_status: GameStatus,
    #[props(default)]
    high_score: i32,
) -> Element {
    let card_width = 11.75f32;
    let card_height = card_width * CARD_HEIGHT_RATIO;
    let spacer_x = 1.5f32;
    let spacer_y = 1.5f32;

    let margin_x = 2f32;
    let posr_x = |i: usize| {
        100. - margin_x - card_width - i as f32 * (card_width + spacer_x)
    };
    let margin_y = 2f32;
    let pos_y = |i: usize| {
        margin_y + i as f32 * (card_height + spacer_y)
    };

    let x_card_offset = Vec2::new(card_width / 2., 0.);
    let y_card_offset = Vec2::new(0., card_height / 2.);

    let get_pos = |depot: usize, ord: usize| {
        let (role, index) = DepotRole::role_and_subindex(depot).unwrap();
        use DepotRole::*;
        match role {
            Tableau => 
                Vec2::new(posr_x(Tableau.number_of() - 1 - index), pos_y(1)) + y_card_offset * ord as f32,
            Stack => 
                Vec2::new(margin_x, margin_y) + x_card_offset * ord as f32,
            Discard => 
                Vec2::new(posr_x(0), pos_y(0)),
        }
    };

    let should_disabled_display = |depot: usize, ord: usize| {
        let role = DepotRole::role(depot).unwrap();
        use DepotRole::*;
        match role {
            Tableau => {
                // grey out if there's an animation from this column
                let from_this_col = board.animation_acts.get(0).is_some_and(|act| {
                    match act {
                        AnimationAct::Move(_cards, pos1, _pos2) => {
                            pos1.depot_index == depot
                        },
                    }
                });
                from_this_col || !board.is_playable(BoardPos { depot_index: depot, card_index: ord })
            },
            Stack => 
                false,
            Discard => 
                false,
        }
    };

    let discard_playable = board.is_playable(board.last_pos(DepotRole::Discard.id(0)));

    let get_hint = |depot: usize| {
        let role = DepotRole::role(depot).unwrap();
        match role {
            DepotRole::Discard => Some(
                    rsx!{
                        if !discard_playable {
                            span {
                                font_family: "'Noto Emoji'",
                                "♻"
                            }
                        }
                    }
                ),
            _ => None,
        }
    };

    let is_face_up = |depot: usize| {
        DepotRole::role(depot).unwrap().is_face_up()
    };

    let moving_card = |p1: Vec2, p2: Vec2, card: Card| rsx! {
        Movement {
            src_translate_vec: p1 - p2,
            CardComponent {
                position: p2,
                width: card_width,
                card: card,
                skin,
            }
        }
    };

    let anims = board.animation_acts.iter().enumerate().map(|(i, act)| {
        match act {
            AnimationAct::Move (cards, pos1, pos2) => {
                let mut pos1 = *pos1;
                let mut pos2 = *pos2;

                let nodes = cards.iter().map(move |card| {
                    let p1 = get_pos(pos1.depot_index, pos1.card_index);
                    let p2 = get_pos(pos2.depot_index, pos2.card_index);
                    let res = moving_card(p1, p2, *card);
                    pos1.card_index += 1;
                    pos2.card_index += 1;
                    res
                });

                rsx! {
                    Fragment {
                        key: "{animation_key},{i}", // needed to force remounts, so animations don't get "stale" and refuse to replay
                        {nodes}
                    }
                }
            },
        }
    });

    let score_board_pos = Vec2::new(margin_x, pos_y(1));
    let score_board_width = get_pos(DepotRole::Tableau.id(0), 0).x - spacer_x - score_board_pos.x;
    let score_board = rsx! {
        ScoreBoardComponent { 
            position: score_board_pos,
            width: score_board_width,
            score_board: board.score_board,
            high_score,
        }
    };
    

    let recycle_symbol = if discard_playable {
        let pos = get_pos(DepotRole::Discard.id(0), 0);
        rsx!{
            div {
                position: "absolute",
                top: rem(pos.y),
                left: rem(pos.x),
                width: rem(card_width),
                height: rem(card_height),
                display: "flex",
                align_items: "center",
                justify_content: "center",
                pointer_events: "none",
                font_size: rem(5. / 12. * card_width),
                color: "#0f0",
                font_family: "'Noto Emoji'",
                font_weight: "bold",

                "♻"
            }
        }
    } else {
        rsx!{}
    };

    let game_status_y = || {
        get_pos(DepotRole::Tableau.id(0), DECK_SIZE / DepotRole::Tableau.number_of() - 1).y
            + card_height + spacer_y
    };

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            for depot in 0..NUM_DEPOTS {
                if let Some(hint) = get_hint(depot) {
                    CardFrame { 
                        position: get_pos(depot, 0),
                        width: card_width,
                        hint,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, !0))
                        },
                    }
                }

                for i in 0..board.depots[depot].len() {
                    CardComponent { 
                        position: get_pos(depot, i),
                        width: card_width,
                        card: if is_face_up(depot) {board.depots[depot][i]},
                        // number_hint: if !is_face_up(depot) {i + 1},
                        disabled: should_disabled_display(depot, i),
                        skin,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, i))
                        },
                    }
                }
            }

            {score_board},
            {recycle_symbol},
            {anims},

            if game_status.is_won {
                div {
                    position: "absolute",
                    top: rem(game_status_y()),
                    left: rem(17.5),
                    width: rem(59.),
                    background_color: "#505",
                    padding: rem(3.),
                    color: "#fff",
                    font_size: rem(7.),
                    border_radius: rem(2.),
                    text_align: "center",
                    "YOU WIN!",
                    if game_status.is_playable {
                        p {
                            font_size: rem(3.),
                            "You may keep playing for a high score."
                        }
                    }
                }
            } else if !game_status.is_playable {
                div {
                    position: "absolute",
                    top: rem(game_status_y()),
                    left: rem(17.5),
                    width: rem(59.),
                    background_color: "#000",
                    padding: rem(3.),
                    color: "#fff",
                    font_size: rem(7.),
                    border_radius: rem(2.),
                    text_align: "center",
                    "GAME OVER",
                    p {
                        font_size: rem(3.),
                        "You didn’t score enough to meet the goal."
                    }
                }
            }
        }
    }
}