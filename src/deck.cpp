#include "headers/deck.hpp"

#include <algorithm>
#include <array>
#include <random>

#include "headers/random.hpp"

// Constructs a Deck
void Deck::initializeDeck() {
    deck.clear();
    std::array<char, 4> suits = {'S', 'H', 'D', 'C'};
    for (int i = 0; i < 4; i++) {
        for (int j = 0; j < 13; j++) {
            Card c(j + 1, suits.at(i));
            deck.push_back(c);
        }
    }

    std::shuffle(deck.begin(), deck.end(), rng::custom_random);
}

// Getter Function for size of deck
int Deck::getSize() { return deck.size(); }

// Deals by returning one card from the deck
Card Deck::deal() {
    std::uniform_int_distribution<> dist(0, deck.size());
    int val = dist(rng::custom_random);
    Card t = deck.at(val);
    deck.erase(deck.begin() + val);
    return t;
}
