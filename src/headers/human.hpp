#pragma once

#include <vector>

#include "card.h"

class Human {
   protected:
    std::vector<Card> hand;
    int sum;

   public:
    Human();
    int getSum();
    void switchAce();
    void addCard(Card c);
    void clearCards();
    void printCards();
};
