use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom as CustomStatus;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

mod libjack;

#[rocket::get("/")]
fn index() -> &'static str {
    "BlackJack!"
}

#[rocket::get("/join?<username>")]
fn join(username: &str, game_state: &State<SvrGame>) -> String {
    let mut locked_bread = game_state.users.lock().unwrap();
    let mut buf = String::new();
    if !locked_bread.contains_key(username) {
        locked_bread.insert(username.to_string(), libjack::Game::new());
        _ = write!(buf, "{username:?} is entering the chat...\n");
    }
    _ = write!(buf, "everyone now: {:?}", locked_bread.keys());
    buf
}

#[rocket::post("/bet/<username>?<amount>")]
fn bet(
    username: &str,
    amount: u16,
    game_state: &State<SvrGame>,
) -> Result<String, CustomStatus<String>> {
    let mut games = game_state.users.lock().unwrap();
    let Some(locked_game) = games.get_mut(username) else {
        return Err(CustomStatus(
            Status::NotFound,
            String::from("can't find user"),
        ));
    };
    if locked_game.player.bet(amount).is_none() {
        return Err(CustomStatus(
            Status::PaymentRequired,
            String::from("not enough money"),
        ));
    }
    Ok(format!("{:?}", locked_game.player))
}

#[rocket::post("/move/<username>?<action>")]
fn r#move(
    username: &str,
    action: libjack::MoveAction,
    game_state: &State<SvrGame>,
) -> Result<String, CustomStatus<String>> {
    let mut games = game_state.users.lock().unwrap();
    let Some(locked_game) = games.get_mut(username) else {
        return Err(CustomStatus(
            Status::NotFound,
            String::from("can't find user"),
        ));
    };
    match action {
        libjack::MoveAction::Hit => {
            locked_game.deal_player();
        }
        libjack::MoveAction::Stand => {
            locked_game.deal_dealer();
        }
    }
    // if locked_game.player.r#move(amount).is_none() {
    //     return Err(CustomStatus(
    //         Status::PaymentRequired,
    //         String::from("not enough money"),
    //     ));
    // }
    Ok(format!("{:?}", locked_game.player))
}

#[rocket::get("/game_state")]
fn game_state(visitors: &State<HitCount>, state: &State<SvrGame>) -> String {
    let count = visitors.count.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{count}: anotheone!\n{state}")
}

#[derive(Debug, Default, Clone)]
struct SvrGame {
    users: Arc<Mutex<HashMap<String, libjack::Game>>>,
}
impl std::fmt::Display for SvrGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Ok(users) = self.users.lock() else {
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
    let x = SvrGame {
        users: Arc::new(Mutex::new(HashMap::new())),
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
