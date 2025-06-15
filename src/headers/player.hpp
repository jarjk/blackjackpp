#pragma once

#include <string>

#include "human.hpp"

class Player : public Human {
   private:
    std::string name;  // Name of Player
    int cash, bet;     // Player's Cash, Player's Bet
    int wins, loses;   // Player's Stats (number of wins and loses)
    bool stood = false;

   public:
    Player();
    std::string getName() const;
    int getBet() const;
    int getCash() const;
    int getWins() const;
    int getLoses() const;
    bool getStood() const { return this->stood; }
    void setStood(const bool stood) { this->stood = stood; }
    void setName(std::string nm);
    void setBet(int b);
    void addCash(int c);
    void incrementWins();
    void incrementLoses();
    void clearCards();
};
