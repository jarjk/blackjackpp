project('blackjackpp-client', 'cpp',
  version : '0.1',
  default_options : [
    'cpp_std=c++20',
    'warning_level=3',
    # 'werror=true',
    'optimization=g',
    'debug=true',
    'b_ndebug=if-release'
  ]
)

fs = import('fs')

# include_dir = include_directories('include')
httplib_dir = meson.project_source_root() / 'external' / 'HttpLib'
cpptui_dir = meson.project_source_root() / 'external' / 'cpptui'
json_dir = meson.project_source_root() / 'external' / 'json'

git = find_program('git', required: true)

if not fs.exists(cpptui_dir)
  message('Cloning cpptui at master...')
  run_command(git, 'clone', 'https://github.com/csboo/cpptui', cpptui_dir, check: true)
else
  message('Cpptui exists!')
endif

message('cpptui cloned to: ' + cpptui_dir)

if not fs.exists(httplib_dir)
  message('Cloning HttpLib at master...')
  run_command(git, 'clone', 'https://github.com/yhirose/cpp-httplib', '--depth=1', httplib_dir, check: true)
else
  message('HttpLib exists!')
endif

message('HttpLib cloned to: ' + httplib_dir)

if not fs.exists(json_dir)
  message('Cloning json at master...')
  run_command(git, 'clone', 'https://github.com/nlohmann/json', '--depth=1', json_dir, check: true)
else
  message('json exists!')
endif

message('json cloned to: ' + json_dir)

main_target_name = 'blackjackpp-client'
sources = ['src/main.cpp', '../card.cpp', '../dealer.cpp', '../deck.cpp', '../game.cpp', '../human.cpp', '../player.cpp', '../print.cpp', '../statistics.cpp', './src/client_game.cpp']
hdrs = include_directories('./headers', '../headers', './external/cpptui', './external/HttpLib', 'external/json/single_include')

executable(main_target_name, sources, include_directories: hdrs, install: true)
