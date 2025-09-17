#pragma once

#include <algorithm>
#include <array>
#include <cstdint>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <string>
#include <tui.hpp>
#include <utility>

#include "player.hpp"

// fixes windows compilation, XD
#undef max

namespace color = tui::text::color;
namespace style = tui::text::style;

class PlayerSet {
   private:
    std::string name = "Unknown";          // Name of Player
    int cash = 1000, wins = 0, loses = 0;  // Stat Data
    // This class is almost similar to Player, but does not need vectors and betting values.

   public:
    std::string getName() { return this->name; }
    int getCash() const { return this->cash; }
    int getWins() const { return this->wins; }
    int getLoses() const { return this->loses; }
    void setValues(std::string nm, int c, int w, int l) {
        this->name = std::move(nm);
        this->cash = c;
        this->wins = w;
        this->loses = l;
    }
};

class Statistics {
   private:
    std::array<PlayerSet, 3> p;  // 3 Players

   public:
    Statistics() {
        std::fstream temp;
        temp.open("data/statistics.bin", std::ios::in | std::ios::binary);
        if (temp.fail()) {
            saveStats();
        } else {
            temp.close();
            loadStats();
        }
    }
    bool check(const Player& pl) {
        bool rewrite = false;
        if (pl.getCash() > this->p.at(0).getCash()) {
            this->p.at(0).setValues(pl.getName(), pl.getCash(), pl.getWins(), pl.getLoses());
            rewrite = true;
        }
        if (pl.getWins() > this->p.at(1).getWins()) {
            this->p.at(1).setValues(pl.getName(), pl.getCash(), pl.getWins(), pl.getLoses());
            rewrite = true;
        }
        if (pl.getLoses() > this->p.at(2).getLoses()) {
            this->p.at(2).setValues(pl.getName(), pl.getCash(), pl.getWins(), pl.getLoses());
            rewrite = true;
        }
        if (rewrite) {
            saveStats();
        }
        return rewrite;
    }
    void print() {
        int maxlength =
            static_cast<int>(std::max({this->p.at(0).getName().length(), this->p.at(0).getName().length(),
                                       this->p.at(0).getName().length()}));
        for (int i = 0; i < 3; i++) {
            switch (i) {
                case 0:
                    std::cout << "MAX CASH  ||||||||| ";
                    break;
                case 1:
                    std::cout << "MAX WINS  ||||||||| ";
                    break;
                case 2:
                    std::cout << "MAX LOSES ||||||||| ";
                    break;
                default:
                    break;
            }
            std::cout << std::setw(maxlength + 1) << this->p.at(i).getName() << "\t | \t"
                      << color::green_fg() << "Cash: " << std::setw(7) << this->p.at(i).getCash()
                      << "\t | \t" << color::yellow_fg() << "Wins: " << std::setw(5)
                      << this->p.at(i).getWins() << "\t | \t" << color::red_fg()
                      << "Loses: " << std::setw(5) << this->p.at(i).getLoses() << style::reset_style()
                      << "\n";
        }
    }
    void saveStats() {
        std::fstream f1;
        f1.open("data/statistics.bin", std::ios::out | std::ios::binary);
        for (int i = 0; i < 3; i++) {
            std::string sName = this->p.at(i).getName();
            int nameSize = static_cast<int>(sName.size());
            int sCash = this->p.at(i).getCash();
            int sWins = this->p.at(i).getWins();
            int sLoses = this->p.at(i).getLoses();
            f1.write(reinterpret_cast<char*>(&nameSize), sizeof(nameSize));
            f1.write(sName.c_str(), static_cast<int64_t>(sName.size()));
            f1.write(reinterpret_cast<char*>(&sCash), sizeof(sCash));
            f1.write(reinterpret_cast<char*>(&sWins), sizeof(sWins));
            f1.write(reinterpret_cast<char*>(&sLoses), sizeof(sLoses));
        }
        f1.close();
    }
    void loadStats() {
        std::fstream f1;
        f1.open("data/statistics.bin", std::ios::in | std::ios::binary);
        for (int i = 0; i < 3; i++) {
            std::string sName;
            int nameSize = 0;
            int sCash = 0;
            int sWins = 0;
            int sLoses = 0;
            f1.read(reinterpret_cast<char*>(&nameSize), sizeof(nameSize));
            sName.resize(nameSize);
            f1.read(&sName.at(0), static_cast<int64_t>(sName.size()));
            f1.read(reinterpret_cast<char*>(&sCash), sizeof(sCash));
            f1.read(reinterpret_cast<char*>(&sWins), sizeof(sWins));
            f1.read(reinterpret_cast<char*>(&sLoses), sizeof(sLoses));
            this->p.at(i).setValues(sName, sCash, sWins, sLoses);
        }
        f1.close();
    }
};
