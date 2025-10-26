#pragma once

#include <iostream>
#include <string>
#include <utility>

#include "human.hpp"

class Player : public Human {
   private:
    std::string name = "Unknown";  // Name of Player
    int cash = 1000, bet = 0;      // Player's Cash, Player's Bet
    int wins = 0, loses = 0;       // Player's Stats (number of wins and loses)
    bool stood = false;

   public:
    std::string getName() const { return this->name; }
    int getBet() const { return this->bet; }
    int getCash() const { return this->cash; }
    int getWins() const { return this->wins; }
    int getLoses() const { return this->loses; }
    bool getStood() const { return this->stood; }
    void setStood(const bool stood) { this->stood = stood; }
    void setName(std::string nm) { this->name = std::move(nm); }
    void setBet(int b) {
        std::cerr << std::format("setting bet, cash: {}, bet: {}\n", this->cash, this->bet);
        this->cash -= b;
        this->bet += b;
        std::cerr << std::format("set bet, cash: {}, bet: {}\n", this->cash, this->bet);
    }
    void setCash(int cash) { this->cash = cash; }
    void addCash(int c) {
        std::cerr << "adding cash: " << this->cash << " + " << c << "\n";
        this->cash += c;
    }
    void incrementWins() { this->wins += 1; }
    void incrementLoses() { this->loses += 1; }
    void clearCards() {
        std::cerr << "resetting player's hand\n";
        Human::clearCards();
        this->stood = false;
        // this->bet = 0;
    }
};
