#pragma once

#include "game.hpp"
#include "httplib.h"

struct ClientGame {
    Game game;
    Player player;
    bool startBet();
    void kinda_beginGame(httplib::Client& cli);
};
