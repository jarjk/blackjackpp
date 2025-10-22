mod structs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Game {
    deck: structs::Deck,
    dealer: structs::Hand,
    pub player: structs::Player,
}
impl Game {
    // pub fn deal(&mut self) -> structs::Card {
    //     self.deck.deal_card()
    // }
    pub fn deal_dealer(&mut self) {
        self.dealer.add_card(self.deck.deal_card()); // TODO: 17 thingy
    }
    pub fn deal_player(&mut self) {
        self.player.add_card(self.deck.deal_card()); // TODO: 17 thingy
    }
    pub fn new() -> Game {
        let mut deck = structs::Deck::init();
        let mut dealer = structs::Hand::default();
        let mut player = structs::Player::default();
        dealer.add_card(deck.deal_card());
        player.add_card(deck.deal_card());
        dealer.add_card(deck.deal_card());
        player.add_card(deck.deal_card());
        Game {
            deck,
            dealer,
            player,
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, Dealer: {{ {:?} }}", self.player, self.dealer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, rocket::FromFormField)]
pub enum MoveAction {
    Hit,
    Stand,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BlackJack {}
