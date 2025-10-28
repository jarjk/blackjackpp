#include <exception>
#include <stdexcept>
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#endif
#include <httplib.h>

#include <cstdlib>
#include <fstream>
#include <input.hpp>
#include <iostream>
#include <tui.hpp>

#include "client_game.hpp"
#include "print.hpp"
#include "utils.hpp"

void help();
void run(ClientGame& cg, httplib::Client& cli);
static inline void connect(httplib::Client& cli, ClientGame& cg);

int main() {
    const char* server_addr = std::getenv("BJ_ADDR");
    if (server_addr == nullptr) {
        std::cerr << "env var 'BJ_ADDR' is not set to the server's ip address, defaulting to 'localhost'";
        server_addr = "localhost";
    }

    httplib::Client cli(server_addr, 18080);

    std::ofstream logf(".blackjacpp-client.log", std::ios::app);
    cli.set_logger([&logf](const httplib::Request& req, const httplib::Response& res) {
        logf << utils::now_s() << " ✓ " << req.method << " " << req.path << " -> " << res.status << " ("
             << res.body.size() << " bytes" << ")" << '\n';
    });

    cli.set_error_logger([&logf](const httplib::Error& err, const httplib::Request* req) {
        logf << utils::now_s() << " ✗ " << req->method << " " << req->path
             << " failed: " << httplib::to_string(err) << '\n';
    });
    cli.set_keep_alive(true);

    ClientGame cg;

    tui::init();

    try {
        run(cg, cli);
    } catch (const std::exception& e) {
        tui::reset();
        std::cerr << "\nsorry, the app ran into a fatal, unexpected error.\nall we know about it is this: '"
                  << e.what() << "'\n";
        cli.stop();
        return 1;
    }

    tui::reset();
}

void run(ClientGame& cg, httplib::Client& cli) {
    tui::screen::clear();
    tui::cursor::home();

    char c = 0;
    do {
        tui::screen::clear();
        tui::cursor::home();
        std::cout << tui::string(utils::raw_mode_converter(Print::title_blackjack())).yellow();
        std::cout << utils::raw_mode_converter(Print::begin_menu());
        // if (!message.empty()) {
        //     std::cout << tui::string(message).red() << "\n";
        // }

        c = Input::read_ch();
        switch (c) {
            case '1':
                connect(cli, cg);
                do {
                    {
                        // not joining actually, just rejoining, so playing again
                        auto res = cli.Get(tui::concat("/join?username=", cg.game.player.getName()));
                        if (!res) {
                            throw std::runtime_error("got no response from server");
                        }
                        if (res.value().status == 200) {
                            cg.game.player.clearCards();
                        }
                    }
                    tui::cursor::set_position(tui::screen::size().first / 2,
                                              (tui::screen::size().second / 2) - 9);
                    tui::screen::clear_line_right();
                    cg.kinda_beginGame(cli);
                    tui::cursor::set_position(tui::screen::size().first / 2,
                                              (tui::screen::size().second / 2) - 9);
                    std::cerr << "continue? (Y/n)";
                    c = Input::read_ch();
                } while (std::tolower(c) != 'n' && c != SpecKey::CtrlC);
                break;
            case '2':
                help();
                Input::read_ch();
                break;
            case 'q':
                break;
            default:
                continue;
        }
    } while (c != 'q');
}

void help() {
    tui::screen::clear();
    tui::cursor::home();
    std::cout << tui::string(utils::raw_mode_converter(Print::instructions())).green();
}

void connect(httplib::Client& cli, ClientGame& cg) {
    tui::screen::clear();
    tui::cursor::home();

    std::string username;

    while (true) {
        tui::cursor::set_position(tui::screen::size().first / 2, (tui::screen::size().second / 2) - 21);
        tui::screen::clear_line_right();
        std::cout << "Enter your username: ";

        tui::disable_raw_mode();
        tui::cursor::visible(true);
        std::cin >> username;
        std::cout << "\n";
        tui::cursor::visible(false);
        tui::enable_raw_mode();

        auto res = cli.Get(tui::concat("/join?username=", username));

        if (!res) {
            throw std::runtime_error("got no response from server");
        }

        if (res.value().status == 200) {
            cg.game.player.setName(username);
            cg.game.player.clearCards();
            break;
        }
    }
}
