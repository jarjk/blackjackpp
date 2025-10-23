use libjack::{Game, MoveAction};
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom as CustomStatus;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

mod libjack;

type CustomResp<T> = Result<T, CustomStatus<String>>;
type GameState = State<Casino>;

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
    if locked_bread
        .get(username)
        .is_none_or(|g| g.current_state().has_ended())
    {
        locked_bread.insert(username.to_string(), Game::new());
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
    if locked_game.player.make_bet(amount).is_none() {
        return custom_err(Status::PaymentRequired, "insufficient money");
    } else if !locked_game.current_state().is_waiting_bet() {
        return custom_err(Status::Forbidden, "not waiting for a bet");
    }
    locked_game.made_a_bet();
    Ok(format!("{locked_game}"))
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
fn game_state(visitors: &State<HitCount>, state: &GameState) -> String {
    let count = visitors.count.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{count}: anotheone!\n{state}")
}

#[derive(Debug, Default, Clone)]
struct Casino {
    // TODO: one dealer and deck for all players|clients
    games: Arc<Mutex<HashMap<String, Game>>>,
}
impl std::fmt::Display for Casino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Ok(users) = self.games.lock() else {
            return Err(std::fmt::Error);
        };
        for (name, game) in users.iter() {
            write!(f, "{name}: {game}")?;
        }
        Ok(())
    }
}

struct HitCount {
    count: AtomicUsize,
}

#[rocket::launch]
fn rocket() -> _ {
    // log
    let b = AtomicBool::new(true);
    let x = Casino {
        games: Arc::new(Mutex::new(HashMap::new())),
    };

    rocket::build() // :5225 = jack
        .manage(x)
        .manage(b)
        .manage(HitCount {
            count: AtomicUsize::new(0),
        })
        .mount("/", rocket::routes![index, join, bet, r#move, game_state])
        .mount("/help", rocket::routes![index])
    // GET help
    // GET join
    // GET state
    // POST bet
    // POST move
}
