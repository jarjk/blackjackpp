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

    int getSum() {
        // switchAce();
        return this->sum;
    }
    void setSum(int sum) { this->sum = sum; }
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
