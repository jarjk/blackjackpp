#pragma once

#include <sstream>

#include "string"
#include "tui.hpp"

namespace utils {
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
