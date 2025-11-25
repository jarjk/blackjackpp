//! a server for playing `BlackJack`

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

mod libjack;
use libjack::{Game, MoveAction};

type SharedTable = Arc<Mutex<BJTable>>;

/// A blackjack table
#[derive(Debug, Default, Clone)]
struct BJTable {
    games: HashMap<String, Game>,
}

async fn index() -> Redirect {
    Redirect::temporary("https://github.com/jarjk/blackjackpp")
}

#[derive(Deserialize)]
struct JoinQ {
    username: String,
}

async fn join(
    Query(JoinQ { username }): Query<JoinQ>,
    State(state): State<SharedTable>,
) -> impl IntoResponse {
    let mut table = state.lock().unwrap();
    let games = &mut table.games;

    if let Some(game) = games.get_mut(&username) {
        if game.current_state().has_ended() {
            game.play_again();
            (StatusCode::OK, Json("new game"))
        } else {
            (StatusCode::FORBIDDEN, Json("shouldn't join twice"))
        }
    } else {
        games.insert(username, Game::default());
        (StatusCode::OK, Json("first game"))
    }
}

#[derive(Deserialize)]
struct BetQ {
    amount: u16,
}

async fn bet(
    Path(username): Path<String>,
    Query(BetQ { amount }): Query<BetQ>,
    State(state): State<SharedTable>,
) -> impl IntoResponse {
    let mut table = state.lock().unwrap();

    let Some(game) = table.games.get_mut(&username) else {
        return Err((StatusCode::NOT_FOUND, "can't find user"));
    };

    if !game.current_state().is_waiting_bet() {
        return Err((StatusCode::FORBIDDEN, "not waiting for a bet"));
    } else if game.player.make_bet(amount).is_none() {
        return Err((
            StatusCode::PAYMENT_REQUIRED,
            "insufficient money, or no bet",
        ));
    }

    game.init();
    game.player.pay_out(game.current_state());

    Ok(Json(game.clone()))
}

#[derive(Deserialize)]
struct MoveQ {
    action: MoveAction,
}

async fn make_move(
    Path(username): Path<String>,
    Query(MoveQ { action }): Query<MoveQ>,
    State(state): State<SharedTable>,
) -> impl IntoResponse {
    let mut table = state.lock().unwrap();

    let Some(game) = table.games.get_mut(&username) else {
        return Err((StatusCode::NOT_FOUND, "can't find user"));
    };

    if game.current_state().has_ended() {
        return Err((StatusCode::FORBIDDEN, "game ended"));
    } else if game.player.get_bet() == 0 {
        return Err((StatusCode::PAYMENT_REQUIRED, "forgot to bet"));
    }

    game.update_state(action);

    Ok(Json(game.clone()))
}

async fn game_state_of(
    Path(username): Path<String>,
    State(state): State<SharedTable>,
) -> impl IntoResponse {
    let table = state.lock().unwrap();

    match table.games.get(&username) {
        Some(game) => Json(game.clone()).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let shared_state = Arc::new(Mutex::new(BJTable::default()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(index))
        .route("/join", get(join))
        .route("/bet/{username}", post(bet))
        .route("/move/{username}", post(make_move))
        .route("/game_state/{username}", get(game_state_of))
        .layer(cors)
        .with_state(shared_state);

    info!("running on http://0.0.0.0:5225");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5225").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
