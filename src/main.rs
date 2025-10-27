//! a server for playing `BlackJack`

use libjack::{Game, MoveAction};
use rocket::response::status::Custom as CustomStatus;
use rocket::serde::json::Json;
use rocket::{State, get, http::Status, post};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::{Arc, Mutex, atomic};

mod libjack;

type CustomResp<T> = Result<T, CustomStatus<String>>;
type GameState = State<Arc<Mutex<BlackJack>>>;
type Games = HashMap<String, Game>;

/// http response with `status` code and `msg`
fn custom_err<T>(status: Status, msg: &str) -> CustomResp<T> {
    Err(CustomStatus(status, msg.to_string()))
}

/// index, displays minimal server info
#[get("/")]
fn index() -> &'static str {
    "BlackJack!"
}

/// join the game as `username`\
/// if already joined and finished playing, rejoins\
/// NOTE: currently each game has it's own dealer\
#[get("/join?<username>")]
fn join(username: &str, game_state: &GameState) -> CustomResp<String> {
    let games = &mut game_state.lock().unwrap().games;
    let mut buf = String::new();
    if let Some(locked_game) = games.get_mut(username) {
        if locked_game.current_state().has_ended() {
            locked_game.play_again();
            _ = writeln!(buf, "{username:?} is up for a new game...");
        } else {
            return custom_err(Status::Forbidden, "shouldn't join twice");
        }
    } else if !games.contains_key(username) {
        games.insert(username.to_string(), Game::default());
        _ = writeln!(buf, "{username:?} is entering the chat...");
    }
    _ = write!(buf, "everyone now: {:?}", games.keys());
    Ok(buf)
}

/// make a bet of `amount` for `username`\
/// requires a game waiting for bet of `username`\
/// `bet` shall be non-zero, but shouldn't exceed user wealth\
/// deals and so might end the game with a blackjack\
/// returns game state
#[post("/bet/<username>?<amount>")]
fn bet(username: &str, amount: u16, game_state: &GameState) -> CustomResp<Json<Game>> {
    let games = &mut game_state.lock().unwrap().games;
    let Some(game) = games.get_mut(username) else {
        return custom_err(Status::NotFound, "can't find user");
    };
    if !game.current_state().is_waiting_bet() {
        return custom_err(Status::Forbidden, "not waiting for a bet"); // might be up to something cheeky
    } else if game.player.make_bet(amount).is_none() {
        return custom_err(Status::PaymentRequired, "insufficient money, or no bet");
    }
    game.init();
    game.player.pay_out(game.current_state()); // if it's ended with a BJ, handle payouts
    Ok(Json(game.clone())) // PERF: oops
}

/// make a [`move`](MoveAction) for `username`\
/// requires a previously made [bet]\
/// might end the game\
/// returns game state
#[post("/move/<username>?<action>")]
fn make_move(username: &str, action: MoveAction, game_state: &GameState) -> CustomResp<Json<Game>> {
    let games = &mut game_state.lock().unwrap().games;
    let Some(game) = games.get_mut(username) else {
        return custom_err(Status::NotFound, "can't find user");
    };
    if game.current_state().has_ended() {
        return custom_err(Status::Forbidden, "game ended");
    } else if game.player.get_bet() == 0 {
        return custom_err(Status::PaymentRequired, "forgot to bet");
    }
    match action {
        MoveAction::Hit => game.deal_player(),
        MoveAction::Stand => game.deal_dealer(),
    }
    game.update_state(action); // maybe it's ended
    Ok(Json(game.clone())) // PERF: oops
}

/// get the game state of a specific `username`
#[get("/game_state/<username>")]
fn game_state_of(username: &str, state: &GameState) -> Option<Json<BlackJack>> {
    let state = &state.lock().unwrap();
    if username.is_empty() || !state.games.contains_key(username) {
        None
    } else {
        Some(Json(BlackJack {
            games: HashMap::from([(username.to_string(), state.games[username].clone())]),
        }))
    }
}

/// get the state of all the games
#[get("/game_state")]
fn game_state(visitor_count: &State<atomic::AtomicUsize>, state: &GameState) -> Json<BlackJack> {
    let count = visitor_count.fetch_add(1, atomic::Ordering::Relaxed) + 1;
    let state = &state.lock().unwrap();
    eprintln!("{count}: anotheone!\n{state}");
    Json((*state).clone()) // PERF: shit!
}

/// a blackjack table, with one dealer for each player
#[derive(Debug, Default, Clone, Serialize)]
struct BlackJack {
    // TODO: one dealer and deck for all players|clients
    games: Games,
}
impl std::fmt::Display for BlackJack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, game) in &self.games {
            writeln!(f, "{name}: {game}")?;
        }
        Ok(())
    }
}

#[rocket::launch]
fn rocket() -> _ {
    // TODO: custom logging?
    let blackjack = BlackJack {
        games: HashMap::new(),
    };

    rocket::build() // see Rocket.toml
        .manage(Arc::new(Mutex::new(blackjack)))
        .manage(atomic::AtomicUsize::new(0))
        .mount(
            "/",
            rocket::routes![index, join, bet, make_move, game_state, game_state_of],
        )
        .mount("/help", rocket::routes![index])
}
