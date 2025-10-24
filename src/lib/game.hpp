#pragma once

#include <cctype>
#include <iostream>
#include <tui.hpp>

#include "dealer.hpp"
#include "deck.hpp"
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
    Deck deck;      // Deck of cards in the game

    // public:
    Game() { this->deck.initializeDeck(); }
    void deal1_dealer() { this->dealer.addCard(this->deal1()); }
    void deal1_player() { this->player.addCard(this->deal1()); }
    Card deal1() { return this->deck.deal(); }
    bool dealDealer() {
        while (this->dealer.getSum() < 17) {
            this->deal1_dealer();
            if (this->hasEnded()) {
                return false;
            }
        }
        return true;
    }
    bool hasEnded() { return this->getWinner() != 'f'; }
    bool handleWins() {
        switch (getWinner()) {
            case 'f':
                return false;
            case 'd':
                this->player.incrementLoses();
                return true;
            case 'p':
                this->player.incrementWins();
                this->player.addCash((this->player.getBet() * 2));
                return true;
            case 'e':
                this->player.addCash(this->player.getBet());
                return true;
            default:
                return false;
        }
    }
    // 'f': false, 'p': player, 'd': dealer, 'e': equals
    char getWinner() {
        auto dealer_sum = this->dealer.getSum();
        auto player_sum = this->player.getSum();

        if (dealer_sum > 21 || player_sum > 21) {
            // printTop();
            // std::cout << tui::string(Print::bust()).red() << "\n    [Dealer : " << dealer_sum
            // << " | " << this->player.getName() << " : " << player_sum << "]\n";
            if (dealer_sum > 21) {
                return 'p';
            }
            if (player_sum > 21) {
                return 'd';
            }
        } else if (dealer_sum == 21 || player_sum == 21) {
            // printTop();
            // std::cout << tui::string(Print::blackjack()).green() << "\n    [Dealer : " <<
            // dealer_sum
            // << " | " << this->player.getName() << " : " << player_sum << "]\n";
            if (dealer_sum == player_sum) {
                return 'e';
            }
            if (dealer_sum == 21) {
                return 'd';
            }
            if (player_sum == 21) {
                return 'p';
            }
        } else if (dealer_sum >= 17 && this->player.getStood()) {
            if (dealer_sum < player_sum) {
                return 'p';
            }
            if (dealer_sum == player_sum) {
                return 'e';
            }
            return 'd';
        }
        return 'f';
    }
    void printTop() {
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
        std::cout << tui::string(utils::raw_mode_converter(Print::player_border())).cyan();
        this->player.printCards();
        std::cout << color::green_fg() << "\nSum: " << color::red_fg() << this->player.getSum()
                  << style::reset_style() << "\n\r";
        tui::enable_raw_mode();
    }
};
