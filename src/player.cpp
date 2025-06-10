#include "headers/player.hpp"

#include <utility>

//////////////* Default Constructor *////

Player::Player() : name("Unknown"), cash(1000), bet(0), wins(0), loses(0) {}

//////////////* Getter Functions *////

// Returns name of Player
std::string Player::getName() { return name; }

// Returns amount of bet
int Player::getBet() const { return bet; }

// Returns Player's cash amount
int Player::getCash() const { return cash; }

// Returns Player's statistic (number of wins)
int Player::getWins() const { return wins; }

// Returns Player's statistic (number of loses)
int Player::getLoses() const { return loses; }

//////////////* Setter Functions *////

// Sets name of Player
void Player::setName(std::string nm) { name = std::move(nm); }

// Sets bet for game
void Player::setBet(int b) {
    cash -= b;
    bet += b;
}

// Adds cash to Player's cash amount
void Player::addCash(int c) { cash += c; }

// Increments Player's number of wins by one
void Player::incrementWins() { wins += 1; }

// Increments Player's number of loses by one
void Player::incrementLoses() { loses += 1; }

//////////////* Game Functions *////

// Clears player's hand
void Player::clearCards() {
    Human::clearCards();
    bet = 0;
}
