//! data types for `BlackJack`
//!
//! adapted from <https://github.com/krisfur/rustjack/blob/master/src/game.rs>\
//! icons from <https://en.wikipedia.org/wiki/Playing_cards_in_Unicode>

use crate::libjack::State as JackState;
use serde::{Serialize, ser::SerializeStruct};
use std::fmt;

/// Represents the four suits of a card deck.
// TODO: manual `Serialize` impl might be better
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        f.write_str(str)
    }
}

/// Represents the 13 ranks of a card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
            num => &num.as_u8().to_string(), // 2..=10
        };
        f.write_str(str)
    }
}

impl Serialize for Rank {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
        // let mut rank = serializer.serialize_struct("Rank", 2)?;
        // rank.serialize_field("as_str", &self.to_string())?;
        // rank.serialize_field("as_u8", &self.as_u8())?;
        // rank.end()
    }
}

impl Rank {
    /// think of it as an id, where 2-10 matches the rank, then comes J, Q, K, A
    pub fn as_u8(self) -> u8 {
        self as u8 + 2
    }
    /// Returns the primary value for a card rank.\
    /// Ace is initially counted as 11.
    pub fn value_hint(self) -> u8 {
        match self {
            Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
            num => num.as_u8(),
        }
    }
    pub const ALL: [Rank; 13] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];
}

/// A single playing card with a suit and rank.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}

/// For displaying the card in a user-friendly way.
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

/// Represents a deck of cards, made up from one or more standard decks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Deck {
    cards: Vec<Card>,
}
impl Default for Deck {
    /// Creates a new, standard 52-card deck.
    fn default() -> Self {
        let mut cards = Vec::with_capacity(Deck::N_CARDS);
        for &suit in &Suit::ALL {
            for &rank in &Rank::ALL {
                cards.push(Card::new(suit, rank));
            }
        }
        Deck { cards }
    }
}

impl Deck {
    /// number of initial Cards per Deck
    pub const N_CARDS: usize = 52;
    /// number of initial Decks per Deck
    pub const N_DECKS: usize = 4;

    /// Creates a new Deck from [`Deck::N_DECKS`]  
    /// WARN: you'll most certainly want to use [`Deck::new`] instead of this
    pub fn init() -> Deck {
        let mut cards = Vec::with_capacity(Deck::N_DECKS * Deck::N_CARDS);
        for _ in 0..Deck::N_DECKS {
            cards.append(&mut Deck::default().cards);
        }
        Deck { cards }
    }

    /// Creates a new Deck from [`Deck::N_DECKS`], shuffled, ready to play
    pub fn new() -> Deck {
        let mut deck = Self::init();
        deck.shuffle();
        deck
    }

    /// Shuffles the deck randomly.
    pub fn shuffle(&mut self) {
        fastrand::shuffle(&mut self.cards);
    }

    /// Deals one card from the top of the deck.\
    /// Refills the whole Deck if half of it is gone.
    #[must_use]
    pub fn deal_card(&mut self) -> Card {
        if self.cards.len() < Deck::N_DECKS / 2 * Deck::N_CARDS {
            *self = Deck::new();
        }
        self.cards.pop().unwrap() // SAFETY: refilled above if wouldn't be enough
    }
}

// For displaying the Hand in a user-friendly way. (with a padding ' ')
// impl fmt::Debug for Deck {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for card in &self.cards {
//             write!(f, "{card} ")?;
//         }
//         Ok(())
//     }
// }

/// Represents a player's or dealer's hand.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
}
impl Serialize for Hand {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Hand", 2)?;
        state.serialize_field("cards", &self.cards)?;
        state.serialize_field("value", &self.value())?;
        state.end()
    }
}

impl Hand {
    /// Adds a card to the hand.
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Calculates the total value of the hand.\
    /// Correctly handles the flexible value of Aces (1 or 11).
    pub fn value(&self) -> u8 {
        let mut value = 0;
        let mut ace_count = 0;

        for card in &self.cards {
            value += card.rank.value_hint();
            if card.rank == Rank::Ace {
                ace_count += 1;
            }
        }

        // Adjust for Aces if the total value is over 21
        while value > 21 && ace_count > 0 {
            value -= 10;
            ace_count -= 1;
        }

        value
    }

    /// returns what's not a secret
    pub fn get(self, public: bool) -> Self {
        if public {
            self
        } else {
            Self {
                cards: self.cards.into_iter().take(1).collect(),
            }
        }
    }
}

// For displaying the Hand in a user-friendly way.
impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.value())?;
        for card in &self.cards {
            write!(f, " {card}")?;
        }
        Ok(())
    }
}

/// Represents a player.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Player {
    hand: Hand,
    pub wealth: u16,
    bet: u16,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            hand: Default::default(),
            wealth: Player::DEF_WEALTH,
            bet: 0,
        }
    }
}
impl Player {
    /// default wealth
    pub const DEF_WEALTH: u16 = 1000;

    /// adds a card to the hand
    pub fn add_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    /// get what the player's bet
    pub fn get_bet(&self) -> u16 {
        self.bet
    }

    /// subtracts the `amount` of bet from `wealth`\
    /// `None` if: already made a bet, making a 0 bet, or not enough wealth.
    #[must_use]
    pub fn make_bet(&mut self, amount: u16) -> Option<()> {
        if self.bet != 0 || amount == 0 {
            return None;
        }
        self.wealth = self.wealth.checked_sub(amount)?;
        self.bet = amount;
        Some(())
    }
    /// returns the value of the player's cards in hand
    pub fn value(&self) -> u8 {
        self.hand.value()
    }

    /// adds or subtracts `bet` from `wealth` if state is `has_ended`\
    /// NOTE: `bet` has to be non-zero  
    /// resets `bet` to 0 after successful payout
    pub fn pay_out(&mut self, state: JackState) {
        if !state.has_ended() {
            return;
        }
        let multip = match state {
            JackState::PlayerJack => 3, // (+orig_bet + 2*bet) yep, we're generous
            JackState::PlayerWin | JackState::DealerBust => 2, // +orig_bet+bet
            JackState::Push => 1,       // +orig_bet
            JackState::DealerJack | JackState::PlayerBust | JackState::DealerWin => 0, // -orig_bet
            _ => unreachable!("it has already ended"),
        };
        self.wealth += self.bet * multip;
        self.bet = 0;
        // JackState::WaitingBet
    }
}

#[cfg(test)]
mod tests {
    use crate::libjack::structs::*;
    use std::collections::HashMap;

    #[test]
    fn deck_new() {
        let deck = Deck::default();
        assert_eq!(deck.cards.len(), Deck::N_CARDS);
        assert_eq!(
            deck.cards.first(),
            Some(Card {
                suit: *Suit::ALL.first().unwrap(),
                rank: *Rank::ALL.first().unwrap()
            })
            .as_ref()
        );
        assert_eq!(
            deck.cards.last(),
            Some(Card {
                suit: *Suit::ALL.last().unwrap(),
                rank: *Rank::ALL.last().unwrap()
            })
            .as_ref()
        );
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), Deck::N_CARDS * Deck::N_DECKS);

        let mut suit_counts = Suit::ALL.iter().map(|s| (*s, 0)).collect::<HashMap<_, _>>();
        let suit_counts_exp = Suit::ALL
            .iter()
            .map(|s| (*s, Rank::ALL.len() * Deck::N_DECKS))
            .collect::<HashMap<_, _>>();

        let mut rank_counts = Rank::ALL.iter().map(|r| (*r, 0)).collect::<HashMap<_, _>>();
        let rank_counts_exp = Rank::ALL
            .iter()
            .map(|r| (*r, Suit::ALL.len() * Deck::N_DECKS))
            .collect::<HashMap<_, _>>();

        deck.cards.iter().for_each(|Card { suit, rank }| {
            suit_counts.entry(*suit).and_modify(|n| {
                *n += 1;
            });
            rank_counts.entry(*rank).and_modify(|n| {
                *n += 1;
            });
        });
        assert_eq!(suit_counts, suit_counts_exp);
        assert_eq!(rank_counts, rank_counts_exp);
        assert_eq!(0, 0);
    }

    #[test]
    fn payout() {
        let mut p = Player::default();
        const BET: u16 = 100;
        p.make_bet(BET).unwrap(); // -1bet
        assert_eq!(p.bet, BET);
        assert_eq!(p.wealth, Player::DEF_WEALTH - BET);
        p.pay_out(JackState::PlayerJack); // + 3bet
        assert_eq!(p.wealth, Player::DEF_WEALTH + 2 * BET);
        p.make_bet(BET).unwrap(); // -1bet
        p.pay_out(JackState::DealerJack);
        assert_eq!(p.wealth, Player::DEF_WEALTH + BET);
        p.make_bet(BET).unwrap(); // -1bet
        p.pay_out(JackState::DealerWin);
        assert_eq!(p.wealth, Player::DEF_WEALTH);
        p.make_bet(BET).unwrap(); // -1bet
        p.pay_out(JackState::DealerBust); // +2bet
        assert_eq!(p.wealth, Player::DEF_WEALTH + BET);
        p.make_bet(BET).unwrap(); // -1bet
        p.pay_out(JackState::PlayerBust);
        p.make_bet(BET).unwrap(); // -1bet
        p.pay_out(JackState::DealerJack);
        p.make_bet(BET).unwrap(); // -1bet
        assert!(p.make_bet(BET).is_none()); // don't bet twice
        p.pay_out(JackState::Push); // +1bet
        assert_eq!(p.wealth, Player::DEF_WEALTH - BET);
        for _ in 0..9 {
            p.make_bet(BET).unwrap(); // -1bet
            p.pay_out(JackState::DealerWin);
        }
        assert_eq!(p.wealth, 0);
        assert!(p.make_bet(BET).is_none()); // no money
    }

    #[test]
    fn hand_value() {
        let hand_from = |ranks: &[Rank]| Hand {
            cards: ranks
                .iter()
                .map(|rank| Card::new(Suit::Hearts, *rank))
                .collect::<Vec<_>>(),
        };

        let hand = hand_from(&[Rank::Two, Rank::Three, Rank::Jack]);
        assert_eq!(2 + 3 + 10, hand.value());
        let mut hand = hand_from(&[Rank::Ace, Rank::Eight]);
        assert_eq!(11 + 8, hand.value());
        hand.add_card(Card::new(Suit::Diamonds, Rank::Queen));
        assert_eq!(1 + 8 + 10, hand.value());
        hand.add_card(Card::new(Suit::Clubs, Rank::Ace));
        hand.add_card(Card::new(Suit::Spades, Rank::Ace));
        assert_eq!(1 + 8 + 10 + 1 + 1, hand.value());
    }
    #[test]
    fn deal_card() {
        let mut deck = Deck::new();
        const ORIG_LEN: usize = Deck::N_CARDS * Deck::N_DECKS;
        for i in 1..ORIG_LEN / 2 + 2 {
            _ = deck.deal_card(); // len--
            assert_eq!(ORIG_LEN - i, deck.cards.len());
        }
        _ = deck.deal_card(); // half of the deck is gone, reinits
        assert_eq!(ORIG_LEN - 1, deck.cards.len());
    }
    #[test]
    fn make_bet_payout() {
        let mut p = Player::default();
        const HALF_WEALTH: u16 = Player::DEF_WEALTH / 2;
        p.make_bet(HALF_WEALTH).unwrap();
        let mut next_p = Player {
            hand: Hand::default(),
            wealth: HALF_WEALTH,
            bet: HALF_WEALTH,
        };
        assert_eq!(next_p, p);
        assert_eq!(None, p.make_bet(HALF_WEALTH)); // don't bet twice
        p.pay_out(JackState::Ongoing); // nothing happens, hasn't ended
        assert_eq!(next_p, p);
        p.pay_out(JackState::DealerBust); // doesn't care about the `hand`
        next_p.bet = 0;
        next_p.wealth = 3 * HALF_WEALTH;
        assert_eq!(next_p, p);
        p.make_bet(1500).unwrap(); // all-in
        assert_eq!(1500, p.bet);
        p.pay_out(JackState::DealerJack);
        next_p.wealth = 0;
        assert_eq!(next_p, p); // bankrupt
    }
}
