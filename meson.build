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
gh_deps = {
  'cpptui': ['csboo/cpptui', 'cfa92dfdf69c6218f9cf95dac70cf8201a350184', '.'],
  'nlohmann_json': ['nlohmann/json', 'b451735fe7bb3283336f934b3f6cd3f484f73649', 'single_include'],
  'cpp-httplib': ['yhirose/cpp-httplib', '3a1f379e751ef6555f06c5c1ef367a6fce26722c', '.']
}

fs = import('fs')
git = find_program('git', required: true)
external_dir = '.' / 'external'
system_deps = get_option('system-deps')

dep_map = {}

foreach name, details : gh_deps
  repo = details[0]
  commit = details[1]
  inc_subdir = details[2]

  sys_dep = dependency(name, required: false)
  if system_deps and sys_dep.found()
    message(name + ' found on system.')
    dep_map += {name: sys_dep}
  else
    dep_dir = external_dir / name

    if not fs.exists(dep_dir)
      message(name + ' not found, cloning from https://github.com/' + repo + ' @' + commit)
      res = run_command(git, 'clone', '--revision=' + commit, 'https://github.com/' + repo, dep_dir, check: true)
    else
      message(name + ' already cloned at: ' + dep_dir + ', updating')
      res = run_command(git, '-C', dep_dir, 'checkout', commit, check: true)
    endif
    message(res.stdout(), res.stderr())
    inc = include_directories(dep_dir / inc_subdir)
    dep_map += {name: declare_dependency(include_directories: inc)}
  endif
endforeach

lib_includes = include_directories('src/lib')
libblackjackpp = declare_dependency(include_directories: lib_includes, dependencies: [dep_map['nlohmann_json'], dep_map['cpptui']])

build_client = get_option('client')
build_server = get_option('server')

deps = [libblackjackpp, dep_map['cpp-httplib']]
if host_machine.system() == 'windows'
  cc = meson.get_compiler('cpp')
  ws2_32 = cc.find_library('ws2_32', required: true)
  deps += ws2_32
endif
cc_args = ['-D_WIN32_WINNT=0x0A00']

# these guys need the dependencies ready
if build_client
  message('building the client')
  executable('blackjackpp-client', ['src/client/main.cpp'], install: true, dependencies: deps, cpp_args: cc_args)
endif

if build_server
  message('building the server')
  executable('blackjackpp-server', ['src/server/main.cpp'], install: true, dependencies: deps, cpp_args: cc_args)
endif
