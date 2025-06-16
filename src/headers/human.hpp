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

    crow::json::wvalue getHandJson(const bool dealer_secret = false) {
        crow::json::wvalue hand_json;
        std::vector<crow::json::wvalue> cards_vector;
        int sum = this->getSum();

        if (dealer_secret && !this->hand.empty()) {
            auto first_card = this->hand[0];
            cards_vector.push_back(first_card.toJson());
            Human tmp_d;
            tmp_d.addCard(first_card);
            sum = tmp_d.getSum(); // overwrite
        } else {
            for (const auto& c : this->hand) {
                cards_vector.push_back(c.toJson());
            }
        }
        hand_json["cards"] = std::move(cards_vector);
        hand_json["sum"] = sum;

        return hand_json;
    }
    int getSum();
    void switchAce();
    void addCard(Card c);
    void clearCards();
    void printCards();
};
