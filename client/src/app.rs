use crate::components::{
    BTN_ACCENT, BTN_PRIMARY, BTN_SECONDARY, GameView, INPUT_CLASS, MessagePanel,
};
use crate::model::{Config, MoveAction};
use crate::net::{bet, fetch_game_state, join, make_move};
use dioxus::prelude::*;
use libjack::Game;

#[cfg(debug_assertions)]
const BASEURL: &str = "http://localhost:5225";
#[cfg(not(debug_assertions))]
const BASEURL: &str = "http://app.lovassy.hu:5225";

#[allow(non_snake_case)]
pub fn App() -> Element {
    let mut cfg = use_signal(|| Config {
        base_url: BASEURL.to_string(),
        username: String::new(),
    });
    let response = use_signal(|| String::new());
    let game_state = use_signal(|| Option::<Game>::None);
    let mut bet_amount = use_signal(|| "10".to_string());

    let join = {
        let cfg = cfg.to_owned();
        let response = response.to_owned();
        let game_state = game_state.to_owned();
        move |_| {
            let snapshot = cfg.read().clone();
            let mut resp = response.to_owned();
            if snapshot.username.trim().is_empty() {
                resp.set("Enter a username first.".to_string());
                return;
            }
            let mut game_sig = game_state.to_owned();
            spawn(async move {
                match join(&snapshot.base_url, &snapshot.username).await {
                    Ok(text) => {
                        resp.set(text);
                        match fetch_game_state(&snapshot).await {
                            Ok(game) => game_sig.set(Some(game)),
                            Err(err) => resp.set(format!("Joined, but failed to load game: {err}")),
                        }
                    }
                    Err(err) => resp.set(format!("Join failed: {err}")),
                }
            });
        }
    };

    let bet = {
        let cfg = cfg.to_owned();
        let response = response.to_owned();
        let bet_amount = bet_amount.to_owned();
        let game_state = game_state.to_owned();
        move |_| {
            let snapshot = cfg.read().clone();
            let mut resp = response.to_owned();
            if snapshot.username.trim().is_empty() {
                resp.set("Enter a username first.".to_string());
                return;
            }
            let amt = bet_amount.read().clone();
            if amt.trim().is_empty() {
                resp.set("Enter a bet amount.".to_string());
                return;
            }
            let mut game_sig = game_state.to_owned();
            spawn(async move {
                match bet(
                    &snapshot.base_url,
                    &snapshot.username,
                    amt.parse::<i32>().unwrap_or(0),
                )
                .await
                {
                    Ok(game) => {
                        resp.set(format!("Bet placed. State: {:?}", game.state));
                        game_sig.set(Some(game));
                    }
                    Err(err) => resp.set(format!("Bet failed: {err}")),
                }
            });
        }
    };

    let hit = {
        let cfg = cfg.to_owned();
        let response = response.to_owned();
        let game_state = game_state.to_owned();
        move |_| {
            trigger_move(MoveAction::Hit, &cfg, &response, &game_state);
        }
    };

    let stand = {
        let cfg = cfg.to_owned();
        let response = response.to_owned();
        let game_state = game_state.to_owned();
        move |_| {
            trigger_move(MoveAction::Stand, &cfg, &response, &game_state);
        }
    };

    let refresh_state = {
        let cfg = cfg.to_owned();
        let response = response.to_owned();
        let game_state = game_state.to_owned();
        move |_| {
            let snapshot = cfg.read().clone();
            let mut resp = response.to_owned();
            if snapshot.username.trim().is_empty() {
                resp.set("Enter a username first.".to_string());
                return;
            }
            let mut game_sig = game_state.to_owned();
            spawn(async move {
                match fetch_game_state(&snapshot).await {
                    Ok(game) => {
                        resp.set("Game state refreshed.".to_string());
                        game_sig.set(Some(game));
                    }
                    Err(err) => resp.set(format!("Refresh failed: {err}")),
                }
            });
        }
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        div { class: "min-h-screen bg-slate-950 text-slate-100 font-sans p-4 md:p-8",
            div { class: "mx-auto flex max-w-5xl flex-col gap-6",
                h1 { class: "text-3xl font-semibold tracking-tight text-white", "BlackJack" }

                div { class: "flex flex-wrap flex-col gap-3 rounded-2xl border border-slate-800 bg-slate-900/70 px-4 py-4 shadow-inner shadow-slate-900",
                    label { class: "text-sm uppercase tracking-wide text-slate-400", "Server" }
                    input {
                        class: INPUT_CLASS,
                        placeholder: "http://localhost:8000",
                        value: cfg.read().base_url.clone(),
                        oninput: move |ev| cfg.write().base_url = ev.value()
                    }
                    label { class: "text-sm uppercase tracking-wide text-slate-400", "Username" }
                    input {
                        class: INPUT_CLASS,
                        placeholder: "player",
                        value: cfg.read().username.clone(),
                        oninput: move |ev| cfg.write().username = ev.value()
                    }
                    button { class: "{BTN_PRIMARY} mt-4", onclick: join, "Join / Reset" }
                }

                div { class: "flex flex-wrap flex-col gap-3 rounded-2xl border border-slate-800 bg-slate-900/40 px-4 py-4 shadow-inner",
                    div {
                        label { class: "text-sm uppercase tracking-wide text-slate-400", "Bet" }
                        input {
                            class: "{INPUT_CLASS} w-24 ml-2",
                            r#type: "number",
                            value: bet_amount.read().clone(),
                            oninput: move |ev| bet_amount.set(ev.value())
                        }
                    }
                    div {
                        button { class: "{BTN_PRIMARY} mr-1", onclick: bet, "Place Bet" }
                        button { class: "{BTN_ACCENT} mx-1", onclick: hit, "Hit" }
                        button { class: "{BTN_ACCENT} mx-1", onclick: stand, "Stand" }
                            button { class: "{BTN_SECONDARY} ml-1", onclick: refresh_state, "Refresh" }
                    }
                }


                MessagePanel { message: response.read().clone() }

                match game_state.read().clone() {
                    Some(game) => rsx!(GameView { game }),
                    None => rsx!(p { class: "text-sm text-slate-400", "No game loaded yet. Join to start playing." })
                }
            }
        }
    }
}

fn trigger_move(
    action: MoveAction,
    cfg: &Signal<Config>,
    response: &Signal<String>,
    game_state: &Signal<Option<Game>>,
) {
    let snapshot = cfg.read().clone();
    let mut resp = response.to_owned();
    if snapshot.username.trim().is_empty() {
        resp.set("Enter a username first.".to_string());
        return;
    }
    let mut game_sig = game_state.to_owned();
    spawn(async move {
        match make_move(&snapshot.base_url, &snapshot.username, action).await {
            Ok(game) => {
                resp.set(format!("Move applied. State: {:?}", game.state));
                game_sig.set(Some(game));
            }
            Err(err) => resp.set(format!("Move failed: {err}")),
        }
    });
}
