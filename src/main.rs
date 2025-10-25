use libjack::{Game, MoveAction};
use rocket::response::status::Custom as CustomStatus;
use rocket::serde::{Serialize, json::Json};
use rocket::{State, http::Status};
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::{Arc, Mutex, atomic};

mod libjack;

type CustomResp<T> = Result<T, CustomStatus<String>>;
type GameState = State<Arc<Mutex<BlackJack>>>;

fn custom_err<T>(status: Status, msg: &str) -> CustomResp<T> {
    Err(CustomStatus(status, msg.to_string()))
}

#[rocket::get("/")]
fn index() -> &'static str {
    "BlackJack!"
}

#[rocket::get("/join?<username>")]
fn join(username: &str, game_state: &GameState) -> CustomResp<String> {
    let locked_bread = &mut game_state.lock().unwrap().games;
    let mut buf = String::new();
    if let Some(locked_game) = locked_bread.get_mut(username) {
        if locked_game.current_state().has_ended() {
            locked_game.play_again();
            _ = writeln!(buf, "{username:?} is up for a new game...");
        } else {
            return custom_err(Status::Forbidden, "shouldn't join twice");
        }
    } else if !locked_bread.contains_key(username) {
        locked_bread.insert(username.to_string(), Game::default());
        _ = writeln!(buf, "{username:?} is entering the chat...");
    }
    _ = write!(buf, "everyone now: {:?}", locked_bread.keys());
    Ok(buf)
}

#[rocket::post("/bet/<username>?<amount>")]
fn bet(username: &str, amount: u16, game_state: &GameState) -> CustomResp<Json<Game>> {
    let games = &mut game_state.lock().unwrap().games;
    let Some(locked_game) = games.get_mut(username) else {
        return custom_err(Status::NotFound, "can't find user");
    };
    if !locked_game.current_state().is_waiting_bet() {
        return custom_err(Status::Forbidden, "not waiting for a bet"); // might be up to something cheeky
    } else if locked_game.player.make_bet(amount).is_none() {
        return custom_err(Status::PaymentRequired, "insufficient money, or no bet");
    }
    locked_game.init();
    locked_game.player.pay_out(locked_game.current_state());
    Ok(Json(locked_game.clone()))
}

#[rocket::post("/move/<username>?<action>")]
fn r#move(username: &str, action: MoveAction, game_state: &GameState) -> CustomResp<Json<Game>> {
    let games = &mut game_state.lock().unwrap().games;
    let Some(locked_game) = games.get_mut(username) else {
        return custom_err(Status::NotFound, "can't find user");
    };
    if locked_game.current_state().has_ended() {
        return custom_err(Status::Forbidden, "game ended");
    } else if locked_game.player.get_bet() == 0 {
        return custom_err(Status::PaymentRequired, "forgot to bet");
    }
    match action {
        MoveAction::Hit => locked_game.deal_player(),
        MoveAction::Stand => locked_game.deal_dealer(),
    }
    locked_game.update_state(action);
    Ok(Json(locked_game.clone()))
}

#[rocket::get("/game_state/<username>")]
fn game_state_of(username: &str, state: &GameState) -> Option<Json<BlackJack>> {
    let state = &state.lock().unwrap();
    if username.is_empty() || !state.games.contains_key(username) {
        None
    } else {
        Some(Json(BlackJack {
            games: HashMap::from([(username.to_string(), (*state).games[username].clone())]),
        }))
    }
}

#[rocket::get("/game_state")]
fn game_state(visitor_count: &State<atomic::AtomicUsize>, state: &GameState) -> Json<BlackJack> {
    let count = visitor_count.fetch_add(1, atomic::Ordering::Relaxed) + 1;
    let state = &state.lock().unwrap();
    eprintln!("{count}: anotheone!\n{state}");
    Json((*state).clone()) // PERF: shit!
}

type Games = HashMap<String, Game>;

#[derive(Debug, Default, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
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
            rocket::routes![index, join, bet, r#move, game_state, game_state_of],
        )
        .mount("/help", rocket::routes![index])
    // GET help
    // GET join
    // GET state
    // POST bet
    // POST move
}
