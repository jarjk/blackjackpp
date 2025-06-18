# ♠️ Blackjack (C++)

> [!NOTE]
> forked from [ineshbose/Blackjack_CPP](https://github.com/ineshbose/Blackjack_CPP)

> [!NOTE] 
> WIP, but we won't continue developing it  
> idea is to have multiplayer mode via webserver, which is pretty much done  
> but the client has some glitches from time to time  
> and the server as well (though mostly behaves), has a separate dealer for each player  

## 🔧 Setup

### Cloning
```sh
git clone https://github.com/jarjk/blackjackpp
cd blackjackpp
```

### Building / Compiling

> [!NOTE]
> if you're using Windows, consider not doing it and choose an OS (if possible), that's not completely dumb and useless  
> although it does compile, be careful!

#### Meson
```sh
# by default only client is built, see meson.options, *bool* is default
meson setup build # can be set with -Dclient=[*true*, false] -Dserver=[true, *false*]
meson compile -C build
# if not localhost: BLACKJACKPP_SERVER_ADDRESS=$desired_address
./build/blackjackpp-client
# if built. needs `asio` dependency from `boost`
./build/blackjackpp-server
```
