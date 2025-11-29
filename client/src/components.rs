// use crate::model::{ApiCard, ApiGame, ApiHand};
use dioxus::prelude::*;
use libjack::Game;
use libjack::structs::{Card, Hand, Rank};

pub const CARD_IMG_CLASS: &str =
    "h-32 w-auto rounded-2xl border border-slate-800 bg-slate-950/80 shadow-lg shadow-black/40";
pub const INPUT_CLASS: &str = "rounded-xl border border-slate-800 bg-slate-950 px-3 py-2 text-slate-100 shadow-inner shadow-black/40 focus:border-blue-400 focus:outline-none focus:ring-2 focus:ring-blue-500";
pub const BTN_PRIMARY: &str = "max-w-40 rounded-xl bg-blue-600 px-4 py-2 font-semibold text-white shadow shadow-blue-900/50 transition hover:bg-blue-500";
pub const BTN_ACCENT: &str = "rounded-xl bg-emerald-600 px-4 py-2 font-semibold text-white shadow shadow-emerald-900/40 transition hover:bg-emerald-500";
pub const BTN_SECONDARY: &str = "rounded-xl border border-slate-700 px-4 py-2 font-medium text-slate-100 hover:border-blue-400 hover:text-blue-300";

#[component]
pub fn MessagePanel(message: String) -> Element {
    if message.trim().is_empty() {
        return rsx!(div {});
    }
    rsx!(
        div { class: "rounded-2xl border border-amber-400/40 bg-amber-400/10 px-4 py-3 text-sm text-amber-100 shadow shadow-amber-900/40", {message} }
    )
}

#[component]
pub fn GameView(game: Game) -> Element {
    let status = format!("State: {:?}", game.state);
    let wealth = format!("Wealth: ${:?}", game.player.wealth);
    let bet = format!("Current bet: ${:?}", game.player.bet);
    rsx!(
        div { class: "flex flex-col gap-6",
            div { class: "grid gap-3 sm:grid-cols-2 lg:grid-cols-3",
                InfoBadge { label: "Status".to_string(), value: status }
                InfoBadge { label: "Wealth".to_string(), value: wealth }
                InfoBadge { label: "Bet".to_string(), value: bet }
            }
            HandView { title: "Dealer".to_string(), hand: game.dealer.clone() } // WARN clone should be *obj, couldnt deref
            HandView { title: "Player".to_string(), hand: game.player.hand.clone() } // WARN clone should be *obj, couldt deref
        }
    )
}

#[component]
fn InfoBadge(label: String, value: String) -> Element {
    rsx!(
        div { class: "flex min-w-[180px] flex-col gap-1 rounded-2xl border border-slate-800 bg-slate-900/70 px-4 py-3 shadow-inner",
            div { class: "text-xs uppercase tracking-[0.2em] text-slate-400", {label} }
            div { class: "text-lg font-semibold", {value} }
        }
    )
}

#[component]
fn HandView(title: String, hand: Hand) -> Element {
    rsx!(
        div { class: "rounded-2xl border border-slate-800 bg-gradient-to-b from-slate-900/70 to-slate-900/30 p-4 shadow-xl shadow-black/40",
            div { class: "mb-3 flex items-center justify-between",
                h3 { class: "text-xl font-semibold", {title.clone()} }
                span { class: "text-sm text-slate-400", "Value: {hand.value()}" }
            }
            div { class: "flex flex-wrap gap-3",
                if hand.cards.is_empty() {
                    img { class: CARD_IMG_CLASS, src: format!("{}/card_back.png", asset!("/assets/cards")), alt: "Card back" }
                } else {
                    for (idx, card) in hand.cards.iter().enumerate() {
                        CardImage { key: "{title}-{idx}", card: card.clone() }
                    }
                }
            }
        }
    )
}

#[component]
fn CardImage(card: Card) -> Element {
    let src = card_image_src(&card);
    let alt = format!("{} of {}", card.rank, card.suit);
    rsx!(img {
        class: CARD_IMG_CLASS,
        src,
        alt
    })
}

fn card_image_src(card: &Card) -> String {
    let rank = match card.rank {
        Rank::Ace => "ace".to_string(),
        Rank::King => "king".to_string(),
        Rank::Queen => "queen".to_string(),
        Rank::Jack => "jack".to_string(),
        other => (other as usize + 2).to_string(),
    };
    debug!("{rank}");
    let suit = card.suit.to_string().to_lowercase();

    let card = format!("{}/{rank}_of_{suit}.png", asset!("/assets/cards"));
    debug!("trying to print a card: {}", card);
    card
}
