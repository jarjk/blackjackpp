//! adapted from <https://github.com/krisfur/rustjack/blob/master/src/game.rs>
//! icons from <https://en.wikipedia.org/wiki/Playing_cards_in_Unicode>

use std::fmt;
// TODO: serde

/// Represents the four suits of a card deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
}

/// Represents the 13 ranks of a card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Rank {
    /// Returns the primary blackjack value for a card rank.
    /// Ace is initially counted as 11.
    pub fn value_hint(self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
        let rank = match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        let suit = match self.suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        write!(f, "{rank}{suit}")
    }
}

/// Represents a deck of cards, made up from one or more standard decks.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// number of initial Cards per Deck
    pub const N_CARDS: usize = 52;
    /// number of initial Decks per Deck
    pub const N_DECKS: usize = 4;

    /// Creates a new Deck from [`N_DECKS`], shuffled, ready to play
    pub fn init() -> Deck {
        let mut cards = Vec::with_capacity(Deck::N_DECKS * Deck::N_CARDS);
        for _ in 0..Deck::N_DECKS {
            cards.append(&mut Deck::new().cards);
        }
        let mut deck = Deck { cards };
        deck.shuffle();
        deck
    }

    /// Creates a new, standard 52-card deck.
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(Deck::N_CARDS);
        for &suit in &Suit::ALL {
            for &rank in &Rank::ALL {
                cards.push(Card::new(suit, rank));
            }
        }
        Deck { cards }
    }

    /// Shuffles the deck randomly.
    pub fn shuffle(&mut self) {
        fastrand::shuffle(&mut self.cards);
    }

    /// Deals one card from the top of the deck.
    #[must_use]
    pub fn deal_card(&mut self) -> Card {
        if self.cards.len() < Deck::N_DECKS / 2 * Deck::N_CARDS {
            *self = Deck::init();
        }
        self.cards.pop().unwrap() // SAFETY: refilled above if wouldn't be enough
    }
}
/// For displaying the Hand in a user-friendly way. (with a padding ' ')
// impl fmt::Debug for Deck {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for card in &self.cards {
//             write!(f, "{card} ")?;
//         }
//         Ok(())
//     }
// }

// Represents a player.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Player {
    hand: Hand,
    wealth: u16,
    bet: u16,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            hand: Default::default(),
            wealth: 1000,
            bet: 0,
        }
    }
}
impl Player {
    /// Adds a card to the hand.
    pub fn add_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    /// None if not enough wealth.
    pub fn bet(&mut self, amount: u16) -> Option<()> {
        self.wealth = self.wealth.checked_sub(amount)?;
        self.bet = amount;
        Some(())
    }
    /// Returns the value of the player's cards in hand
    pub fn value(&self) -> u8 {
        self.hand.value()
    }
}

// Represents a player's or dealer's hand.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Reset the hand to it's default state.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Adds a card to the hand.
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Calculates the total value of the hand.
    /// It correctly handles the flexible value of Aces (1 or 11).
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
}

/// For displaying the Hand in a user-friendly way.
impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.value())?;
        for card in &self.cards {
            write!(f, " {card}")?;
        }
        Ok(())
    }
}
