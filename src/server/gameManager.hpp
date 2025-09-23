#include <algorithm>
#include <mutex>
#include <string>
#include <unordered_map>

#include "game.hpp"
#include "player.hpp"

// === Basic Models ===
class ServerGame {
   public:
    // [[nodiscard]] bool getMoveMade() const { return this->move_made; };
    // [[nodiscard]] bool getIsWaiting() const { return this->is_waiting; };
    // [[nodiscard]] std::string getId() const { return this->id; };

    // void setMoveMade(const bool& move_made) { this->move_made = move_made; };
    // void setIsWaiting(const bool& is_waiting) { this->is_waiting = is_waiting; };
    // void setId(std::string id) { this->id = std::move(id); };

    // private:
    Game game;
};

// === Global Game Manager ===
class GameManager {
   private:
    mutable std::mutex mtx;

   public:
    std::unordered_map<std::string, ServerGame> players;

    std::unique_lock<std::mutex> acquire_lock() const {
        return std::unique_lock<std::mutex>(mtx);
    }

    // static std::string generate_id() {
    //     static std::mt19937 rng(std::random_device{}());
    //     static std::uniform_int_distribution<int> dist(0, 15);
    //     std::string id;
    //     for (int i = 0; i < 8; ++i) {
    //         id += std::string("0123456789abcdef").at(dist(rng));
    //     }
    //     return id;
    // }

    // check if player already exists
    bool already_joined(const std::string& uname) const {
        return std::ranges::any_of(this->players, [&uname](const auto& x) { return x.first == uname; });
    }

    std::pair<ServerGame*, bool> join_game(const std::string& name) {
        std::lock_guard<std::mutex> lock(this->mtx);

        if (this->already_joined(name)) {
            return {&players[name], true};
        }

        // Add new player
        // std::string new_id = generate_id();
        ServerGame p;
        // p.game.printInstructions()
        p.game.player.setName(name);

        players.insert({name, p});
        return {&players[name], false};
    }

    nlohmann::json get_game_state() {
        std::lock_guard<std::mutex> lock(this->mtx);
        nlohmann::json res;

        for (auto& [id, p] : players) {
            res["games"][id]["bet"] = p.game.player.getBet();
            res["games"][id]["cash"] = p.game.player.getCash();
            res["games"][id]["loses"] = p.game.player.getLoses();
            res["games"][id]["wins"] = p.game.player.getWins();
            res["games"][id]["hand"] = p.game.player.getHandJson();

            auto has_ended = false;
            if (p.game.hasEnded()) {
                // res["games"][id]["dealers_hand"] = p.game.dealer.dbg_cards();
                has_ended = true;
                res["games"][id]["winner"] = std::format("{}", p.game.getWinner());
            }
            res["games"][id]["dealer"] = p.game.dealer.getHandJson(!has_ended);
            // res["players"][idx]["move_made"] = p.game.player.getMoveMade();
            // res["players"][idx]["waiting"] = p.game.player.getIsWaiting();
            // idx++;
        }
        return res;
    }

    // void reset_round_if_all_moved() {
    //     this->lock();
    //     if (players.empty()) {
    //         return;
    //     }

    //     bool all_moved = true;
    //     for (const auto& [_, p] : players) {
    //         if (!p.getMoveMade() && !p.getIsWaiting()) {
    //             all_moved = false;
    //         }
    //     }

    //     if (all_moved) {
    //         for (auto& [_, p] : players) {
    //             p.setMoveMade(false);
    //             if (p.getIsWaiting()) {
    //                 p.setIsWaiting(false);
    //             }
    //         }
    //     }
    // }

    // void player_move(const std::string& pid) {
    //     this->lock();
    //     auto it = players.find(pid);
    //     if (it != players.end()) {
    //         it->second.setMoveMade(true);
    //     }
    //     reset_round_if_all_moved();
    // }
};
