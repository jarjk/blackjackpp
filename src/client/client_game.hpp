#pragma once

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#endif
#include <httplib.h>

#include <cctype>
#include <input.hpp>
#include <iostream>
#include <nlohmann/json.hpp>
#include <string>
#include <tui.hpp>
#include <utility>

#include "game.hpp"
#include "print.hpp"
#include "utils.hpp"

using json = nlohmann::json;

inline httplib::Response unwrap_or(const httplib::Result& res,
                                   const std::string& msg = "can't reach server\r\n") {
    if (res) {
        return res.value();
    }
    auto err_resp = httplib::Response();
    err_resp.body = msg;

    return err_resp;
}

struct ClientGame {
    Game game;
    Player player;
    bool startBet() {
        tui::screen::clear();
        tui::cursor::home();
        if (this->game.player.getCash() > 0) {
            while (true) {
                this->game.printTop();
                std::cout << "Place your bet!\t\t $" << tui::string(this->game.player.getBet()).green()
                          << "\r\n[W = Raise Bet | S = Decrease Bet | R = Done]\n";
                switch (Input::read_ch()) {
                    case 'w':
                        if (this->game.player.getCash() >= 5) {
                            this->game.player.setBet(5);
                        }
                        break;
                    case 's':
                        if (this->game.player.getBet() >= 5) {
                            this->game.player.setBet(-5);
                        }
                        break;
                    case 'r':
                    case '\n':  // this doesn't work for whatever reason
                    case 13:    // enter
                        if (this->game.player.getBet() == 0) {
                            continue;
                        }
                        return true;
                    default:
                        break;
                }
            }
            return true;
        }
        return false;
    }
    void kinda_beginGame(httplib::Client& cli) {
        auto gs = cli.Get(tui::concat("/game_state"));
        auto gs_json = json::parse(gs.value().body);
        // std::cerr << gs_json << "\n\r";
        // Input::read_ch();
        auto cash = gs_json["games"][this->game.player.getName()]["player"]["wealth"].get<int>();
        this->game.player.setCash(cash);

        if (!this->startBet()) {
            utils::cls();
            tui::cursor::set_position(tui::screen::size().first / 2, (tui::screen::size().second / 2) - 9);
            std::cout << tui::string("Bankrupt! Game over.").yellow();
            Input::read_ch();
            return;  // TODO dont panic
        }

        auto res = cli.Post(
            tui::concat("/bet/", this->game.player.getName(), "?amount=", this->game.player.getBet()));
        // std::cerr << res.value().body;
        if (res.value().status != 200) {
            std::cerr << res.value().body;
            Input::read_ch();
            return;  // TODO dont panic
        }

        auto res_json = json::parse(unwrap_or(res).body);

        this->game.player.setAllCards(res_json["player"]["hand"]["cards"].get<std::vector<Card>>());
        this->game.dealer.setAllCards(res_json["dealer"]["cards"].get<std::vector<Card>>());

        utils::cls();
        switch (res_json["winner"].get<std::string>().at(0)) {
            case 'd':
                std::cout << tui::string(utils::raw_mode_converter(Print::dealer_wins())).red();
                this->game.printBody();
                return;
            case 'p':
                std::cout << tui::string(utils::raw_mode_converter(Print::you_win())).yellow();
                this->game.printBody();
                return;
            case 'e':
                std::cout << tui::string(utils::raw_mode_converter(Print::draw())).magenta();
                this->game.printBody();
                return;
            default:
                break;
        }

        this->game.printTop();
        this->game.printBody();

        tui::cursor::save();
        while (true) {
            tui::cursor::restore();
            std::cout << tui::string("\n\nH : Hit | S : Stand\n").yellow().blink();
            int c = tolower(Input::read_ch());
            std::string action;
            if (c == 'h') {
                action = "hit";
            } else if (c == 's') {
                action = "stand";
            } else {
                continue;
            }

            auto res = cli.Post(tui::concat("/move/", this->game.player.getName(), "?action=", action));

            if (res.value().status != 200) {
                std::cerr << res.value().body;
                Input::read_ch();
                return;  // TODO dont panic
            }
            auto res_json = json::parse(unwrap_or(res).body);

            this->game.player.setAllCards(res_json["player"]["hand"]["cards"].get<std::vector<Card>>());
            this->game.dealer.setAllCards(res_json["dealer"]["cards"].get<std::vector<Card>>());

            if (res_json["winner"].is_null() || res_json["winner"] == "f") {
                this->game.printTop();
                this->game.printBody();
                continue;
            }

            utils::cls();
            switch (res_json["winner"].get<std::string>().at(0)) {
                case 'd':
                    std::cout << tui::string(utils::raw_mode_converter(Print::dealer_wins())).red();
                    this->game.printBody();
                    return;
                case 'p':
                    std::cout << tui::string(utils::raw_mode_converter(Print::you_win())).yellow();
                    this->game.printBody();
                    return;
                case 'e':
                    std::cout << tui::string(utils::raw_mode_converter(Print::draw())).magenta();
                    this->game.printBody();
                    return;
                default:
                    break;
            }
        }

        std::cout << tui::string(utils::raw_mode_converter(Print::dealer_border())).red();
        // TODO get rest of dealers cards
        // this->dealer.printCards();
        std::cout << tui::string(utils::raw_mode_converter(Print::player_border())).cyan();
        this->player.printCards();
        // TODO get wins

        // std::cout << color::yellow_fg() << "\nYour wins: " << this->player.getWins() << color::red_fg()
        // << "\nYour loses: " << this->player.getLoses() << style::reset_style() << "\n";
        // if (s.check(this->player)) {
        // std::cout << tui::string("High Score!\n").yellow();
        // }

        /// STH TODO HERE

        // std::cout << "\nContinue playing? [Y/N]: ";
        // std::cin >> cont;
        // } while (cont != 'N' && cont != 'n');
        // char saveChoice = 0;
        // std::cout << "\nSave game? [Y/N]: ";
        // std::cin >> saveChoice;
        // if (saveChoice == 'Y' || saveChoice == 'y') {
        //     saveGame();
        // }

        Input::read_ch();
    }
};
