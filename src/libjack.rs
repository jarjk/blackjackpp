//! `BlackJack` logic
//!
//! based on, inspiration from, further docs:
//! - <https://en.wikipedia.org/wiki/Blackjack>
//! - <https://vegasfreedom.com/blackjack/payouts/>
//! - <https://github.com/kevin-lesenechal/freebj>
//! - <https://chipy.com/academy/blackjack/blackjack-payouts>
//! - <https://serde.rs/impl-serialize.html>
//! - <https://bewersdorff-online.de/black-jack/>

use serde::{Deserialize, Serialize, ser::SerializeStruct};
use std::cmp::Ordering;

mod structs;

/// a game between one player and one dealer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    // TODO: statistics?
    deck: structs::Deck,
    dealer: structs::Hand,
    pub player: structs::Player,
    state: State,
}

impl Serialize for Game {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Game", 4)?;
        state.skip_field("deck")?;
        state.serialize_field("dealer", &self.dealer.clone().get(self.state.has_ended()))?;
        state.serialize_field("player", &self.player)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("winner", &self.state.winner_ch())?;
        state.end()
    }
}
impl Game {
    /// deals cards\
    /// called once a bet is made\
    /// hole card check (USA)\
    /// requires already set `deck`, call [`Game::default()`] before first use
    pub fn init(&mut self) {
        self.dealer.add_card(self.deck.deal_card());
        self.player.add_card(self.deck.deal_card());
        self.dealer.add_card(self.deck.deal_card());
        self.player.add_card(self.deck.deal_card());
        self.state = if self.player.value() == 21 && self.dealer.value() == 21 {
            State::Push
        } else if self.player.value() == 21 {
            State::PlayerJack
        } else if self.dealer.value() == 21 {
            State::DealerJack
        } else {
            State::Ongoing
        };
    }
    /// deals the dealer cards, till 17 is reached\
    /// S17: stands on soft 17
    pub fn deal_dealer(&mut self) {
        while self.dealer.value() < 17 {
            self.dealer.add_card(self.deck.deal_card());
        }
    }
    /// deals the player one card
    pub fn deal_player(&mut self) {
        self.player.add_card(self.deck.deal_card());
    }
    /// reset deck, hands, but remember player wealth\
    /// NOTE: doesn't check if previous game has ended
    pub fn play_again(&mut self) {
        let player_wealth = self.player.wealth;
        *self = Self::default();
        self.player.wealth = player_wealth;
    }
    /// returns current state
    pub fn current_state(&self) -> State {
        self.state
    }
    /// calculates state according to current dealer and player values and previous action\
    /// NOTE: `BlackJack` is handled in [`Self::init`]
    fn new_state(&self, prev_action: MoveAction) -> State {
        let p_value = self.player.value();
        let d_value = self.dealer.value();
        if p_value > 21 {
            return State::PlayerBust;
        } else if d_value > 21 {
            return State::DealerBust;
        } else if p_value == 21 {
            return State::PlayerWin;
        } else if d_value == 21 {
            return State::DealerWin;
        } else if p_value < 17 && d_value < 17 || prev_action == MoveAction::Hit {
            return State::Ongoing;
        }
        // no further actions to be taken, let's just see who's got the bigger penis
        match p_value.cmp(&d_value) {
            Ordering::Less => State::DealerWin,
            Ordering::Equal => State::Push,
            Ordering::Greater => State::PlayerWin,
        }
    }
    /// updates state according to player, dealer values and `action`\
    /// also handles paying out bet
    pub fn update_state(&mut self, action: MoveAction) {
        self.state = self.new_state(action);
        self.player.pay_out(self.state);
    }
}
impl Default for Game {
    fn default() -> Self {
        Game {
            deck: structs::Deck::init(),
            dealer: structs::Hand::default(),
            player: structs::Player::default(),
            state: State::WaitingBet,
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}, Dealer: {{ {:?} }}, State: {{ {:?} }}",
            self.player,
            self.dealer.clone().get(self.state.has_ended()),
            self.state
        )
    }
}

/// what a user can do during their turn
#[derive(Debug, Clone, Copy, PartialEq, Eq, rocket::FromFormField, Deserialize, Serialize)]
pub enum MoveAction {
    // TODO: double-down, surrender, insurance, splitting?
    Hit,
    Stand,
}

/// state of one game\
/// waiting for bet, ongoing, winner
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum State {
    WaitingBet,
    Ongoing,
    PlayerJack,
    DealerJack,
    PlayerBust,
    DealerBust,
    PlayerWin,
    DealerWin,
    Push,
}
impl State {
    /// currently playing
    pub fn is_ongoing(self) -> bool {
        self == Self::Ongoing
    }
    /// player hasn't made their bet yet
    pub fn is_waiting_bet(self) -> bool {
        self == Self::WaitingBet
    }
    /// we've got the outcome
    pub fn has_ended(self) -> bool {
        !(self.is_ongoing() || self.is_waiting_bet())
    }
    /// 'p'layer, 'd'ealer, 'e'qual, 'f'alse
    // TODO: #[deprecated = "make clients use state itself?"]
    pub fn winner_ch(self) -> char {
        match self {
            State::WaitingBet | State::Ongoing => 'f',
            State::PlayerJack | State::DealerBust | State::PlayerWin => 'p',
            State::DealerJack | State::PlayerBust | State::DealerWin => 'd',
            State::Push => 'e',
        }
    }
}
