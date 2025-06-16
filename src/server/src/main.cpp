#include <cctype>
#include <iostream>
#include <string>

#include "crow/app.h"
#include "crow/http_request.h"
#include "crow/http_response.h"
#include "crow/json.h"
#include "gameManager.hpp"
#include "print.hpp"

GameManager manager;

crow::response join(const crow::request& req) {
    const char* username = req.url_params.get("username");
    if (username == nullptr) {
        return {400, "missing username"};
    }
    manager.lock();

    auto [p, joined_already] = manager.join_game(username);
    if (joined_already) {
        return {400, "already joined"};
    }

    return {};  // ok
}

crow::response bet(const crow::request& req, const std::string& name) {
    if (!manager.already_joined(name)) {
        return {400, "not joined"};
    }
    auto& game = manager.players[name].game;
    auto& player = game.player;
    if (!game.player.getHand().empty() && !game.hasEnded()) {
        return {400, "dont bet during zhe game"};
    }

    const char* amount_s = req.url_params.get("amount");
    int amount = 0;
    // clang-format off
    try { amount = std::stoi(amount_s); } catch (...) { }
    // clang-format on
    if (amount_s == nullptr || amount == 0 || amount > player.getCash()) {
        return {400, "missing or incorrect amount of bet"};
    }

    game.player.clearCards();  // reset game
    game.dealer.clearCards();
    player.setBet(amount);
    std::cerr << "new game\n";

    // dealing starting cards
    game.deal1_dealer();
    game.deal1_player();
    game.deal1_dealer();
    game.deal1_player();

    crow::json::wvalue res;
    res["cash"] = player.getCash();
    res["hand"] = player.getHandJson();
    res["dealer"] = game.dealer.getHandJson(!game.hasEnded());
    res["winner"] = std::format("{}", game.getWinner());
    return crow::response(res);
}

crow::response make_move(const crow::request& req, const std::string& username) {
    const char* action = req.url_params.get("action");
    if (action == nullptr) {
        return {400, "missing action parameter"};
    }
    std::string action_s = std::string(action);
    auto& game = manager.players[username].game;
    if (game.hasEnded() || game.player.getBet() == 0) {
        return {400, "game finished or forgot to bet"};
    }
    if (action_s == "hit" || action_s == "h") {
        game.deal1_player();
    } else if (action_s == "stand" || action_s == "s") {
        std::cerr << "standing...\n";
        game.player.setStood(true);
        game.dealDealer();
    } else {
        return {400, "should 'hit' or 'stand'"};
    }
    crow::json::wvalue res;
    res["hand"] = game.player.getHandJson();
    bool has_ended = false;
    // if has ended, updates player status
    if (game.handleWins()) {
        has_ended = true;
        res["winner"] = std::format("{}", game.getWinner());
    }
    res["dealer"] = game.dealer.getHandJson(!has_ended);
    return res;
}

int main() {
    crow::SimpleApp app;

    // GET /join?username=...
    CROW_ROUTE(app, "/join").methods("GET"_method)(join);

    // POST /bet/<username>?amount=<int>
    CROW_ROUTE(app, "/bet/<string>").methods("POST"_method)(bet);

    // POST /move/<username>?action=[hit, stand]
    CROW_ROUTE(app, "/move/<string>").methods("POST"_method)(make_move);

    // GET /game_state
    CROW_ROUTE(app, "/game_state").methods("GET"_method)([]() {
        return crow::response(manager.get_game_state());
    });

    // GET /help
    CROW_ROUTE(app, "/help").methods("GET"_method)([]() { return crow::response(Print::instructions()); });

    app.port(18080).multithreaded().run();
}
