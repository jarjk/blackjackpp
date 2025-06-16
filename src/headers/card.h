#pragma once

#include <format>
#include <string>

#include "crow/json.h"
#include "nlohmann/json.hpp"

class Card {
   private:
    int number;  // Card Number
    char suit;   // Card Suit
    bool block;  // Boolean value for Ace Switching

   public:
    // Default Constructor
    Card();
    // Parameterised Constructor (for initializing deck)
    Card(int no, char s);
    // Getter Functions
    int getNumber() const;

    char getSuit() const;
    bool getBlock() const;
    // Setter Functions
    void setNumber(int no);
    void setSuit(char c);
    void setBlock(bool b);
    // Printing Card Details
    char getPrintNumber() const;
    void printCardL1() const;
    void printCardL2() const;
    crow::json::wvalue toJson() const {
        crow::json::wvalue card_json;
        card_json["number"] = number;
        card_json["suit"] = std::format("{}", suit);
        card_json["block"] = block;
        return card_json;
    }
};

inline void from_json(const nlohmann::json& j, Card& c) {
    c.setNumber(j.at("number").get<int>());
    c.setSuit(j.at("suit").get<std::string>().at(0));
    if (j.contains("block")) {
        c.setBlock(j.at("block").get<bool>());
    }
}
