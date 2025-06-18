#pragma once

#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <vector>

#include "card.hpp"
#include "random.hpp"

// Deck (Vector) of Cards
using Cards = std::vector<Card>;

class Deck {
   private:
    const static unsigned short N_DECKS = 4;
    static const unsigned short N_CARDS = 52;
    Cards deck;

   public:
    void dealAnotherDeck() {
        std::array<char, 4> suits = {'S', 'H', 'D', 'C'};
        for (int i = 0; i < 4; i++) {
            for (int j = 0; j < 13; j++) {
                Card c(j + 1, suits.at(i));
                this->deck.push_back(c);
            }
        }
    }
    // construct a deck consisting of N_DECKS of N_CARDS, shuffled
    void initializeDeck() {
        this->deck.clear();
        for (auto i = 0; i < N_DECKS; i++) {
            this->dealAnotherDeck();
        }

        std::shuffle(this->deck.begin(), this->deck.end(), rng::custom_random);
    }
    // number of Cards in Deck
    int getSize() { return static_cast<int>(this->deck.size()); }
    // removes the Card dealt from Deck
    Card deal() {
        // more than half of all the cards are dealt
        if (this->deck.size() < N_DECKS / 2 * N_CARDS) {
            std::cerr << "not enough cards in the deck, redealing...";
            this->initializeDeck();
        }
        Card t = this->deck.back();
        this->deck.pop_back();
        return t;
    }
};
