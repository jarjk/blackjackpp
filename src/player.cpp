#include "headers/player.hpp"

#include <iostream>
#include <utility>

//////////////* Default Constructor *////

Player::Player() : name("Unknown"), cash(1000), bet(0), wins(0), loses(0) {}

//////////////* Getter Functions *////

// Returns name of Player
std::string Player::getName() const { return this->name; }

// Returns amount of bet
int Player::getBet() const { return this->bet; }

// Returns Player's cash amount
int Player::getCash() const { return this->cash; }

// Returns Player's statistic (number of wins)
int Player::getWins() const { return this->wins; }

// Returns Player's statistic (number of loses)
int Player::getLoses() const { return this->loses; }

//////////////* Setter Functions *////

// Sets name of Player
void Player::setName(std::string nm) { this->name = std::move(nm); }

// Sets bet for game
void Player::setBet(int b) {
    std::cerr << std::format("setting bet, cash: {}, bet: {}\n", this->cash, this->bet);
    this->cash -= b;
    this->bet += b;
    std::cerr << std::format("set bet, cash: {}, bet: {}\n", this->cash, this->bet);
}

// Adds cash to Player's cash amount
void Player::addCash(int c) {
    std::cerr << "adding cash: " << this->cash << " + " << c << "\n";
    this->cash += c;
}

// Increments Player's number of wins by one
void Player::incrementWins() { this->wins += 1; }

// Increments Player's number of loses by one
void Player::incrementLoses() { this->loses += 1; }

//////////////* Game Functions *////

// Clears player's hand
void Player::clearCards() {
    std::cerr << "resetting player's hand\n";
    Human::clearCards();
    this->stood = false;
    this->bet = 0;
}
