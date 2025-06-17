#pragma once

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
#endif
#include <httplib.h>

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

    inline httplib::Response unwrap_or(const httplib::Result& res,
                                       const std::string& msg = "can't reach server\r\n") {
        if (res) {
            return res.value();
        }
        auto err_resp = httplib::Response();
        err_resp.body = msg;

        return err_resp;
    }

}  // namespace utils
