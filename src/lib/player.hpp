#pragma once

#include <string>

#include "human.hpp"

class Player : public Human {
   private:
    std::string name = "Unknown";  // Name of Player
    int cash = 1000, bet = 0;      // Player's Cash, Player's Bet
    int wins = 0, loses = 0;       // Player's Stats (number of wins and loses)

   public:
    std::string getName() const { return this->name; }
    int getBet() const { return this->bet; }
    int getCash() const { return this->cash; }
    int getWins() const { return this->wins; }
    int getLoses() const { return this->loses; }
    void setName(std::string nm) { this->name = std::move(nm); }
    void setBet(int b) { this->bet = b; }
    void makeBet(int b) {
        this->cash -= b;
        this->bet += b;
    }
    void setCash(int cash) { this->cash = cash; }
    void addCash(int c) { this->cash += c; }
    void incrementWins() { this->wins += 1; }
    void incrementLoses() { this->loses += 1; }
    void clearCards() { Human::clearCards(); }
};
