//! a server for playing `BlackJack`

use dioxus::fullstack::{
    FullstackContext, Redirect,
    axum_core::extract,
    body::Body,
    extract::{FromRef, Request},
    response::IntoResponse,
};
use dioxus::logger::tracing::Level;
#[cfg(feature = "server")]
use dioxus::prelude::*;
use dioxus::server::{
    Bytes,
    axum::{self, middleware},
};
use http_body_util::BodyExt;
use libjack::{Game, MoveAction};
use std::collections::HashMap;
#[cfg(feature = "server")]
use {
    std::sync::LazyLock,
    std::sync::{Arc, Mutex},
};



// type CustomResp<T> = Result<T, CustomStatus<&'static str>>;
// type GameState = State<BJTable>;

#[cfg(feature = "server")]
static GAME_STATE: LazyLock<Arc<Mutex<BJTable>>> =
    LazyLock::new(|| Arc::new(Mutex::new(BJTable::default())));

// /// http response with `status` code and `msg`
// fn custom_err<T>(status: Status, msg: &'static str) -> CustomResp<T> {
//     Err(CustomStatus(status, msg))
// }

/// index, redirect to git repo
#[get("/")]
async fn index() -> Result<Redirect> {
    Ok(Redirect::to("https://github.com/jarjk/blackjackpp"))
}

// /// handles CORS-related requests
// #[options("/<_..>")]
// fn cors_options() -> Status {
//     Status::Ok
// }

/// join the game as `username`\
/// if already joined and finished playing, rejoins\
/// NOTE: currently each game has it's own dealer\
#[get("/join?username")]
async fn join(username: String) -> Result<String> {
    let game_state = Arc::clone(&GAME_STATE);
    let games = &mut game_state.lock().unwrap().games;
    let resp;
    if let Some(game) = games.get_mut(&username) {
        game.current_state()
            .has_ended()
            .or_forbidden("shouldn't join twice")?;
        game.play_again();
        resp = "new game";
    } else {
        games.insert(username.to_string(), Game::default());
        resp = "first game";
    }
    // _ = write!(buf, "all the players now: {:?}", games.keys());
    Ok(resp.to_string())
}

/// make a bet of `amount` for `username`\
/// requires a game waiting for bet of `username`\
/// `bet` shall be non-zero, but shouldn't exceed user wealth\
/// deals and so might end the game with a blackjack\
/// returns game state
#[post("/bet/{username}?amount")]
async fn bet(username: String, amount: u16) -> Result<Game> {
    let game_state = Arc::clone(&GAME_STATE);
    let games = &mut game_state.lock().unwrap().games;
    let game = games.get_mut(&username).or_not_found("can't find user")?;
    game.current_state()
        .is_waiting_bet()
        .or_forbidden("not waiting for bet")?;
    game.player.make_bet(amount).or_http_error(
        StatusCode::PAYMENT_REQUIRED,
        "insufficient money, or no bet",
    )?;
    game.init();
    game.player.pay_out(game.current_state()); // if it's ended with a BJ, handle payouts
    Ok(game.clone()) // PERF: oops, TODO: shouldn't use `game_state_of` instead?
}

/// make a [`move`](MoveAction) for `username`\
/// requires a previously made [bet]\
/// might end the game\
/// returns game state
#[post("/move/{username}?action")]
async fn make_move(username: String, action: MoveAction) -> Result<Game> {
    let game_state = Arc::clone(&GAME_STATE);
    let games = &mut game_state.lock().unwrap().games;
    let game = games.get_mut(&username).or_not_found("can't find user")?;
    (!game.current_state().has_ended()).or_forbidden("game ended")?;
    (game.player.get_bet() != 0).or_http_error(StatusCode::PAYMENT_REQUIRED, "forgot to bet")?;
    game.update_state(action); // deals, maybe now it's ended
    Ok(game.clone()) // PERF: oops, TODO: shouldn't use `game_state_of` instead?
}

/// get the game state of a specific `username`
#[get("/game_state/{username}")]
async fn game_state_of(username: String) -> Result<Game> {
    let game_state = Arc::clone(&GAME_STATE);
    let games = &mut game_state.lock().unwrap().games;
    if username.is_empty() || !games.contains_key(&username) {
        return HttpError::not_found("can't find user")?;
    }
    Ok(games[&username].clone()) // PERF: don't worry
}

/// a blackjack table, with a separate dealer for each player
#[derive(Debug, Default, Clone)]
struct BJTable {
    // TODO: one dealer and deck for all players|clients at the same table
    games: HashMap<String, Game>,
}

impl FromRef<FullstackContext> for BJTable {
    fn from_ref(state: &FullstackContext) -> Self {
        state.extension::<BJTable>().unwrap_or_default()
    }
}

impl std::fmt::Display for BJTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, game) in &self.games {
            writeln!(f, "{name}: {game}")?;
        }
        Ok(())
    }
}

#[component]
fn App() -> Element {
    rsx! {
        "helo blackjack"
    }
}

fn main() {
    dioxus::logger::init(Level::TRACE).expect("logger failed to init"); // configure level with 'RUST_LOG' env var

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::{fullstack::Method, server::axum::middleware};
        use tower_http::cors::CorsLayer;

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(tower_http::cors::Any)
            .allow_origin(tower_http::cors::Any);
        let router = dioxus::server::router(App)
            .layer(cors)
            .layer(middleware::from_fn(print_request_response));

        Ok(router)
    });
}

async fn print_request_response(
    req: extract::Request,
    next: middleware::Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // debug!("{} {}", req.method(), req.uri()); // NOTE dioxus does this by deafult
    trace!(
        "version: {:?}, headers: {:?}, extensions: {:?}",
        req.version(),
        req.headers(),
        req.extensions(),
    );
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    // NOTE dioxus does this by default
    // let (parts, body) = res.into_parts();
    // let bytes = buffer_and_print("response", body).await?;
    // let res = Response::from_parts(parts, Body::from(bytes));
    // debug!("{}", res.status());

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        trace!("{direction} body = {body:?}");
    }

    Ok(bytes)
}
