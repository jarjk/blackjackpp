#pragma once

#include <string>

#include "input.hpp"
#include "tui.hpp"

inline char read_ch() {
    tui::enable_raw_mode();
    char ch = Input::read_ch();
    tui::disable_raw_mode();
    return ch;
}

struct Print {
    static std::string title_blackjack();
    static std::string begin_menu();
    static std::string statistics();
    static std::string instructions();
    static std::string bust();
    static std::string blackjack();
    static std::string dealer_wins();
    static std::string you_win();
    static std::string draw();
    static std::string dealer_border();
    static std::string player_border();
};
