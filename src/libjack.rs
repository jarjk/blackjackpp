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
    pub fn deal_dealer(&mut self) {
        // S17: stand on soft 17
        while self.dealer.value() < 17 {
            self.dealer.add_card(self.deck.deal_card());
        }
    }
    pub fn deal_player(&mut self) {
        self.player.add_card(self.deck.deal_card());
    }
    pub fn new() -> Game {
        let mut deck = structs::Deck::init();
        let mut dealer = structs::Hand::default();
        let mut player = structs::Player::default();
        dealer.add_card(deck.deal_card());
        player.add_card(deck.deal_card());
        dealer.add_card(deck.deal_card());
        player.add_card(deck.deal_card());
        // hole card check (USA)
        let state = if player.value() == 21 && dealer.value() == 21 {
            State::Push
        } else if player.value() == 21 {
            State::PlayerJack
        } else if dealer.value() == 21 {
            State::DealerJack
        } else {
            State::WaitingBet
        };
        // TODO: handle wealth, statistics
        Game {
            deck,
            dealer,
            player,
            state,
        }
    }
    pub fn current_state(&self) -> State {
        self.state
    }
    /// NOTE: BlackJack is handled in [`Self::new`]
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
        // no further actions to be taken, let's just see who's got a bigger penis
        match p_value.cmp(&d_value) {
            Ordering::Less => State::DealerWin,
            Ordering::Equal => State::Push,
            Ordering::Greater => State::PlayerWin,
        }
    }
    pub fn made_a_bet(&mut self) {
        self.state = State::Ongoing;
    }
    pub fn update_state(&mut self, action: MoveAction) {
        self.state = self.new_state(action);
        // TODO: handle wealth, statistics
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: don't give dealer cards away
        write!(
            f,
            "{:?}, Dealer: {{ {:?} }}, Outcome: {{ {:?} }}",
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
