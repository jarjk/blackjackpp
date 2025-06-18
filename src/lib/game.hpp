#pragma once

#include <algorithm>
#include <cctype>
#include <cstdint>
#include <fstream>
#include <iostream>

#include "print.hpp"
#include "tui.hpp"
#include "utils.hpp"

namespace color = tui::text::color;
namespace style = tui::text::style;

#include <string>

#include "dealer.hpp"
#include "deck.hpp"
#include "player.hpp"
#include "statistics.hpp"

class Game {
    // private:
   public:
    Player player;  // Player in the game (user)
    Dealer dealer;  // Dealer in the game
    Deck deck;      // Deck of cards in the game
    Statistics s;   // Leaderboard

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
    char compareSum() {
        tui::screen::clear();
        tui::cursor::home();
        if (this->player.getSum() > this->dealer.getSum()) {
            // printTop();
            std::cout << tui::string(utils::raw_mode_converter(Print::you_win())).yellow();
            // "\n    (Dealer has" << this->dealer.getSum() << ")\n";
            return 'p';
        }
        if (this->dealer.getSum() > this->player.getSum()) {
            // printTop();
            std::cout << tui::string(utils::raw_mode_converter(Print::dealer_wins())).red();
            // << "\n    (" << this->dealer.getSum() << ")\n";
            return 'd';
        }
        // printTop();
        std::cout << tui::string(utils::raw_mode_converter(Print::draw())).magenta();
        return 'n';
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
    bool startBet() {
        tui::screen::clear();
        tui::cursor::home();
        if (this->player.getCash() > 0) {
            while (true) {
                printTop();
                std::cout << "Place your bet!\t\t $" << tui::string(this->player.getBet()).green()
                          << "\r\n[W = Raise Bet | S = Decrease Bet | R = Done]\n";
                switch (toupper(Input::read_ch())) {
                    case 'W':
                        if (this->player.getCash() >= 5) {
                            this->player.setBet(5);
                        }
                        break;
                    case 'S':
                        if (this->player.getBet() >= 5) {
                            this->player.setBet(-5);
                        }
                        break;
                    case 'R':
                    case '\n':
                        return true;
                    default:
                        break;
                }
            }
            return true;
        }
        return false;
    }
    bool startGame() {
        this->player.addCard(this->deck.deal());
        this->dealer.addCard(this->deck.deal());
        this->player.addCard(this->deck.deal());
        this->dealer.addCard(this->deck.deal());
        printBody();
        if (handleWins()) {
            return false;
        }
        while (true) {
            std::cout << tui::string("\n\nH : Hit | S : Stand\n").yellow();
            int c = toupper(read_ch());
            if (c == 72) {
                this->player.addCard(this->deck.deal());
                printBody();
                if (handleWins()) {
                    return false;
                }
            } else if (c == 83) {
                break;
            }
        }
        return true;
    }
    void beginGame() {
        char cont = 0;
        do {
            if (this->deck.getSize() < 36) {
                this->deck.initializeDeck();
            }
            this->player.clearCards();
            this->dealer.clearCards();
            if (!startBet()) {
                std::cout << tui::string("\nBankrupt! Game over.\n").yellow();
                break;
            }
            if (startGame()) {
                if (dealDealer()) {
                    switch (compareSum()) {
                        case 'p':
                            this->player.incrementWins();
                            this->player.addCash((this->player.getBet() * 2));
                            break;
                        case 'd':
                            this->player.incrementLoses();
                            break;
                        case 'n':
                            this->player.addCash(this->player.getBet());
                            break;
                        default:
                            break;
                    }
                }
            }
            std::cout << tui::string(Print::dealer_border()).red();
            this->dealer.printCards();
            std::cout << tui::string(Print::player_border()).cyan();
            this->player.printCards();
            std::cout << color::yellow_fg() << "\nYour wins: " << this->player.getWins() << color::red_fg()
                      << "\nYour loses: " << this->player.getLoses() << style::reset_style() << "\n";
            if (s.check(this->player)) {
                std::cout << tui::string("High Score!\n").yellow();
            }
            std::cout << "\nContinue playing? [Y/N]: ";
            std::cin >> cont;
        } while (cont != 'N' && cont != 'n');
        char saveChoice = 0;
        std::cout << "\nSave game? [Y/N]: ";
        std::cin >> saveChoice;
        if (saveChoice == 'Y' || saveChoice == 'y') {
            saveGame();
        }
    }
    void beginMenu(const std::string &message = "") {
        tui::screen::clear();
        tui::cursor::home();
        std::cout << tui::string(Print::title_blackjack()).yellow() << "\n";
        std::cout << Print::begin_menu() << "\n";
        if (!message.empty()) {
            std::cout << tui::string(message).red() << "\n";
        }
        char c = 0;
        std::cout << "Input : ";
        std::cin >> c;
        std::string nm;
        switch (c) {
            case '1':
                std::cout << "Enter player name: ";
                std::cin >> nm;
                this->player.setName(nm);
                beginGame();
                break;
            case '2':
                loadGame();
                beginGame();
                break;
            case '3':
                printStatistics();
                beginMenu();
                break;
            case '4':
                printInstructions();
                beginMenu();
                break;
            case '5':
                exit(0);
                break;
            default:
                beginMenu("Invalid input.");
        }
    }
    void saveGame() {
        std::fstream f1;
        std::fstream f2;
        std::string filename;
        std::string path = "data/";
        do {
            std::cout << "Enter filename: ";
            std::cin >> filename;
            std::transform(filename.begin(), filename.end(), filename.begin(), ::tolower);
        } while (filename == "statistics");
        path += filename + ".bin";
        std::string sName = this->player.getName();
        int sCash = this->player.getCash();
        int sWins = this->player.getWins();
        int sLoses = this->player.getLoses();
        int nameSize = static_cast<int>(sName.size());
        f2.open(path, std::ios::in | std::ios::binary);
        if (!f2.fail()) {
            char choice = 0;
            std::cout << tui::string("File already exists.").red()
                      << " Do you want to overwrite it? [Y/N]: ";
            std::cin >> choice;
            if (choice == 'N' || choice == 'n') {
                saveGame();
            }
        }
        f2.close();
        f1.open(path, std::ios::out | std::ios::binary);
        f1.write(reinterpret_cast<char *>(&nameSize), sizeof(nameSize));
        f1.write(sName.c_str(), static_cast<int64_t>(sName.size()));
        f1.write(reinterpret_cast<char *>(&sCash), sizeof(sCash));
        f1.write(reinterpret_cast<char *>(&sWins), sizeof(sWins));
        f1.write(reinterpret_cast<char *>(&sLoses), sizeof(sLoses));
        f1.close();
    }
    void loadGame() {
        std::fstream f1;
        std::string filename;
        std::string path = "data/";
        do {
            std::cout << "Enter filename: ";
            std::cin >> filename;
            std::transform(filename.begin(), filename.end(), filename.begin(), ::tolower);
        } while (filename == "statistics");
        path += filename + ".bin";
        f1.open(path, std::ios::in | std::ios::binary);
        if (!f1.fail()) {
            std::string sName;
            int sCash = 0;
            int sWins = 0;
            int sLoses = 0;
            int nameSize = 0;
            f1.read(reinterpret_cast<char *>(&nameSize), sizeof(nameSize));
            sName.resize(nameSize);
            f1.read(&sName.at(0), static_cast<int64_t>(sName.size()));
            f1.read(reinterpret_cast<char *>(&sCash), sizeof(sCash));
            f1.read(reinterpret_cast<char *>(&sWins), sizeof(sWins));
            f1.read(reinterpret_cast<char *>(&sLoses), sizeof(sLoses));
            f1.close();
            player.setName(sName);
            player.addCash(sCash - player.getCash());
            while (player.getWins() != sWins) {
                player.incrementWins();
            }
            while (player.getLoses() != sLoses) {
                player.incrementLoses();
            }
        } else {
            beginMenu("File does not exist.");
        }
    }
    void printStatistics() {
        tui::screen::clear();
        tui::cursor::home();
        std::cout << tui::string(Print::title_blackjack()).yellow() << "\n";
        std::cout << "\n" << tui::string(Print::statistics()).green() << "\n";
        s.print();
        std::cout << "\n\n\t(Press any key to continue)\n";
        read_ch();
    }
    static void printInstructions() {
        tui::screen::clear();
        tui::cursor::home();
        std::cout << tui::string(Print::title_blackjack()).yellow() << "\n";
        std::cout << "\n" << tui::string(Print::instructions()).green() << "\n";
        read_ch();
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
