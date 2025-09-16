#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
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

void help(httplib::Client& cli);
void run(ClientGame& cg, httplib::Client& cli);
static inline void connect(httplib::Client& cli, ClientGame& cg);

int main() {
    const char* server_addr = std::getenv("BLACKJACKPP_SERVER_ADDRESS");
    if (server_addr == nullptr) {
        std::cerr << "env var 'BLACKJACKPP_SERVER_ADDRESS' is not set to the server's ip address, "
                     "defaulting to 'localhost'";
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

    run(cg, cli);

    tui::reset();
    logf.close();
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
                    tui::cursor::set_position(tui::screen::size().first / 2,
                                              (tui::screen::size().second / 2) - 9);
                    tui::screen::clear_line_right();
                    cg.kinda_beginGame(cli);
                    tui::cursor::set_position(tui::screen::size().first / 2,
                                              (tui::screen::size().second / 2) - 9);
                    std::cerr << "continue? (Y/n)";
                    c = Input::read_ch();
                } while (c == 'y' || c == 'Y' || c == SpecKey::Enter);
                break;
            case '2':
                help(cli);
                Input::read_ch();
                break;
            case 'q':
                break;
            default:
                continue;
        }
    } while (c != 'q');
}

void help(httplib::Client& cli) {
    auto res = unwrap_or(cli.Get("/help"));

    tui::screen::clear();
    tui::cursor::home();
    std::cout << tui::string(utils::raw_mode_converter(res.body)).green();
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

        auto res = cli.Get(std::format("/join?username={}", username));

        if (!res) {
            tui::reset();
            std::cout << "got no response from server\n";
            exit(0);
        }

        if (res.value().status == 200) {
            cg.game.player.setName(username);
            cg.game.player.clearCards();
            break;
        }
    }
}
