project('blackjac', 'cpp',
  version : '0.1',
  default_options : [
    'cpp_std=c++17',
    'warning_level=3',
    # 'werror=true',
    'optimization=g',
    'debug=true',
    'b_ndebug=if-release'
  ]
)

fs = import('fs')

# include_dir = include_directories('include')
cpptui_dir = meson.project_source_root() / 'external' / 'cpptui'

git = find_program('git', required: true)

if not fs.exists(cpptui_dir)
  message('Cloning cpptui at master...')
  run_command(git, 'clone', 'https://github.com/csboo/cpptui', cpptui_dir, check: true)
else
  message('Cpptui exists!')
endif

message('cpptui cloned to: ' + cpptui_dir)

main_target_name = 'blackjack'
sources = ['src/blackjack.cpp', 'src/card.cpp', 'src/dealer.cpp', 'src/deck.cpp', 'src/game.cpp', 'src/human.cpp', 'src/player.cpp', 'src/print.cpp', 'src/statistics.cpp']
hdrs = include_directories('src/headers', 'external/cpptui')

executable(main_target_name, sources, include_directories: hdrs, install: true)
