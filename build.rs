fn main() {
    if cfg!(all(feature = "builtin-lua", feature = "system-lua")) {
        panic!("cannot enable both builtin-lua and system-lua features when building rlua");
    }

    #[cfg(feature = "builtin-lua")]
    {
        use std::env;

        let target_os = env::var("CARGO_CFG_TARGET_OS");
        let target_family = env::var("CARGO_CFG_TARGET_FAMILY");

        let mut config = cc::Build::new();

        if target_os == Ok("linux".to_string()) {
            config.define("LUA_USE_LINUX", None);
        } else if target_os == Ok("macos".to_string()) {
            config.define("LUA_USE_MACOSX", None);
        } else if target_family == Ok("unix".to_string()) {
            config.define("LUA_USE_POSIX", None);
        } else if target_family == Ok("windows".to_string()) {
            config.define("LUA_USE_WINDOWS", None);
        }

        if cfg!(debug_assertions) {
            config.define("LUA_USE_APICHECK", None);
        }

        config
            .cpp(true)
            .include("lua")
            .file("lua/src/lapi.c")
            .file("lua/src/lauxlib.c")
            .file("lua/src/lbaselib.c")
            .file("lua/src/lcode.c")
            .file("lua/src/lcorolib.c")
            .file("lua/src/lctype.c")
            .file("lua/src/ldblib.c")
            .file("lua/src/ldebug.c")
            .file("lua/src/ldo.c")
            .file("lua/src/ldump.c")
            .file("lua/src/lfunc.c")
            .file("lua/src/lgc.c")
            .file("lua/src/linit.c")
            .file("lua/src/liolib.c")
            .file("lua/src/llex.c")
            .file("lua/src/lmathlib.c")
            .file("lua/src/lmem.c")
            .file("lua/src/loadlib.c")
            .file("lua/src/lobject.c")
            .file("lua/src/lopcodes.c")
            .file("lua/src/loslib.c")
            .file("lua/src/lparser.c")
            .file("lua/src/lstate.c")
            .file("lua/src/lstring.c")
            .file("lua/src/lstrlib.c")
            .file("lua/src/ltable.c")
            .file("lua/src/ltablib.c")
            .file("lua/src/ltm.c")
            .file("lua/src/lundump.c")
            //.file("lua/src/lutf8lib.c")
            .file("lua/src/lvm.c")
            .file("lua/src/lzio.c")
            .compile("liblua5.2.a");
    }

    #[cfg(feature = "system-lua")]
    {
        pkg_config::Config::new()
            .atleast_version("5.4")
            .probe("lua")
            .unwrap();
    }
}
