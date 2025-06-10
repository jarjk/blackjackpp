#pragma once

#include <array>
#include <string>

#include "player.hpp"
#include "tui.hpp"

class PlayerSet {
   private:
    std::string name;       // Name of Player
    int cash, wins, loses;  // Stat Data
    // This class is almost similar to Player, but does not need vectors and betting values.

   public:
    PlayerSet();
    std::string getName();
    int getCash() const;
    int getWins() const;
    int getLoses() const;
    void setValues(std::string nm, int c, int w, int l);
};

class Statistics {
   private:
    std::array<PlayerSet, 3> p;  // 3 Players

   public:
    Statistics();
    bool check(Player pl);
    void print();
    void saveStats();
    void loadStats();
};
