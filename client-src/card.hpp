#pragma once

#include <array>
#include <iostream>
#include <nlohmann/json.hpp>
#include <string>

class Card {
   private:
    int number = 0;    // Card Number
    char suit = '\0';  // Card Suit

   public:
    // Getter Functions
    int getNumber() const { return this->number; }
    char getSuit() const { return this->suit; }
    // Setter Functions
    void setNumber(int no) { this->number = no; }
    void setSuit(char c) { this->suit = c; }
    // Printing Card Details
    char getPrintNumber() const {
        switch (this->number) {
            case 10:
                return 'X';
            case 11:
                return 'J';
            case 12:
                return 'Q';
            case 13:
                return 'K';
            case 14:
                return 'A';
            default:
                std::array<char, 10> digits = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'};
                return digits.at(this->number);
        }
    }
    void printCardL1() const {
        switch (this->suit) {
            case 'C':
                std::cout << "| :(): |";
                break;
            case 'H':
                std::cout << "| (\\/) |";
                break;
            case 'D':
            case 'S':
                std::cout << "| :/\\: |";
                break;
            default:
                std::cout << "|  //  |";
        }
    }
    void printCardL2() const {
        switch (this->suit) {
            case 'C':
                std::cout << "| ()() |";
                break;
            case 'H':
            case 'D':
                std::cout << "| :\\/: |";
                break;
            case 'S':
                std::cout << "| (__) |";
                break;
            default:
                std::cout << "|  //  |";
        }
    }
};

#define STOI(str_num, integer_num) \
    if (rank == (str_num)) {       \
        rank_num = integer_num;    \
    } else

inline void from_json(const nlohmann::json& j, Card& c) {
    std::string rank = j["rank"];
    int rank_num = 0;
    STOI("A", 14) STOI("K", 13) STOI("Q", 12) STOI("J", 11) { rank_num = std::stoi(rank); }
    c.setNumber(rank_num);
    c.setSuit(j.at("suit").get<std::string>().at(0));
}
