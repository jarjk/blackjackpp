#pragma once

#include <iostream>
#include <vector>

#include "card.hpp"

class Human {
   protected:
    std::vector<Card> hand;
    int sum = 0;

   public:
    void setAllCards(const std::vector<Card>& manycards) { this->hand = manycards; };
    std::vector<Card> getHand() const { return this->hand; }

    int getSum() const { return this->sum; }
    void setSum(int sum) { this->sum = sum; }
    void addCard(Card c) { this->hand.push_back(c); }
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
