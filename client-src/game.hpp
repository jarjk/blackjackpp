#pragma once

#include <cctype>
#include <iostream>
#include <tui.hpp>

#include "dealer.hpp"
#include "player.hpp"
#include "print.hpp"
#include "utils.hpp"

namespace color = tui::text::color;
namespace style = tui::text::style;

class Game {
    // private:
   public:
    Player player;  // Player in the game (user)
    Dealer dealer;  // Dealer in the game

    void printTop() const {
        tui::screen::clear();
        tui::cursor::home();
        std::cout << tui::string(utils::raw_mode_converter(Print::title_blackjack())).yellow() << "\n\r";
        // std::cout << color::red_fg() << "\t\tCards: " << deck.getSize() << color::green_fg()
        //           << " \tCash: " << player.getCash() << color::magenta_fg() << " \tBet: " <<
        //           player.getBet()
        //           << color::blue_fg() << " \tName: " << player.getName() << style::reset_style() <<
        //           "\n\r";
        std::cout << color::green_fg() << " \tCash: " << this->player.getCash() << color::magenta_fg()
                  << " \tBet: " << this->player.getBet() << color::blue_fg()
                  << " \tName: " << this->player.getName() << style::reset_style() << "\n\r";
    }
    void printBody() {
        // printTop();
        tui::disable_raw_mode();
        std::cout << tui::string(utils::raw_mode_converter(Print::dealer_border())).red();
        if (dealer.getHand().size() == 1) {
            this->dealer.printFirstCard();
        } else {
            this->dealer.printCards();
        }
        std::cout << color::green_fg() << "\nSum: " << color::red_fg() << this->dealer.getSum()
                  << style::reset_style() << "\n\r";
        std::cout << tui::string(utils::raw_mode_converter(Print::player_border())).cyan();
        this->player.printCards();
        std::cout << color::green_fg() << "\nSum: " << color::red_fg() << this->player.getSum()
                  << style::reset_style() << "\n\r";
        tui::enable_raw_mode();
    }
};
