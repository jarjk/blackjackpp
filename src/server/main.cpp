#include <httplib.h>

#include <cctype>
#include <csignal>
#include <cstddef>
#include <iostream>
#include <nlohmann/json.hpp>
#include <string>

#include "gameManager.hpp"
#include "print.hpp"

GameManager manager;
httplib::Server svr;

// stop server
void signal_handler(int /*signal*/) {
    std::cout << "\nReceived signal, shutting down gracefully...\n";
    svr.stop();
}
// GET /join?username=...
void join(const httplib::Request& req, httplib::Response& res) {
    if (!req.has_param("username")) {
        res.status = 400;
        res.set_content("missing username", "text/plain");
        return;
    }

    std::string uname = req.get_param_value("username");

    auto [p, joined_already] = manager.join_game(uname);
    if (joined_already) {
        res.status = 400;
        res.set_content("already joined", "text/plain");
        return;
    }

    // ok (200) with empty body
    res.status = 200;
    res.set_content("", "text/plain");
}

// POST /bet/<username>?amount=<int>
void bet(const httplib::Request& req, httplib::Response& res) {
    std::string uname;
    if (!req.matches.empty() && req.matches.size() > 1) {
        uname = req.matches[1];
    } else {
        res.status = 400;
        res.set_content("missing username", "text/plain");
        return;
    }

    if (!manager.already_joined(uname)) {
        res.status = 400;
        res.set_content("not joined", "text/plain");
        return;
    }

    auto lock = manager.acquire_lock();
    auto& game = manager.players[uname].game;
    auto& player = game.player;
    if (!player.getHand().empty() && !game.hasEnded()) {
        res.status = 400;
        res.set_content("dont bet during zhe game", "text/plain");
        return;
    }

    if (!req.has_param("amount")) {
        res.status = 400;
        res.set_content("missing or incorrect amount of bet", "text/plain");
        return;
    }

    std::string amount_s = req.get_param_value("amount");
    int amount = 0;
    try {
        amount = std::stoi(amount_s);
    } catch (...) {
        amount = 0;
    }

    if (amount == 0 || amount > player.getCash()) {
        res.status = 400;
        res.set_content("missing or incorrect amount of bet", "text/plain");
        return;
    }

    player.clearCards();  // reset game
    game.dealer.clearCards();
    player.setBet(amount);
    std::cerr << "new game\n";

    // dealing starting cards
    game.deal1_dealer();
    game.deal1_player();
    game.deal1_dealer();
    game.deal1_player();

    nlohmann::json out;
    out["cash"] = player.getCash();
    out["hand"] = player.getHandJson();
    out["dealer"] = game.dealer.getHandJson(!game.handleWins());
    out["winner"] = std::format("{}", game.getWinner());

    res.status = 200;
    res.set_content(out.dump(), "application/json");
}

// POST /move/<username>?action=[hit, stand]
void move(const httplib::Request& req, httplib::Response& res) {
    std::string uname;
    if (!req.matches.empty() && req.matches.size() > 1) {
        uname = req.matches[1];
    } else {
        auto pos = req.path.find_last_of('/');
        if (pos != std::string::npos && pos + 1 < req.path.size()) {
            uname = req.path.substr(pos + 1);
        }
    }

    auto lock = manager.acquire_lock();
    if (!req.has_param("action")) {
        res.status = 400;
        res.set_content("missing action parameter", "text/plain");
        return;
    }

    std::string action_s = req.get_param_value("action");
    auto& game = manager.players[uname].game;
    if (game.hasEnded() || game.player.getBet() == 0) {
        res.status = 400;
        res.set_content("game finished or forgot to bet", "text/plain");
        return;
    }

    if (action_s == "hit" || action_s == "h") {
        game.deal1_player();
    } else if (action_s == "stand" || action_s == "s") {
        std::cerr << "standing...\n";
        game.player.setStood(true);
        game.dealDealer();
    } else {
        res.status = 400;
        res.set_content("should 'hit' or 'stand'", "text/plain");
        return;
    }

    nlohmann::json out;
    bool has_ended = false;
    if (game.handleWins()) {
        has_ended = true;
        out["winner"] = std::format("{}", game.getWinner());
    }
    out["hand"] = game.player.getHandJson();
    out["dealer"] = game.dealer.getHandJson(!has_ended);

    res.status = 200;
    res.set_content(out.dump(), "application/json");
}

int main() {
    const char* log_v = std::getenv("BJ_LOG");
    size_t log_l = 512;
    if (log_v != nullptr && std::string(log_v) == "verbose") {
        log_l = std::string::npos;
    } else {
        std::cerr << "setting env var 'BJ_LOG' to 'verbose' will enable verbose logging\n";
    }

    signal(SIGINT, signal_handler);

    std::cerr << utils::now_s() << " starting server initialisation\n";
    svr.set_logger([&log_l](const httplib::Request& req, const httplib::Response& res) {
        std::cerr << utils::now_s() << " " << req.remote_addr << ':' << req.remote_port << " ["
                  << req.method << "] " << req.target << " -> " << res.status << ", \""
                  << res.body.substr(0, log_l) << "...\"\n";
    });

    // GET /join?username=...
    svr.Get("/join", join);

    // POST /bet/<username>?amount=<int>
    // Use regex route so we capture username as first match group
    svr.Post(R"(/bet/(.+))", bet);

    // POST /move/<username>?action=[hit, stand]
    svr.Post(R"(/move/(.+))", move);

    // GET /game_state
    svr.Get("/game_state", [](const httplib::Request&, httplib::Response& res) {
        res.set_content(manager.get_game_state().dump(), "application/json");
    });

    // GET /help
    svr.Get("/help", [](const httplib::Request&, httplib::Response& res) {
        res.set_content(Print::instructions(), "text/plain");
    });

    std::cerr << "starting server on 0.0.0.0:18080\n";
    svr.listen("0.0.0.0", 18080);
    return 0;
}
