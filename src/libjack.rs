use std::cmp::Ordering;

mod structs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Game {
    // TODO: statistics?
    deck: structs::Deck,
    dealer: structs::Hand,
    pub player: structs::Player,
    state: State,
}
impl Game {
    /// called once a bet is made
    pub fn init(&mut self) {
        self.dealer.add_card(self.deck.deal_card());
        self.player.add_card(self.deck.deal_card());
        self.dealer.add_card(self.deck.deal_card());
        self.player.add_card(self.deck.deal_card());
        // hole card check (USA)
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
    pub fn deal_dealer(&mut self) {
        // S17: stand on soft 17
        while self.dealer.value() < 17 {
            self.dealer.add_card(self.deck.deal_card());
        }
    }
    pub fn deal_player(&mut self) {
        self.player.add_card(self.deck.deal_card());
    }
    /// redeal cards, but remember wealth
    pub fn play_again(&mut self) {
        let player_wealth = self.player.wealth;
        *self = Self::default();
        self.player.wealth = player_wealth;
    }
    pub fn current_state(&self) -> State {
        self.state
    }
    /// NOTE: `BlackJack` is handled in [`Self::init`]
    fn new_state(&self, action: MoveAction) -> State {
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
        } else if p_value < 17 && d_value < 17 || action == MoveAction::Hit {
            return State::Ongoing;
        }
        // no further actions to be taken, let's just see who's got the bigger penis
        match p_value.cmp(&d_value) {
            Ordering::Less => State::DealerWin,
            Ordering::Equal => State::Push,
            Ordering::Greater => State::PlayerWin,
        }
    }
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
        // TODO: don't give dealer cards away
        write!(
            f,
            "{:?}, Dealer: {{ {:?} }}, State: {{ {:?} }}",
            self.player, self.dealer, self.state
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, rocket::FromFormField)]
pub enum MoveAction {
    // TODO: double-down, surrender, insurance, splitting?
    Hit,
    Stand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, rocket::FromFormField)]
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
    pub fn is_ongoing(self) -> bool {
        self == Self::Ongoing
    }
    pub fn is_waiting_bet(self) -> bool {
        self == Self::WaitingBet
    }
    pub fn has_ended(self) -> bool {
        !(self.is_ongoing() || self.is_waiting_bet())
    }
}
