#pragma once
#include <random>

namespace rng {
    std::random_device rd;
    std::mt19937 custom_random(rd());
}  // namespace rng
