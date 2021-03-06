extern crate bufstream;
extern crate cargo;
extern crate filetime;
extern crate flate2;
extern crate git2;
extern crate hamcrest;
extern crate libc;
extern crate rustc_serialize;
extern crate tar;
extern crate tempdir;
extern crate term;
extern crate url;
#[cfg(windows)] extern crate kernel32;
#[cfg(windows)] extern crate winapi;

#[macro_use]
extern crate log;

use cargo::util::Rustc;

mod support;
macro_rules! test {
    ($name:ident $expr:expr) => (
        #[test]
        fn $name() {
            ::support::paths::setup();
            setup();
            $expr;
        }
    )
}

mod test_bad_config;
mod test_cargo;
mod test_cargo_bench;
mod test_cargo_build_auth;
mod test_cargo_build_lib;
mod test_cargo_clean;
mod test_cargo_compile;
mod test_cargo_compile_custom_build;
mod test_cargo_compile_git_deps;
mod test_cargo_compile_path_deps;
mod test_cargo_compile_plugins;
mod test_cargo_cross_compile;
mod test_cargo_doc;
mod test_cargo_features;
mod test_cargo_fetch;
mod test_cargo_freshness;
mod test_cargo_generate_lockfile;
mod test_cargo_new;
mod test_cargo_package;
mod test_cargo_profiles;
mod test_cargo_publish;
mod test_cargo_registry;
mod test_cargo_run;
mod test_cargo_rustc;
mod test_cargo_search;
mod test_cargo_test;
mod test_cargo_tool_paths;
mod test_cargo_version;
mod test_shell;

thread_local!(static RUSTC: Rustc = Rustc::new("rustc").unwrap());

fn rustc_host() -> String {
    RUSTC.with(|r| r.host.clone())
}

fn is_nightly() -> bool {
    RUSTC.with(|r| {
        r.verbose_version.contains("-nightly") ||
            r.verbose_version.contains("-dev")
    })
}

fn can_panic() -> bool {
    RUSTC.with(|r| !r.host.contains("msvc"))
}
