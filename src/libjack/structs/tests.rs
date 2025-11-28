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
    let exp = Player::DEF_WEALTH - BET;
    assert_eq!(p.wealth, exp);
    p.pay_out(JackState::PlayerJack); // + 2.5bet
    let exp = exp + BET * 5 / 2;
    assert_eq!(p.wealth, exp);
    p.make_bet(BET).unwrap(); // -1bet
    p.pay_out(JackState::DealerJack);
    let exp = exp - BET;
    assert_eq!(p.wealth, exp);
    p.make_bet(BET).unwrap(); // -1bet
    p.pay_out(JackState::DealerWin);
    let exp = exp - BET;
    assert_eq!(p.wealth, exp);
    p.make_bet(BET).unwrap(); // -1bet
    p.pay_out(JackState::DealerBust); // +2bet
    let exp = exp + BET;
    assert_eq!(p.wealth, exp);
    p.make_bet(BET).unwrap(); // -1bet
    p.pay_out(JackState::PlayerBust);
    p.make_bet(BET).unwrap(); // -1bet
    p.pay_out(JackState::DealerJack);
    p.make_bet(BET).unwrap(); // -1bet
    assert!(p.make_bet(BET).is_none()); // don't bet twice
    p.pay_out(JackState::Push); // +1bet
    let exp = exp - 2 * BET;
    assert_eq!(p.wealth, exp);
    for _ in 0..8 {
        p.make_bet(BET).unwrap(); // -1bet
        p.pay_out(JackState::DealerWin);
    }
    assert_eq!(p.wealth, 50);
    assert!(p.make_bet(BET).is_none()); // no money
}

#[test]
fn hand_value() {
    let hand_from = |ranks: &[Rank]| Hand {
        cards: ranks
            .iter()
            .map(|rank| Card::new(*rank, Suit::Hearts))
            .collect::<Vec<_>>(),
    };

    let hand = hand_from(&[Rank::Two, Rank::Three, Rank::Jack]);
    assert_eq!(2 + 3 + 10, hand.value());
    let mut hand = hand_from(&[Rank::Ace, Rank::Eight]);
    assert_eq!(11 + 8, hand.value());
    hand.add_card(Card::new(Rank::Queen, Suit::Diamonds));
    assert_eq!(1 + 8 + 10, hand.value());
    hand.add_card(Card::new(Rank::Ace, Suit::Clubs));
    hand.add_card(Card::new(Rank::Ace, Suit::Spades));
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
