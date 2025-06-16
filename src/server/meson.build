project('blackjackpp-server', 'cpp',
  version : '0.1.0',
  default_options : [
    'cpp_std=c++20',
    'warning_level=2',
    'optimization=g',
    'debug=true',
    'b_ndebug=if-release'
  ]
)

# crow_dep = dependency('crow', required: true)
fs = import('fs')
git = find_program('git', required: true)
crow_dir = meson.project_source_root() / 'external' / 'Crow'
if not fs.exists(crow_dir)
  message('Cloning crow at master...')
  run_command(git, 'clone', 'https://github.com/CrowCpp/Crow', '--depth=1', crow_dir, check: true)
else
  message('crow exists!')
endif

asio_dep = dependency('asio', required: true)

sources = ['src/main.cpp', '../card.cpp', '../dealer.cpp', '../deck.cpp', '../game.cpp', '../human.cpp', '../player.cpp', '../print.cpp', '../statistics.cpp']
hdrs = include_directories('./headers', '../headers', '../../external/cpptui', '../client/external/json/single_include', './external/Crow/include', '../client/external/HttpLib')

proj_name = 'blackjackpp-server'
executable(proj_name, sources,
  dependencies : [asio_dep],
  include_directories: hdrs,
  install : true
)
