# ♠️ Blackjack

> [!NOTE]
> forked from [ineshbose/Blackjack_CPP](https://github.com/ineshbose/Blackjack_CPP)

> [!NOTE]
> WIP, but we won't really continue developing it.  
> the idea was to have multiplayer mode via webserver, which is pretty much done  
> but the client has some glitches from time to time  
> and the server as well (though should be fine). also, it has a separate dealer for each player  

## Installing

we provide pre-compiled binaries for linux, mac and windows, check out [the latest release](https://github.com/jarjk/blackjackpp/releases/latest)

## 🔧 Development Setup

### Cloning

```sh
git clone https://github.com/jarjk/blackjackpp
cd blackjackpp
```

### Building / Compiling

#### Client, Meson

```sh
meson setup build-dir
meson compile -C build-dir
# if not localhost: BJ_ADDR=$server_address
./build/blackjackpp-client
```

#### Server, Cargo

```sh
cargo run # --release
```
