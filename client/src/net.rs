use crate::model::{Config, MoveAction};
use anyhow::{self};
use libjack::Game;
use libjack::apis::Error as ApiError;
use libjack::apis::{configuration::Configuration, default_api};

/// Build a libjack client configuration for the given base URL.
fn cfg_for(base_url: &str) -> Configuration {
    let mut cfg = Configuration::default();
    cfg.base_path = base_url.to_string();
    cfg
}

/// Fetch game state for the configured user.
pub async fn fetch_game_state(cfg: &Config) -> anyhow::Result<Game> {
    let game = default_api::game_state_of(&cfg_for(&cfg.base_url), &cfg.username).await?;
    Ok(game)
}

pub async fn join(base_url: &str, username: &str) -> anyhow::Result<String> {
    default_api::join(&cfg_for(base_url), username)
        .await
        .map_err(map_api_err)
}

pub async fn bet(base_url: &str, username: &str, amount: i32) -> anyhow::Result<Game> {
    let game = default_api::bet(&cfg_for(base_url), username, amount)
        .await
        .map_err(map_api_err)?;
    Ok(game)
}

pub async fn make_move(base_url: &str, username: &str, action: MoveAction) -> anyhow::Result<Game> {
    let action_str = match action {
        MoveAction::Hit => "Hit",
        MoveAction::Stand => "Stand",
    };
    let game = default_api::make_move(&cfg_for(base_url), username, action_str)
        .await
        .map_err(map_api_err)?;
    Ok(game)
}

fn map_api_err<E: std::fmt::Debug>(err: ApiError<E>) -> anyhow::Error {
    match err {
        ApiError::ResponseError(resp) => {
            anyhow::anyhow!("{}: {}", resp.status, resp.content)
        }
        other => anyhow::anyhow!(other.to_string()),
    }
}
