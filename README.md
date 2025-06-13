# ♠️ Blackjack (C++)

> [!NOTE]
> forked from [ineshbose/Blackjack_CPP](https://github.com/ineshbose/Blackjack_CPP)

> [!NOTE]
> WIP, idea is to have multiplayer mode via webserver

## 🔧 Setup

### Cloning
```sh
git clone https://github.com/jeromeschmied/blackjackpp
cd blackjackpp
```

### Building / Compiling

#### Meson
```sh
mkdir build; cd build # have a build dir
meson setup ..    # generates build configuration (ninja)
ninja             # builds the executable
./blackjack       # voila!
```

do the same in `src/client` or `src/server`
