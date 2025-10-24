use libjack::{Game, MoveAction};
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom as CustomStatus;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::{Arc, Mutex, atomic};

mod libjack;

type CustomResp<T> = Result<T, CustomStatus<String>>;
type GameState = State<BlackJack>;

fn custom_err<T>(status: Status, msg: &str) -> CustomResp<T> {
    Err(CustomStatus(status, msg.to_string()))
}

#[rocket::get("/")]
fn index() -> &'static str {
    "BlackJack!"
}

#[rocket::get("/join?<username>")]
fn join(username: &str, game_state: &GameState) -> String {
    let mut locked_bread = game_state.games.lock().unwrap();
    let mut buf = String::new();
    if let Some(locked_game) = locked_bread.get_mut(username) {
        if locked_game.current_state().has_ended() {
            locked_game.play_again();
            _ = writeln!(buf, "{username:?} is up for a new game...");
        } else {
            _ = writeln!(buf, "{username:?} shouldn't join twice!"); // TODO: err code resp
        }
    } else if !locked_bread.contains_key(username) {
        locked_bread.insert(username.to_string(), Game::default());
        _ = writeln!(buf, "{username:?} is entering the chat...");
    }
    _ = write!(buf, "everyone now: {:?}", locked_bread.keys());
    buf
}

#[rocket::post("/bet/<username>?<amount>")]
fn bet(username: &str, amount: u16, game_state: &GameState) -> CustomResp<String> {
    let mut games = game_state.games.lock().unwrap();
    let Some(locked_game) = games.get_mut(username) else {
        return custom_err(Status::NotFound, "can't find user");
    };
    if !locked_game.current_state().is_waiting_bet() {
        // might be up to something cheeky
        return custom_err(Status::Forbidden, "not waiting for a bet");
    } else if locked_game.player.make_bet(amount).is_none() {
        return custom_err(Status::PaymentRequired, "insufficient money, no bet");
    }
    locked_game.init();
    locked_game.player.pay_out(locked_game.current_state());
    Ok(locked_game.to_string())
}

// TODO: abbrs: [h(it), s(tand)]
#[rocket::post("/move/<username>?<action>")]
fn r#move(username: &str, action: MoveAction, game_state: &GameState) -> CustomResp<String> {
    let mut games = game_state.games.lock().unwrap();
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
    Ok(format!("{:?}", locked_game.player))
}

#[rocket::get("/game_state")]
fn game_state(visitor_count: &State<atomic::AtomicUsize>, state: &GameState) -> String {
    let count = visitor_count.fetch_add(1, atomic::Ordering::Relaxed) + 1;
    format!("{count}: anotheone!\n{state}")
}

#[derive(Debug, Default, Clone)]
struct BlackJack {
    // TODO: one dealer and deck for all players|clients
    games: Arc<Mutex<HashMap<String, Game>>>,
}
impl std::fmt::Display for BlackJack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Ok(users) = self.games.lock() else {
            return Err(std::fmt::Error);
        };
        for (name, game) in users.iter() {
            writeln!(f, "{name}: {game}")?;
        }
        Ok(())
    }
}

#[rocket::launch]
fn rocket() -> _ {
    // TODO: custom logging?
    let blackjack = BlackJack {
        games: Arc::new(Mutex::new(HashMap::new())),
    };

    rocket::build() // see Rocket.toml
        .manage(blackjack)
        .manage(atomic::AtomicUsize::new(0))
        .mount("/", rocket::routes![index, join, bet, r#move, game_state])
        .mount("/help", rocket::routes![index])
    // GET help
    // GET join
    // GET state
    // POST bet
    // POST move
}
