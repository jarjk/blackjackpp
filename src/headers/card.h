#pragma once

#include <format>
#include <string>

#include "crow/json.h"
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
