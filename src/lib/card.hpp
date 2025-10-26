#pragma once

#include <array>
#include <format>
#include <fstream>
#include <iostream>
#include <nlohmann/json.hpp>
#include <string>

class Card {
   private:
    int number;  // Card Number
    char suit;   // Card Suit
    bool block;  // Boolean value for Ace Switching

   public:
    // Default Constructor
    Card() : number(0), suit('\0'), block(false) {}
    // Parameterised Constructor (for initializing deck)
    Card(int no, char s) : number(no), suit(s), block(false) {}
    // Getter Functions
    int getNumber() const { return this->number; }

    char getSuit() const { return this->suit; }
    bool getBlock() const { return this->block; }
    // Setter Functions
    void setNumber(int no) { this->number = no; }
    void setSuit(char c) { this->suit = c; }
    void setBlock(bool b) { this->block = b; }
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
    nlohmann::json toJson() const {
        // crow::json::wvalue card_json;
        nlohmann::json card_json;
        card_json["number"] = number;
        card_json["suit"] = std::format("{}", suit);
        card_json["block"] = block;
        return card_json;
    }
};

#define STOI(str_num, integer_num) \
    if (rank == (str_num)) {       \
        rank_num = integer_num;    \
    } else

inline void from_json(const nlohmann::json& j, Card& c) {
    std::string rank = j["rank"];
    std::ofstream logf("client.log");
    logf << "desering: '" << j << "'\nrank: '" << rank << "'\n";
    int rank_num = 0;
    STOI("A", 14) STOI("K", 13) STOI("Q", 12) STOI("J", 11) { rank_num = std::stoi(rank); }
    c.setNumber(rank_num);
    c.setSuit(j.at("suit").get<std::string>().at(0));
    if (j.contains("block")) {  // won't ever happen
        c.setBlock(j.at("block").get<bool>());
    }
}
