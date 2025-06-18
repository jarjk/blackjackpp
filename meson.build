project('blackjackpp', 'cpp',
  version: '0.1',
  meson_version: '>=1.1.0',
  default_options: [
    'cpp_std=c++20',
    'warning_level=3',
    # 'werror=true',
    'optimization=g',
    'debug=true',
    'b_ndebug=if-release'
  ]
)

# dep: [name, github_repo, include_dir]
gh_deps = [['cpptui', 'csboo/cpptui', '.'], ['nlohmann_json', 'nlohmann/json', 'single_include']]

build_client = get_option('client')
build_server = get_option('server')

if build_client
  gh_deps += [['cpp-httplib', 'yhirose/cpp-httplib', '.']]
  message('building the client')
endif

if build_server
  gh_deps += [['crow', 'crowcpp/crow', 'include']]
  message('building the server')
endif

fs = import('fs')
git = find_program('git', required: true)
external_dir = '.' / 'external'

dep_map = {}

foreach dep : gh_deps
  name = dep[0]
  repo = dep[1]
  inc_subdir = dep[2]

  sys_dep = dependency(name, required: false)
  if sys_dep.found()
    message(name + ' found on system.')
    dep_map += {name: sys_dep}
  else
    dep_dir = external_dir / name
    if not fs.exists(dep_dir)
      message(name + ' not found, cloning from https://github.com/' + repo)
      run_command(git, 'clone', '--depth=1', 'https://github.com/' + repo, dep_dir, check: true)
    else
      message(name + ' already cloned at: ' + dep_dir)
    endif
    inc = include_directories(dep_dir / inc_subdir)
    dep_map += {name: declare_dependency(include_directories: inc)}
  endif
endforeach


lib_includes = include_directories('src/lib')
libblackjackpp = declare_dependency(include_directories: lib_includes, dependencies: [dep_map['nlohmann_json'], dep_map['cpptui']])

# these guys need the dependencies ready
if build_client
  ws2_32 = []
  if host_machine.system() == 'windows'
    cc = meson.get_compiler('cpp')
    ws2_32 = cc.find_library('ws2_32', required: true)
  endif
  executable('blackjackpp-client', ['src/client/main.cpp'], dependencies: [libblackjackpp, ws2_32, dep_map['cpp-httplib']], install: true)
endif
if build_server
  asio_dep = dependency('asio', required: true)

  sources = ['src/server/main.cpp']
  executable('blackjackpp-server', sources,
    dependencies : [asio_dep, libblackjackpp, dep_map['crow']],
    install : true
  )
endif
