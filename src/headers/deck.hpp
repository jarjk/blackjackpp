#pragma once

#include <array>
#include <cassert>
#include <vector>

#include "card.h"

using Cards = std::vector<Card>;

class Deck {
   private:
    const static unsigned short N_DECKS = 4;
    static const unsigned short N_CARDS = 52;
    Cards deck;  // Deck (Vector) of Cards

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
    void initializeDeck();
    int getSize();
    Card deal();
};
