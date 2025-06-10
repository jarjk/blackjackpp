#include "headers/game.hpp"
#include "print.hpp"
#include "tui.hpp"
#include "input.hpp"
#include <vector>
#include <iostream>
#include <fstream>
#include <algorithm>
#include <fstream>
#include <iostream>

#include "headers/color.hpp"
#include "headers/compatible.hpp"
#include "headers/print.hpp"

namespace color = tui::text::color;
namespace style = tui::text::style;

//////////////* Default Constructor *////

Game::Game() { deck.initializeDeck(); }

//////////////* Deals dealer towards the end *////

bool Game::dealDealer() {
    if (dealer.getSum() < player.getSum()) {
        while (dealer.getSum() < 17) {
            dealer.addCard(deck.deal());
            if (checkWins()) {
                return false;
            }
        }
        return true;
    }
    if (checkWins()) {
        return false;
    }
    return true;
}

//////////////* Checkers *////

char Game::compareSum() {
    if (player.getSum() > dealer.getSum()) {
        printTop();
        std::cout << tui::string(Print::you_win()).yellow() << style::reset_style() << "\n    (Dealer has "<<dealer.getSum()<<")\n";
        return 'p';
    }
    if (dealer.getSum() > player.getSum()) {
        printTop();
        std::cout << tui::string(Print::dealer_wins()).red() <<"\n    ("<<dealer.getSum()<<")\n";
        return 'd';
    }
    else{
        printTop();
        std::cout<<tui::string(Print::draw()).magenta();
        return 'n';
    }
}

bool Game::checkWins() {
    switch (checkEnd()) {
        case 'f':
            return false;
        case 'd':
            player.incrementLoses();
            return true;
        case 'p':
            player.incrementWins();
            player.addCash((player.getBet() * 2));
            return true;
    }
    return false;
}

char Game::checkEnd() {
    if (dealer.getSum() > 21 || player.getSum() > 21) {
        printTop();
        std::cout<<tui::string(Print::bust()).red()<<"\n    [Dealer : "<<dealer.getSum()<<" | "<<player.getName()<<" : "<<player.getSum()<<"]\n";
        if(dealer.getSum()>21){
            return 'p';
        }
        if (player.getSum() > 21) {
            return 'd';
        }
    } else if (dealer.getSum() == 21 || player.getSum() == 21) {
        printTop();
        std::cout<<tui::string(Print::blackjack()).green()<<"\n    [Dealer : "<<dealer.getSum()<<" | "<<player.getName()<<" : "<<player.getSum()<<"]\n";
        if(dealer.getSum()==21){
            return 'd';
        }
        if (player.getSum() == 21) {
            return 'p';
        }
    }
    return 'f';
}

//////////////* Game Starters *////

bool Game::startBet() {
    if (player.getCash() > 0) {
        while (true) {
            printTop();
            std::cout<<"Place your bet!\t\t $"<<tui::string(player.getBet()).green()<<"\n[W = Raise Bet | S = Decrease Bet | R = Done]\n";
            int c = toupper(read_ch());
            switch(c){
                case 87: if(player.getCash()>=5){
                            player.setBet(5);
                         }
                         break;
                case 83: if(player.getBet()>=5){
                            player.setBet(-5);
                         }
                         break;
            }
        }
        return true;
    }
    return false;
}

bool Game::startGame() {
    player.addCard(deck.deal());
    dealer.addCard(deck.deal());
    player.addCard(deck.deal());
    dealer.addCard(deck.deal());
    printBody();
    if (checkWins()) {
        return false;
    }
    while(true){
        std::cout << tui::string("\n\nH : Hit | S : Stand\n").yellow();
        int c = toupper(read_ch());
        if(c==72){
            player.addCard(deck.deal());
            printBody();
            if (checkWins()) {
                return false;
            }
        } else if (c == 83) {
            break;
        }
    }
    return true;
}

void Game::beginGame() {
    char cont = 0;
    do {
        if (deck.getSize() < 36) {
            deck.initializeDeck();
        }
        player.clearCards();
        dealer.clearCards();
        if(!startBet()){
            std::cout<<tui::string("\nBankrupt! Game over.\n").yellow();
            break;
        }
        if (startGame()) {
            if (dealDealer()) {
                switch (compareSum()) {
                    case 'p':
                        player.incrementWins();
                        player.addCash((player.getBet() * 2));
                        break;
                    case 'd':
                        player.incrementLoses();
                        break;
                    case 'n':
                        player.addCash(player.getBet());
                        break;
                }
            }
        }
        std::cout<<tui::string(Print::dealer_border()).red();
        dealer.printCards();
        std::cout<<tui::string(Print::player_border()).cyan();
        player.printCards();
        std::cout << color::yellow_fg() << "\nYour wins: " << player.getWins()<< color::red_fg() <<"\nYour loses: "<<player.getLoses()<<style::reset_style()<<"\n";
        if(s.check(player)){
            std::cout<<  tui::string("High Score!\n").yellow();
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

//////////////* Main Method to be Called *////

void Game::beginMenu(bool rep, std::string message){
    tui::screen::clear();
    tui::cursor::home();
    std::cout<<tui::string(Print::title_blackjack()).yellow()<<"\n";
    std::cout<<Print::begin_menu()<<"\n";
    if(rep){
        std::cout<<tui::string(message).red()<<"\n";
    }
    char c = 0;
    std::cout << "Input : ";
    std::cin >> c;
    std::string nm;
    switch (c) {
        case '1':
            std::cout << "Enter player name: ";
            std::cin >> nm;
            player.setName(nm);
            beginGame();
            break;
        case '2':
            loadGame();
            beginGame();
            break;
        case '3':
            printStatistics();
            beginMenu(false, "");
            break;
        case '4':
            printInstructions();
            beginMenu(false, "");
            break;
        case '5':
            exit(0);
            break;
        default:
            beginMenu(true, "Invalid input.");
    }
}

//////////////* Data File Handling *////

void Game::saveGame() {
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
    std::string sName = player.getName();
    int sCash = player.getCash();
    int sWins = player.getWins();
    int sLoses = player.getLoses();
    int nameSize = sName.size();
    f2.open(path, std::ios::in | std::ios::binary);
    if(!f2.fail()){
        char choice;
        std::cout<<tui::string("File already exists.").red()<<" Do you want to overwrite it? [Y/N]: ";
        std::cin>>choice;
        if(choice == 'N' || choice == 'n'){
            saveGame();
        }
    }
    f2.close();
    f1.open(path, std::ios::out | std::ios::binary);
    f1.write((char*)&nameSize, sizeof(nameSize));
    f1.write(sName.c_str(), sName.size());
    f1.write((char*)&sCash, sizeof(sCash));
    f1.write((char*)&sWins, sizeof(sWins));
    f1.write((char*)&sLoses, sizeof(sLoses));
    f1.close();
}

void Game::loadGame() {
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
        f1.read((char*)&nameSize, sizeof(nameSize));
        sName.resize(nameSize);
        f1.read(&sName[0], sName.size());
        f1.read((char*)&sCash, sizeof(sCash));
        f1.read((char*)&sWins, sizeof(sWins));
        f1.read((char*)&sLoses, sizeof(sLoses));
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
        beginMenu(true, "File does not exist.");
    }
}

//////////////* Printing Stuff *////

<<<<<<< HEAD
void Game::printStatistics() {
    clearscr();
    std::cout << yellow << Print::title_blackjack() << def << "\n";
    std::cout << "\n" << lightGreen << Print::statistics() << def << "\n";
    s.print();
    std::cout << "\n\n\t(Press any key to continue)\n";
    static_cast<void>(getch());
}

void Game::printInstructions() {
    clearscr();
    std::cout << yellow << Print::title_blackjack() << def << "\n";
    std::cout << "\n" << lightGreen << Print::instructions() << def << "\n";
    static_cast<void>(getch());
}

void Game::printTop() {
    clearscr();
    std::cout << yellow << Print::title_blackjack() << def << "\n";
    std::cout << lightRed << "\t\tCards: " << deck.getSize() << lightGreen
              << " \tCash: " << player.getCash() << lightMagenta << " \tBet: " << player.getBet()
              << lightBlue << " \tName: " << player.getName() << def << "\n\n\n";
=======
void Game::printStatistics(){
    tui::screen::clear();
    tui::cursor::home();
    std::cout<<tui::string(Print::title_blackjack()).yellow()<<"\n";
    std::cout<<"\n"<<tui::string(Print::statistics()).green()<<"\n";
    s.print();
    std::cout<<"\n\n\t(Press any key to continue)\n";
    read_ch();
}

void Game::printInstructions(){
    tui::screen::clear();
    tui::cursor::home();
    std::cout<<tui::string(Print::title_blackjack()).yellow()<<"\n";
    std::cout<<"\n"<<tui::string(Print::instructions()).green()<<"\n";
    read_ch();
}

void Game::printTop(){
    tui::screen::clear();
    tui::cursor::home();
    std::cout<<tui::string(Print::title_blackjack()).yellow()<<"\n";
    std::cout<<color::red_fg()<<"\t\tCards: "<<deck.getSize()<<color::green_fg()<<" \tCash: "<<player.getCash()<<color::magenta_fg()
             <<" \tBet: "<<player.getBet()<<color::blue_fg()<<" \tName: "<<player.getName()<<style::reset_style()<<"\n\n\n";
}

void Game::printBody() {
    printTop();
    std::cout<<tui::string(Print::dealer_border()).red();
    dealer.printFirstCard();
    std::cout<<tui::string(Print::player_border()).cyan();
    player.printCards();
    std::cout << color::green_fg()<< "\nSum: "<<color::red_fg()<< player.getSum()<<style::reset_style()<<"\n";
}
