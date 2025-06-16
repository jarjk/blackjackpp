#include "headers/deck.hpp"

#include <algorithm>

#include "headers/random.hpp"

// Constructs a Deck
void Deck::initializeDeck() {
    this->deck.clear();
    for (auto i = 0; i < N_DECKS; i++) {
        this->dealAnotherDeck();
    }

    std::shuffle(this->deck.begin(), this->deck.end(), rng::custom_random);
}

// Getter Function for size of deck
int Deck::getSize() { return static_cast<int>(this->deck.size()); }

// Deals by returning one card from the deck
Card Deck::deal() {
    // more than half of all the cards are dealt
    if (this->deck.size() < N_DECKS / 2 * N_CARDS) {
        std::cerr << "not enough cards in the deck, redealing...";
        this->initializeDeck();
    }
    Card t = this->deck.back();
    this->deck.pop_back();
    return t;
}
