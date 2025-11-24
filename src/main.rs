//! a server for playing `BlackJack`

use libjack::{Game, MoveAction};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::response::status::Custom as CustomStatus;
use rocket::serde::json::Json;
use rocket::{Request, Response, State, get, http::Status, post, response::Redirect};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod libjack;

type CustomResp<T> = Result<T, CustomStatus<&'static str>>;
type GameState = State<Arc<Mutex<BJTable>>>;

// Source - https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs
// Posted by Ibraheem Ahmed, modified by community. See post 'Timeline' for change history
// Retrieved 2025-11-13, License - CC BY-SA 4.0
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/// http response with `status` code and `msg`
fn custom_err<T>(status: Status, msg: &'static str) -> CustomResp<T> {
    Err(CustomStatus(status, msg))
}

/// index, redirect to git repo
#[openapi]
#[get("/")]
fn index() -> Redirect {
    Redirect::to("https://github.com/jarjk/blackjackpp")
}

/// join the game as `username`  
/// if already joined and finished playing, rejoins  
/// NOTE: currently each game has it's own dealer  
#[openapi]
#[get("/join?<username>")]
fn join(username: &str, game_state: &GameState) -> CustomResp<Json<&'static str>> {
    let games = &mut game_state.lock().unwrap().games;
    let resp;
    if let Some(game) = games.get_mut(username) {
        if game.current_state().has_ended() {
            game.play_again();
            resp = "new game";
        } else {
            return custom_err(Status::Forbidden, "shouldn't join twice");
        }
    } else {
        games.insert(username.to_string(), Game::default());
        resp = "first game";
    }
    // _ = write!(buf, "all the players now: {:?}", games.keys());
    Ok(Json(resp))
}

/// make a bet of `amount` for `username`  
/// requires a game waiting for bet of `username`  
/// `bet` shall be non-zero, but shouldn't exceed user wealth  
/// deals and so might end the game with a blackjack  
/// returns game state
#[openapi]
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
    Ok(Json(game.clone())) // PERF: oops, TODO: shouldn't use `game_state_of` instead?
}

/// make a [`move`](MoveAction) for `username`  
/// requires a previously made [bet]  
/// might end the game  
/// returns game state
#[openapi]
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
    game.update_state(action); // deals, maybe now it's ended
    Ok(Json(game.clone())) // PERF: oops, TODO: shouldn't use `game_state_of` instead?
}

/// get the game state of a specific `username`
#[openapi]
#[get("/game_state/<username>")]
fn game_state_of(username: &str, game_state: &GameState) -> Option<Json<Game>> {
    let games = &game_state.lock().unwrap().games;
    if username.is_empty() || !games.contains_key(username) {
        None
    } else {
        Some(Json(games[username].clone())) // PERF: don't worry
    }
}

/// a blackjack table, with a separate dealer for each player
#[derive(Debug, Default, Clone, JsonSchema)]
struct BJTable {
    // TODO: one dealer and deck for all players|clients at the same table
    games: HashMap<String, Game>,
}
impl std::fmt::Display for BJTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, game) in &self.games {
            writeln!(f, "{name}: {game}")?;
        }
        Ok(())
    }
}

#[rocket::launch]
fn rocket() -> _ {
    let blackjack = BJTable::default();

    rocket::build() // see Rocket.toml
        .attach(CORS)
        .manage(Arc::new(Mutex::new(blackjack)))
        .mount(
            "/",
            openapi_get_routes![index, join, bet, make_move, game_state_of],
        )
}
