#pragma once

#include <format>
#include <vector>

#include "card.h"
#include "crow/json.h"

class Human {
   protected:
    std::vector<Card> hand;
    int sum;

   public:
    Human();
    std::vector<Card> getHand() const { return this->hand; }

    crow::json::wvalue getHandJson() {
        crow::json::wvalue hand_json;

        std::vector<crow::json::wvalue> cards_vector;
        cards_vector.reserve(this->hand.size());
        for (const auto& c : this->hand) {
            cards_vector.push_back(c.toJson());
        }

        hand_json["cards"] = std::move(cards_vector);
        hand_json["sum"] = this->getSum();

        return hand_json;
    }
    int getSum();
    void switchAce();
    void addCard(Card c);
    void clearCards();
    void printCards();
};
