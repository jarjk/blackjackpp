#pragma once

#include "human.hpp"

class Dealer : public Human {
   public:
    // prints first card revealed and second card hidden
    void printFirstCard() {
        std::cout << "\n";
        std::cout << ".------..------." << "\n";
        std::cout << "|" << this->hand.at(0).getPrintNumber() << ".--. || .--. |" << "\n";
        this->hand.at(0).printCardL1();
        std::cout << "|  //  |" << "\n";
        this->hand.at(0).printCardL2();
        std::cout << "|  //  |" << "\n";
        std::cout << "| '--'" << this->hand.at(0).getPrintNumber() << "|| '--' |" << "\n";
        std::cout << "`------'`------'" << "\n";
    }
};
