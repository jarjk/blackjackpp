#pragma once

#include "game.hpp"
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
#endif
#include <httplib.h>

struct ClientGame {
    Game game;
    Player player;
    bool startBet();
    void kinda_beginGame(httplib::Client& cli);
};
