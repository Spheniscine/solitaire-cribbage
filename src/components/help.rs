use dioxus::prelude::*;

use crate::{components::{SkinTrait, VIDEO_GAMEPLAY, rem}, game::{Card, GameState, RANK_JACK, RANK_KING, RANK_QUEEN, ScreenState, Skin, Suit}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

fn rank_text(skin: &Skin, rank: u8) -> Element {
    rsx! {
        span {
            font_size: "1.1em",
            {skin.render_rank(&Card { rank, suit: Suit::Spades })}
        }
    }
}

fn join_ranks(skin: &Skin, ranks: impl IntoIterator<Item = u8>, separator: Element) -> Element {
    let mut ite = ranks.into_iter().map(|rank| {
        rank_text(skin, rank)
    });

    let Some(first) = ite.next() else {return rsx!{}};

    rsx! {
        {first},
        for remaining in ite {
            {separator.clone()},
            {remaining}
        }
    }
}

#[component]
pub fn Help(mut game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let skin = st.skin;

    let rank_text = |rank: u8| rank_text(&skin, rank);


    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 4rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The deck is a standard 52-card deck. There are 4 suits, each with ranks: Ace (low), ",{rank_text(2)},
                    "~",{rank_text(10)},", Jack, Queen, King. Suits are ignored in this game."
                }

                p {
                    "Cards are played from the ",Emph{"tableau"}," to the ",Emph{"stack"}," one at a time, and may ",Emph{"score points"},
                    " based on the criteria listed below."
                }

                p {
                    "The ",Emph{"stack total"}," is the sum of the values of the cards in the stack (",{rank_text(1)},
                    " has value 1, ",{join_ranks(&skin, RANK_JACK..=RANK_KING, rsx!{"/"})}," have value 10). The total can ",
                    Emph{"never exceed 31"},"."
                }

                p {
                    "If you can play a card, you must. If you can’t, click on the ",Emph{"discard pile"}," to start a new stack."
                }

                p {
                    Emph {"Scoring criteria:"}
                    ul {
                        li { Emph{"Jack First"},": The first card played to the stack is a Jack. Scores 2 points."}

                        li { Emph{"Stack Total = 15/31"},": The stack total is exactly 15 or 31. Scores 2 points."}

                        li { 
                            Emph{"Pair/Triple/Quad"},": The frontmost 2/3/4 cards of the stack are the same rank. Scores 2/6/12 points 
                            respectively."
                        }

                        li {
                            Emph{"Run"},
                            ": The frontmost cards of the stack form a set of ",
                            Emph {"3 or more"},
                            " consecutive ranks, in any order, such as ",
                            {join_ranks(&skin, [RANK_KING, RANK_JACK, RANK_QUEEN], rsx!{"–"})},
                            " or ",
                            {join_ranks(&skin, [2, 4, 3, 1], rsx!{"–"})},
                            ". Scores 1 point for each card in the run."
                        }
                    }
                }

                p {
                    "To ",Emph{"win the game"},", you must score ",Emph{"61 points"}," or higher."
                }
            }

            div {
                position: "absolute",
                bottom: rem(2.),
                width: "92rem",
                display: "flex",
                justify_content: "center",

                a {
                    href: VIDEO_GAMEPLAY,
                    target: "_blank",
                    text_decoration: "none",
                    margin_right: rem(4.),
                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        "Example video"
                    }
                }

                div {
                    width: rem(30.),
                    position: "relative",
                    class: "game-button",
                    onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                    "Back to game"
                }
            }
        }
    }
}