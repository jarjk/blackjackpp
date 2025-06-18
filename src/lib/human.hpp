#pragma once

#include <iostream>
#include <vector>

#include "card.hpp"

class Human {
   protected:
    std::vector<Card> hand;
    int sum;

   public:
    Human() : sum(0) {}
    void setAllCards(const std::vector<Card>& manycards) {
        this->clearCards();
        for (const auto& card : manycards) {
            this->addCard(card);
        }
    };
    std::vector<Card> getHand() const { return this->hand; }

    nlohmann::json getHandJson(const bool dealer_secret = false) {
        nlohmann::json hand_json;
        std::vector<nlohmann::json> cards_vector;
        int sum = this->getSum();

        if (dealer_secret && !this->hand.empty()) {
            auto first_card = this->hand[0];
            cards_vector.push_back(first_card.toJson());
            Human tmp_d;
            tmp_d.addCard(first_card);
            sum = tmp_d.getSum();  // overwrite
        } else {
            for (const auto& c : this->hand) {
                cards_vector.push_back(c.toJson());
            }
        }
        hand_json["cards"] = std::move(cards_vector);
        hand_json["sum"] = sum;

        return hand_json;
    }
    int getSum() {
        switchAce();
        return this->sum;
    }
    // Switches Ace between 1 and 11
    void switchAce() {
        if (this->sum > 21) {
            for (auto& i : this->hand) {
                if (i.getNumber() == 1 && !(i.getBlock())) {
                    i.setBlock(true);
                    this->sum -= 10;
                    return;
                }
            }
        }
    }
    void addCard(Card c) {
        this->hand.push_back(c);
        if (c.getNumber() > 10) {
            c.setNumber(10);
        } else if (c.getNumber() == 1) {
            c.setNumber(11);
        }
        this->sum += c.getNumber();
    }
    void clearCards() {
        this->hand.clear();
        this->sum = 0;
    }
    void printCards() {
        std::cout << "\n";
        for (int i = 0; i < 6; i++) {
            for (auto& j : this->hand) {
                switch (i) {
                    case 0:
                        std::cout << ".------.";
                        break;
                    case 1:
                        std::cout << "|" << j.getPrintNumber() << ".--. |";
                        break;
                    case 2:
                        j.printCardL1();
                        break;
                    case 3:
                        j.printCardL2();
                        break;
                    case 4:
                        std::cout << "| '--'" << j.getPrintNumber() << "|";
                        break;
                    case 5:
                        std::cout << "`------'";
                    default:
                        break;
                }
            }
            std::cout << "\n";
        }
    }
};
