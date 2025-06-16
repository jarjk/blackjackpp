#pragma once

#include <format>
#include <vector>

#include "card.h"

class Human {
   protected:
    std::vector<Card> hand;
    int sum;

   public:
    Human();
    void setAllCards(const std::vector<Card>& manycards) {
        this->clearCards();
        for (const auto& card : manycards) {
            this->addCard(card);
        }
    };
    std::vector<Card> getHand() const { return this->hand; }
    std::string dbg_cards() const {
        std::string tmp;
        for (const auto &c : this->hand) {
            tmp += c.dbg() + ',';
        }
        tmp += std::format("sum: {}", this->sum);
        return tmp;
    }
    int getSum();
    void switchAce();
    void addCard(Card c);
    void clearCards();
    void printCards();
};
