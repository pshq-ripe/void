pub mod commands;
pub mod irc;
pub mod scripting;
pub mod ui;
pub mod logging;
pub mod flood;
pub mod dcc;
pub mod storage;
pub mod motd;
pub mod format;
pub mod charset;
pub mod bouncer;

pub mod app;
pub use app::App;

#[cfg(test)]
mod app_test;
#[cfg(test)]
mod settings_test;
#[cfg(test)]
mod theme_test;
