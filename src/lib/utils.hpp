#pragma once

#include <chrono>
#include <sstream>
#include <tui.hpp>

#include "string"

namespace utils {
    // now as millisecond unix timestamp, string
    inline long long now_s() {
        auto now = std::chrono::system_clock::now();
        return now.time_since_epoch().count() / 1000;
    }

    inline void cls() {
        tui::screen::clear();
        tui::cursor::home();
    }

    inline std::string raw_mode_converter(const std::string& str) {
        std::ostringstream toReturn;

        for (char ch : str) {
            if (ch == '\n') {
                toReturn << "\r\n";
            } else {
                toReturn << ch;
            }
        }
        return toReturn.str();
    }
}  // namespace utils
