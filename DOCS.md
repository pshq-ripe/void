# Void IRC Client — Complete Documentation

> A modern, Lua-scriptable IRC client written in Rust, inspired by **epic5** with **LiCe5** scripts and **epic6** features.

**Version:** 0.3.0  
**License:** MIT  
**Repository:** https://github.com/pshq/void

---

## Table of Contents

1. [Installation](#1-installation)
2. [Quick Start](#2-quick-start)
3. [CLI Reference](#3-cli-reference)
4. [Command Reference](#4-command-reference)
5. [Lua Scripting Guide](#5-lua-scripting-guide)
6. [Modules Guide](#6-modules-guide)
7. [Configuration](#7-configuration)
8. [IRCv3 Features](#8-ircv3-features)
9. [Keyboard Shortcuts](#9-keyboard-shortcuts)
10. [Architecture](#10-architecture)
11. [Troubleshooting](#11-troubleshooting)
12. [Credits](#12-credits)

---

## 1. Installation

### 1.1 Prerequisites

Void is built with Rust and requires the following system dependencies:

| Dependency | Purpose | Required |
|---|---|---|
| **Rust toolchain** (1.75+) | Build from source | Yes |
| **OpenSSL dev headers** | TLS connections, SQLCipher | Yes |
| **C compiler** (gcc/clang) | Native crate compilation | Yes |
| **pkg-config** | Library discovery | Recommended |
| **Lua 5.4** | Scripting engine (vendored by default) | Bundled |

#### Installing Rust

```bash
# Install Rust via rustup (official installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

#### Installing OpenSSL dev headers

**Debian / Ubuntu:**
```bash
sudo apt update
sudo apt install -y libssl-dev pkg-config build-essential
```

**Fedora / RHEL:**
```bash
sudo dnf install -y openssl-devel pkg-config gcc
```

**Arch Linux:**
```bash
sudo pacman -S openssl pkgconf base-devel
```

**macOS (Homebrew):**
```bash
brew install openssl pkg-config
export PKG_CONFIG_PATH="$(brew --prefix openssl)/lib/pkgconfig"
```

**Windows:**
Install [vcpkg](https://vcpkg.io) or use the `openssl` crate's vendored feature. Alternatively, use the pre-built Windows binary if available.

### 1.2 Build from Source

```bash
# Clone the repository
git clone https://github.com/pshq/void.git
cd void

# Development build (faster compilation, slower runtime)
cargo build

# Release build (optimized, recommended for daily use)
cargo build --release

# The binary will be at:
#   target/release/void    (release)
#   target/debug/void      (development)
```

### 1.3 Install via Cargo

```bash
# Install directly from the local project
cargo install --path .

# This places the `void` binary in ~/.cargo/bin/
# Make sure ~/.cargo/bin is in your PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### 1.3.1 System-Wide Install

To install Void system-wide so all users can access it:

```bash
# Build the release binary
cargo build --release

# Copy to a system-wide location
sudo cp target/release/void /usr/local/bin/
sudo chmod +x /usr/local/bin/void

# Verify installation
void --version
```

**Binary location after `cargo install`:** `~/.cargo/bin/void`

To find where cargo installs binaries:
```bash
cargo install --list | grep void
which void
```

### 1.4 Running Tests

```bash
# Run all tests
cargo test

# Run Lua integration tests with output
cargo test --test lua_integration -- --nocapture
```

### 1.5 Directory Structure

After first run, Void creates the following directory structure:

```
~/.void/
├── void.db          # SQLCipher encrypted SQLite database
├── void.conf        # Text backup of settings (INI-style)
└── logs/            # Log files (if logging enabled)
    ├── #channel-2026-01-15.log
    └── (Status)-2026-01-15.log
```

**Project source layout:**

```
void/
├── Cargo.toml              # Rust project manifest
├── config.lua              # Example Lua configuration
├── modules/                # LiCe5 compatibility modules
│   ├── init.lua            # Module loader (entry point)
│   ├── ignore.lua          # Enhanced ignore system
│   ├── gone.lua            # Away/back with random reasons
│   ├── kick.lua            # Enhanced kick/kickban
│   ├── mass.lua            # Mass mode commands
│   ├── userlist.lua        # Bot-style auto-op/voice
│   ├── alarm.lua           # Timer/reminder system
│   ├── reconnect.lua       # Auto-reconnect with rejoin
│   ├── paste.lua           # Multi-line paste mode
│   ├── logman.lua          # Per-channel log management
│   ├── autovoice.lua       # Auto-voice on join
│   ├── anti_flood.lua      # Anti-flood protection
│   ├── highlight.lua       # Nick/pattern highlight
│   ├── ctcp.lua            # Enhanced CTCP replies
│   ├── nickserv.lua        # NickServ auto-identify
│   ├── channel_protect.lua # Anti-kick, anti-deop
│   ├── invite.lua          # Invite tracking
│   ├── dns.lua             # DNS lookup
│   ├── signoff.lua         # Random quit messages
│   ├── wall.lua            # Broadcast to channels
│   ├── finger.lua          # User info lookup
│   ├── memo.lua            # Offline memo system
│   ├── note.lua            # Quick notes
│   ├── party.lua           # Party mode with disco colors
│   ├── sensors.lua         # Channel activity monitoring
│   ├── help.lua            # Enhanced help system
│   ├── *.reasons           # Random reason files
│   └── logos/              # ASCII art logos
├── scripts/                # User custom Lua scripts
└── src/
    ├── main.rs             # Entry point, CLI args, main loop
    ├── lib.rs              # Module declarations
    ├── app.rs              # Core application state
    ├── commands/
    │   └── registry.rs     # All built-in commands
    ├── irc/
    │   ├── connection.rs   # IRC connection, SASL, proxy
    │   └── proto.rs        # IRC protocol message handling
    ├── scripting/
    │   ├── api.rs          # Lua API (void.* functions)
    │   └── engine.rs       # Lua engine initialization
    ├── ui/
    │   ├── input.rs        # Keyboard input handling
    │   ├── renderer.rs     # TUI rendering (ratatui)
    │   ├── statusbar.rs    # Status bar rendering
    │   ├── scrollback.rs   # Scrollback buffer
    │   └── handler.rs      # UI event handler
    ├── storage.rs          # SQLCipher database layer
    ├── logging.rs          # Log file management
    ├── flood.rs            # Flood protection
    ├── dcc.rs              # DCC file transfer
    └── motd.rs             # ASCII art MOTD
```

---

## 2. Quick Start

### 2.1 First Connection

The simplest way to connect to an IRC server:

```bash
# Connect to Libera.Chat with TLS (default port 6697)
./target/release/void -c irc.libera.chat -n mynick

# Connect and auto-join a channel
./target/release/void -c irc.libera.chat -n mynick -j "#rust"

# Connect to a non-TLS server on port 6667
./target/release/void -c irc.example.com -n mynick -P 6667 --no-tls
```

### 2.2 Basic Commands

Once connected, you can type commands in the input line at the bottom of the screen:

| Action | Command | Example |
|---|---|---|
| Join a channel | `/join` | `/join #void` |
| Leave a channel | `/part` | `/part Goodbye!` |
| Send a message | Just type | `Hello everyone!` |
| Send a private message | `/msg` | `/msg friendname Hey there` |
| Change nickname | `/nick` | `/nick newname` |
| Set yourself away | `/away` | `/away Gone for lunch` |
| Return from away | `/away` | `/away` (with no message) |
| Get user info | `/whois` | `/whois someuser` |
| Quit the client | `/quit` | `/quit See you later!` |

### 2.3 Joining Channels

```
/join #programming
/join #rust-lang secretkey     -- join a channel with a key
/join &localchannel            -- join a local channel
```

### 2.4 Sending Messages

```
Hello world!                   -- sends to current channel
/msg #channel some message     -- send to a specific channel
/msg nickname private message  -- send a private message
/me dances                     -- send an action (/me)
/notice nickname important     -- send a notice
```

### 2.5 Using config.lua

Create a `config.lua` in the working directory to auto-configure Void:

```lua
math.randomseed(os.time())

config = {
    nickname = "my_nick",
    server = "irc.libera.chat",
    channels = {"#mychannel", "#another"}
}

-- Load LiCe5 modules
dofile("modules/init.lua")

-- Custom quit reasons
local quit_reasons = {
    "Leaving",
    "Ping timeout",
    "BRB",
}

function get_quit_message()
    return quit_reasons[math.random(1, #quit_reasons)]
end
```

---

## 3. CLI Reference

### 3.1 Connection Options

| Flag | Long Form | Argument | Default | Description |
|---|---|---|---|---|
| `-c` | `--server` | `<hostname>` | `irc.libera.chat` | IRC server hostname to connect to |
| `-n` | `--nickname` | `<nick>` | `void_user` | Your nickname on the IRC network |
| `-j` | `--channel` | `<#channel>` | — | Channel to auto-join after connecting |
| `-p` | `--password` | `<password>` | — | Server password (PASS command) |
| `-P` | `--port` | `<port>` | `6697` | Server port (6697 = TLS, 6667 = plain) |
| `-h` | `--vhost` | `<hostname>` | — | Bind to a specific virtual host |
| `-N` | `--nickserv` | `<password>` | — | NickServ password for auto-identify |

### 3.2 TLS Options

| Flag | Description |
|---|---|
| `--no-tls` | Disable TLS encryption (connect via plain text) |

### 3.3 SASL Authentication

| Flag | Argument | Description |
|---|---|---|
| `--sasl` | `<nick:password>` | SASL PLAIN authentication |
| `--sasl` | `EXTERNAL` | SASL EXTERNAL (client certificate) |
| `--sasl` | `SCRAM-SHA-512:<nick:password>` | SASL SCRAM-SHA-512 authentication |

**Examples:**

```bash
# SASL PLAIN
void -c irc.libera.chat -n mynick --sasl "mynick:secretpass"

# SASL EXTERNAL (requires client certificate configured on server)
void -c irc.libera.chat -n mynick --sasl EXTERNAL

# SASL SCRAM-SHA-512 (strongest mechanism)
void -c irc.libera.chat -n mynick --sasl "SCRAM-SHA-512:mynick:secretpass"
```

### 3.4 Proxy Options

| Flag | Argument | Default | Description |
|---|---|---|---|
| `--proxy-type` | `socks5` | — | Proxy type (currently only SOCKS5) |
| `--proxy-server` | `<hostname>` | — | Proxy server hostname |
| `--proxy-port` | `<port>` | `1080` | Proxy server port |
| `--proxy-user` | `<username>` | — | Proxy authentication username |
| `--proxy-pass` | `<password>` | — | Proxy authentication password |

**Example:**

```bash
# Connect through a SOCKS5 proxy
void -c irc.libera.chat -n mynick \
    --proxy-type socks5 \
    --proxy-server 127.0.0.1 \
    --proxy-port 9050
```

### 3.5 Miscellaneous Options

| Flag | Argument | Description |
|---|---|---|
| `--ipv6` | — | Force IPv6 connection |
| `--db-pass` | `<passphrase>` | Custom database encryption passphrase |

### 3.6 Full Examples

```bash
# Basic connection to Libera.Chat
void -c irc.libera.chat -n myuser -j "#rust"

# Connection with NickServ auto-identify
void -c irc.libera.chat -n myuser --nickserv "mypassword" -j "#general"

# Multi-server: connect to second server
# (use /server -m from within the client)

# Through Tor SOCKS5 proxy
void -c irc.libera.chat -n anon_user \
    --proxy-type socks5 \
    --proxy-server 127.0.0.1 \
    --proxy-port 9050 \
    --no-tls

# With SASL and custom DB passphrase
void -c irc.libera.chat -n myuser \
    --sasl "myuser:mysecretpass" \
    --db-pass "my-database-key"
```

---

## 4. Command Reference

All commands are case-insensitive. Commands prefixed with `/` are entered in the input line. Aliases are shown in parentheses.

### 4.1 Server Commands

| Command | Aliases | Syntax | Description |
|---|---|---|---|
| `/server` | `/connect` | `/server <host> [port] [pass]` | Connect to an IRC server |
| `/server` | | `/server` | List all server connections |
| `/server` | | `/server -m <host> [port]` | Add a new server connection (multi-server) |
| `/server` | | `/server <index>` | Switch to server by index |
| `/disconnect` | `/discon` | `/disconnect` | Disconnect from current server |
| `/reconnect` | | `/reconnect` | Reconnect to the last server |
| `/quit` | `/exit`, `/bye` | `/quit [reason]` | Quit the client with optional reason |
| `/nick` | | `/nick <newnick>` | Change your nickname |
| `/away` | | `/away [message]` | Set yourself as away (or return if no message) |
| `/whois` | `/wi` | `/whois <nick>` | Query detailed user information |
| `/whowas` | | `/whowas <nick>` | Query past user information |
| `/who` | | `/who <mask>` | List users matching a mask |
| `/userhost` | | `/userhost <nick>` | Query a user's host |
| `/oper` | | `/oper <login> <password>` | Authenticate as IRC operator |
| `/kill` | | `/kill <nick> [reason]` | Force disconnect a user (requires IRC operator) |

**Examples:**

```
/server irc.libera.chat 6697
/server -m irc.oftc.net 6697    -- add second server
/server 1                        -- switch to server index 1
/nick NewNick
/away Gone for lunch, back in 1h
/whois someuser
```

### 4.2 Channel Commands

| Command | Aliases | Syntax | Description |
|---|---|---|---|
| `/join` | `/j` | `/join <#channel> [key]` | Join a channel (with optional key) |
| `/part` | `/leave` | `/part [#channel] [reason]` | Leave a channel |
| `/topic` | `/t` | `/topic [text]` | View or set the channel topic |
| `/names` | | `/names [#channel]` | List users in a channel |
| `/kick` | `/k` | `/kick <nick> [reason]` | Kick a user from the channel |
| `/mode` | | `/mode <target> <modes> [args]` | Set channel or user modes |
| `/invite` | | `/invite <nick> <#channel>` | Invite a user to a channel |
| `/ban` | `/b` | `/ban <nick\|mask>` | Ban a user from the channel |
| `/unban` | `/ub` | `/unban <nick\|mask>` | Remove a ban |
| `/kickban` | `/kb` | `/kickban <nick> [reason]` | Ban and kick a user |
| `/op` | | `/op <nick>` | Give operator status (+o) |
| `/deop` | | `/deop <nick>` | Remove operator status (-o) |
| `/voice` | `/v` | `/voice <nick>` | Give voice status (+v) |
| `/devoice` | `/dv` | `/devoice <nick>` | Remove voice status (-v) |
| `/list` | | `/list [mask]` | List channels on the server |
| `/cycle` | | `/cycle` | Part and rejoin the current channel |
| `/knock` | | `/knock <#channel> [msg]` | Request access to an invite-only channel |
| `/except` | `/ex` | `/except [mask]` | Manage ban exceptions (+e) |
| `/invex` | `/in` | `/invex [mask]` | Manage invite exceptions (+I) |
| `/reop` | `/re` | `/reop [mask]` | Manage reop hints (+R, ircd 2.11) |

**Examples:**

```
/join #programming
/join #private secretkey
/part #oldchannel Goodbye everyone!
/topic Welcome to #programming! Be nice.
/kick spamer Please stop spamming
/kickban troll Get out
/op trusteduser
/voice newuser
/mode #channel +nt
/ban *!*@spammer.com
```

### 4.3 Message Commands

| Command | Aliases | Syntax | Description |
|---|---|---|---|
| `/msg` | `/m` | `/msg <target> <text>` | Send a private message |
| `/me` | `/describe` | `/me <action>` | Send an action message |
| `/notice` | | `/notice <target> <text>` | Send a notice |
| `/say` | | `/say <text>` | Send text to the current channel |
| `/query` | `/q` | `/query <nick>` | Open a query (private message) window |
| `/ctcp` | | `/ctcp <nick> <type> [args]` | Send a CTCP request |
| `/ping` | | `/ping <nick>` | Send a CTCP PING to measure latency |
| `/wallops` | | `/wallops <text>` | Send a wallops message (operator) |

**Examples:**

```
/msg friendname Hey, how are you?
/me waves hello
/notice #channel Server maintenance in 5 minutes
/query friendname
/ctcp friendname VERSION
/ping someuser
```

### 4.4 Window Commands

| Command | Aliases | Syntax | Description |
|---|---|---|---|
| `/window` | `/w` | `/window` | List all windows/buffers |
| `/window` | | `/window next` (or `/w n`) | Switch to next window |
| `/window` | | `/window prev` (or `/w p`) | Switch to previous window |
| `/window` | | `/window close` (or `/w c`) | Close current window |
| `/window` | | `/window goto <N\|name>` | Go to window by index or name |
| `/window` | | `/window <N>` | Go to window by index (shortcut) |
| `/window` | | `/window <#channel>` | Go to channel window (shortcut) |
| `/window` | | `/window list` | List all windows with details |
| `/window` | | `/window last` | Switch to the last active window |
| `/window` | | `/window number [N]` | Show/set window number |
| `/window` | | `/window name [newname]` | Show/set window name |
| `/window` | | `/window swap <N>` | Swap current window with another |
| `/window` | | `/window move <N>` | Move current window to position N |
| `/window` | | `/window hide` | Hide current window |
| `/window` | | `/window show <name>` | Show a hidden window |
| `/window` | | `/window split [name]` | Enable split-screen mode |
| `/window` | | `/window unsplit` | Disable split-screen mode |
| `/window` | | `/window level [level]` | Set window message level filter |
| `/window` | | `/window logfile [path]` | Set window log file |
| `/window` | | `/window server [N\|host]` | Bind window to a server |
| `/window` | | `/window notify [level]` | Set window notification level |
| `/window` | | `/window format [fmt]` | Set window format |
| `/window` | | `/window balance` | Balance split windows (50/50) |
| `/window` | | `/window grow` | Grow top split window |
| `/window` | | `/window shrink` | Shrink top split window |
| `/clear` | `/cls` | `/clear` | Clear the current window |
| `/lastlog` | `/ll` | `/lastlog [pattern]` | Search scrollback buffer |

**Examples:**

```
/window next                    -- cycle to next window
/window goto 3                  -- jump to window 3
/window goto #rust              -- jump to #rust window
/window split #other            -- split screen with #other
/window unsplit                 -- disable split screen
/lastlog error                  -- search scrollback for "error"
/clear                          -- clear current window
```

### 4.5 Configuration Commands

| Command | Aliases | Syntax | Description |
|---|---|---|---|
| `/set` | | `/set` | Show all settings |
| `/set` | | `/set <variable>` | Show a specific setting |
| `/set` | | `/set <variable> <value>` | Change a setting |
| `/alias` | | `/alias` | Show all aliases |
| `/alias` | | `/alias <name>` | Show a specific alias |
| `/alias` | | `/alias <name> <body>` | Define an alias |
| `/unalias` | | `/unalias <name>` | Remove an alias |
| `/highlight` | `/hilight` | `/highlight` | Show highlight patterns |
| `/highlight` | | `/highlight <pattern> [color]` | Add/toggle a highlight pattern |
| `/bind` | `/keybind` | `/bind` | Show key bindings |
| `/bind` | | `/bind <key> <action>` | Set a key binding |
| `/format` | | `/format` | Show all format templates |
| `/format` | | `/format <type>` | Show a specific format template |
| `/format` | | `/format <type> <template>` | Set a format template |
| `/status` | | `/status [format]` | Show/set status bar format |
| `/save` | | `/save` | Save all settings to database and config file |
| `/theme` | | `/theme` | Show current theme and available themes |
| `/theme` | | `/theme <name>` | Apply a color theme |
| `/flood` | | `/flood [on\|off] [rate] [per]` | Manage flood protection settings |

**Examples:**

```
/set SCROLLBACK 1000
/set BEEP_ON_MSG ON
/set AUTO_RECONNECT OFF
/alias hi /me says hello to $0!
/alias j /join $0
/unalias hi
/highlight mykeyword red
/highlight TODO green
/bind ALT-1 /window goto 1
/format JOIN * $0 has joined $1
/status [ $N ] [ $C ] [ $T ]
/save
/flood on 5 3
```

### 4.6 System Commands

| Command | Aliases | Syntax | Description |
|---|---|---|---|
| `/help` | `/?` | `/help [command]` | Show help for all or a specific command |
| `/raw` | `/quote` | `/raw <text>` | Send a raw IRC protocol line |
| `/echo` | | `/echo <text>` | Display local text in the current window |
| `/exec` | | `/exec <command>` | Execute a shell command and display output |
| `/log` | | `/log [on\|off]` | Toggle global logging |
| `/log` | | `/log <#channel> on\|off` | Toggle per-channel logging |
| `/log` | | `/log rotate <size_mb>` | Set log rotation size |
| `/eval` | | `/eval <expression>` | Evaluate an expression |
| `/timer` | | `/timer <seconds> <repeats> <command>` | Create a timer |
| `/timer` | | `/timer` | List active timers |
| `/notify` | | `/notify [nick]` | Add/remove from notify list |
| `/ignore` | | `/ignore [pattern] [flags]` | Manage ignore list |
| `/load` | | `/load <script.lua>` | Load a Lua script |
| `/reload` | | `/reload` | Reload all Lua scripts |
| `/cd` | | `/cd <path>` | Change working directory |
| `/pwd` | | `/pwd` | Print working directory |
| `/debug` | | `/debug [on\|off]` | Toggle debug mode |
| `/repaint` | `/refresh` | `/repaint` | Force screen redraw |
| `/scroll` | | `/scroll <up\|down\|top\|bottom>` | Programmatic scroll |
| `/play` | | `/play <logfile>` | Replay a log file into the current buffer |
| `/SHH` | | `/SHH` | Suppress next display output (epic6) |
| `/if` | | `/if <cond> <then> [else <else>]` | Conditional execution |
| `/while` | | `/while <cond> <body>` | Loop while condition is true |

**Examples:**

```
/raw PRIVMSG #channel :Hello from raw!
/exec curl -s https://api.example.com
/log on
/log #programming on
/log rotate 10
/timer 300 1 /msg #channel 5 minutes have passed!
/notify friendname
/ignore *!*@spammer.com ALL
/load my_script.lua
/if $N == mynick /echo That's me!
```

### 4.7 DCC Commands

| Command | Syntax | Description |
|---|---|---|
| `/dcc` | `/dcc` | Show DCC sessions |
| `/dcc` | `/dcc list` | List DCC sessions |
| `/dcc` | `/dcc chat <nick>` | Start DCC chat |
| `/dcc` | `/dcc send <nick> <file>` | Send file via DCC |
| `/dcc` | `/dcc get <id>` | Accept incoming DCC transfer |
| `/dcc` | `/dcc close <id>` | Close DCC session |

### 4.8 Server Information Commands

| Command | Syntax | Description |
|---|---|---|
| `/lusers` | `/lusers [mask] [target]` | Get server user statistics |
| `/admin` | `/admin [server]` | Get server admin information |
| `/info` | `/info [server]` | Get server information |
| `/motd` | `/motd [server]` | Get Message of the Day |
| `/stats` | `/stats <flag> [server]` | Get server statistics (flags: c, h, i, k, K, l, m, o, p, u, y, z, ?) |
| `/links` | `/links [mask]` | List connected servers |
| `/map` | `/map` | Display server network map |
| `/trace` | `/trace [target]` | Trace server connection route |
| `/tkline` | `/tkline <mins> <user@host> [reason]` | Temporary K-line (ircd 2.11) |

### 4.9 Alias Variables

Aliases support epic5-style variable expansion:

| Variable | Description | Example |
|---|---|---|
| `$0` - `$9` | Positional arguments | `/alias hi /msg $0 Hello $1!` |
| `$*` | All arguments | `/alias echo /echo $*` |
| `$N` | Your current nickname | `/alias whoami /echo I am $N` |
| `$C` | Current channel name | `/alias ch /echo Channel: $C` |
| `$S` | Current server hostname | `/alias srv /echo Server: $S` |
| `$$` | Literal `$` character | |

**Multi-command aliases** use `;` as separator:

```
/alias greet /me waves; /msg $0 Hello!
```

### 4.10 Conditional Expressions

The `/if` and `/while` commands support these operators:

| Operator | Description | Example |
|---|---|---|
| `==` or `eq` | Equality | `/if $N eq mynick /echo It's me` |
| `!=` or `ne` | Inequality | `/if $N != bot /echo Human` |
| `>` | Greater than | `/if $# > 5 /echo Many args` |
| `<` | Less than | |
| `>=` | Greater or equal | |
| `<=` | Less or equal | |
| `=~` | Pattern match (with `*`) | `/if $0 =~ *test* /echo Found` |

---

## 5. Lua Scripting Guide

### 5.1 Getting Started

Void embeds a **Lua 5.4** engine (via the `mlua` crate with vendored Lua). Scripts are loaded in this order:

1. **`config.lua`** — loaded from the working directory on startup
2. **`scripts/*.lua`** — all `.lua` files in the `scripts/` directory
3. **Manual loading** — via `/load <path>` command

### 5.2 Writing Your First Script

Create a file called `config.lua` (or any `.lua` file in `scripts/`):

```lua
-- config.lua — Void IRC Client configuration

-- Server settings (used if no CLI args provided)
config = {
    nickname = "my_bot",
    server = "irc.libera.chat",
    channels = {"#mychannel"}
}

-- Register a custom command
void.register_command("HELLO", "cmd_hello")
function cmd_hello(args)
    local name = args[1] or "world"
    void.echo("Hello, " .. name .. "! I am " .. void.nick())
end

-- Hook into IRC events
void.on("JOIN", "on_user_join")
function on_user_join(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    -- Don't greet ourselves
    if nick ~= void.nick() then
        void.echo("-!- Welcome to " .. channel .. ", " .. nick .. "!")
    end
end
```

### 5.3 The `void.*` API Reference

All API functions are accessed through the global `void` table.

#### 5.3.1 Display & Output

##### `void.echo(text)`

Display text in the status window.

```lua
void.echo("Hello from Lua!")
void.echo("-!- System message style")
```

##### `void.version()`

Returns the client version string.

```lua
local ver = void.version()  -- "void 0.3.0"
void.echo("Running: " .. ver)
```

#### 5.3.2 Messaging

##### `void.msg(target, text)`

Send a private message to a nick or channel.

```lua
void.msg("#general", "Hello channel!")
void.msg("friendname", "Private message")
```

##### `void.notice(target, text)`

Send a notice to a nick or channel.

```lua
void.notice("user123", "Your account will expire soon")
void.notice("#ops", "Alert: suspicious activity")
```

##### `void.me(target, action)`

Send an action message (/me).

```lua
void.me("#channel", "waves hello")
```

##### `void.ctcp(target, type, args)`

Send a CTCP request.

```lua
void.ctcp("someuser", "VERSION", "")
void.ctcp("someuser", "PING", os.time())
```

#### 5.3.3 Channel Operations

##### `void.join(channel [, key])`

Join a channel, optionally with a key.

```lua
void.join("#programming")
void.join("#secret", "mysecretpassword")
```

##### `void.part(channel [, reason])`

Leave a channel with an optional reason.

```lua
void.part("#oldchannel", "Goodbye!")
void.part("#boring")
```

##### `void.op(channel, nick)`

Give operator status to a user.

```lua
void.op("#channel", "trusteduser")
```

##### `void.deop(channel, nick)`

Remove operator status from a user.

```lua
void.deop("#channel", "formerop")
```

##### `void.voice(channel, nick)`

Give voice status to a user.

```lua
void.voice("#channel", "newuser")
```

##### `void.devoice(channel, nick)`

Remove voice status from a user.

```lua
void.devoice("#channel", "noisyuser")
```

##### `void.ban(channel, mask)`

Set a ban on a channel.

```lua
void.ban("#channel", "baduser!*@*")
void.ban("#channel", "*!*@spammer.com")
```

##### `void.unban(channel, mask)`

Remove a ban from a channel.

```lua
void.unban("#channel", "baduser!*@*")
```

##### `void.kick(channel, nick [, reason])`

Kick a user from a channel.

```lua
void.kick("#channel", "spammer", "Stop spamming!")
void.kick("#channel", "troll")
```

##### `void.mode(channel, modes)`

Set channel modes.

```lua
void.mode("#channel", "+nt")
void.mode("#channel", "+o trusteduser")
```

##### `void.topic(channel, text)`

Set the channel topic.

```lua
void.topic("#channel", "Welcome to #channel! Be nice.")
```

##### `void.invite(nick, channel)`

Invite a user to a channel.

```lua
void.invite("friend", "#private")
```

#### 5.3.4 User Information

##### `void.nick()`

Returns your current nickname.

```lua
local me = void.nick()
void.echo("I am: " .. me)
```

##### `void.nick_change(newnick)`

Change your nickname.

```lua
void.nick_change("NewNick")
```

##### `void.channel()`

Returns the current channel name (empty string if in status window).

```lua
local ch = void.channel()
if ch ~= "" then
    void.echo("Currently in: " .. ch)
end
```

##### `void.server()`

Returns the current server hostname.

```lua
void.echo("Connected to: " .. void.server())
```

##### `void.connected()`

Returns `true` if connected to a server, `false` otherwise.

```lua
if void.connected() then
    void.echo("Online!")
else
    void.echo("Offline")
end
```

##### `void.whois(nick)`

Send a WHOIS request for a user.

```lua
void.whois("someuser")
```

##### `void.away([message])`

Set or unset away status.

```lua
void.away("Gone for lunch")  -- set away
void.away(nil)               -- return from away
void.away()                  -- return from away
```

##### `void.quit([reason])`

Quit the client.

```lua
void.quit("Goodbye!")
void.quit()  -- default reason
```

#### 5.3.5 String Utilities

##### `void.match(pattern, text)`

Pattern matching with `*` wildcards. Returns `true` if the pattern matches.

```lua
if void.match("*test*", "this is a test string") then
    void.echo("Match found!")
end

if void.match("hello*", "hello world") then
    void.echo("Starts with hello")
end
```

##### `void.strip(text)`

Remove all IRC formatting codes (colors, bold, italic, etc.) from text.

```lua
local clean = void.strip("\x034Red text\x03 normal")
-- Returns: "Red text normal"
```

##### `void.length(text)`

Returns the byte length of a string.

```lua
local len = void.length("Hello")  -- 5
```

##### `void.sub(text, start, len)`

Extract a substring. `start` is 0-indexed.

```lua
local s = void.sub("Hello World", 6, 5)  -- "World"
local s2 = void.sub("Hello", 2)          -- "llo"
```

##### `void.upper(text)` / `void.lower(text)`

Convert text to uppercase or lowercase.

```lua
void.upper("hello")  -- "HELLO"
void.lower("WORLD")  -- "world"
```

##### `void.token(text, delimiter)`

Destructive string tokenizer (epic6 style). Returns two values: the part before the delimiter, and the remainder.

```lua
local first, rest = void.token("one:two:three", ":")
-- first = "one", rest = "two:three"
```

##### `void.coalesce(...)`

Returns the first non-empty argument (epic6 style).

```lua
local result = void.coalesce("", "", "hello", "world")
-- result = "hello"
```

#### 5.3.6 Cryptographic Functions

##### `void.sha256(text)`

Compute SHA-256 hash (hex-encoded).

```lua
local hash = void.sha256("password123")
-- "ef92b778bafe771e89245b89ecbc08a44a4e166c06659911881f383d4473e94f"
```

##### `void.sha512(text)`

Compute SHA-512 hash (hex-encoded).

```lua
local hash = void.sha512("hello")
```

##### `void.hmac_sha256(key, text)`

Compute HMAC-SHA-256 (hex-encoded).

```lua
local sig = void.hmac_sha256("secret_key", "message")
```

##### `void.pbkdf2(password, salt, iterations)`

PBKDF2 key derivation (HMAC-SHA-512). Returns base64-encoded key.

```lua
local key = void.pbkdf2("password", "salt", 10000)
```

#### 5.3.7 Encoding Functions

##### `void.base64_encode(text)` / `void.base64_decode(text)`

Base64 encoding and decoding.

```lua
local encoded = void.base64_encode("Hello World")
-- "SGVsbG8gV29ybGQ="
local decoded = void.base64_decode(encoded)
-- "Hello World"
```

##### `void.xform(mode, text)`

Multi-format encoder/decoder (epic6 style).

| Mode | Description |
|---|---|
| `+B85` | Base85 (ASCII85) encode |
| `-B85` | Base85 (ASCII85) decode |
| `+B64` | Base64 encode |
| `-B64` | Base64 decode |
| `+URL` | URL encode |
| `-URL` | URL decode |

```lua
local encoded = void.xform("+B85", "Hello")
local decoded = void.xform("-B85", encoded)

local url_safe = void.xform("+URL", "hello world&foo")
-- "hello%20world%26foo"
```

#### 5.3.8 File Operations

##### `void.file_read(path)`

Read the contents of a file. Returns the file content as a string, or an error message.

```lua
local content = void.file_read("/etc/hostname")
if not content:match("^Error:") then
    void.echo("File: " .. content)
end
```

##### `void.file_write(path, content)`

Write content to a file (overwrites). Returns `true` on success.

```lua
void.file_write("/tmp/void_note.txt", "Hello from Void!")
```

##### `void.file_append(path, content)`

Append content to a file. Returns `true` on success.

```lua
void.file_append("~/.void/notes.txt", os.date() .. ": Something happened\n")
```

#### 5.3.9 Miscellaneous

##### `void.send(raw)`

Send a raw IRC protocol line.

```lua
void.send("PRIVMSG #channel :Raw IRC message")
void.send("MODE #channel +o someuser")
```

##### `void.set(key, value)`

Set a client configuration variable.

```lua
void.set("BEEP_ON_MSG", "ON")
void.set("SCROLLBACK", "2000")
```

##### `void.get(key)`

Get a client configuration variable value.

```lua
local val = void.get("SCROLLBACK")
```

##### `void.timer(seconds, fn_name)`

Schedule a function to be called after a delay.

```lua
function delayed_greeting()
    void.msg("#channel", "Timer fired!")
end

void.timer(30, "delayed_greeting")  -- call after 30 seconds
```

##### `void.random(min, max)`

Generate a pseudo-random number in the range [min, max].

```lua
local roll = void.random(1, 6)
void.echo("You rolled: " .. roll)
```

##### `void.json_encode(text)`

Encode a string as a JSON string value.

```lua
local json = void.json_encode('Hello "World"')
-- "\"Hello \\\"World\\\"\""
```

### 5.4 Event Hooks

Use `void.on(event_type, function_name)` to register event handlers. The function receives an `args` table with event-specific arguments.

#### Available Event Types

| Event | Args[1] | Args[2] | Args[3] | Description |
|---|---|---|---|---|
| `JOIN` | nick | channel | — | User joined a channel |
| `PART` | nick | channel | reason | User left a channel |
| `QUIT` | nick | reason | — | User quit IRC |
| `KICK` | channel | kicked_nick | kicker | User was kicked |
| `NICK` | old_nick | new_nick | — | User changed nick |
| `MODE` | target | modes | args | Mode change |
| `TOPIC` | nick | topic_text | — | Topic changed |
| `PRIVMSG` | nick | target | text | Private message received |
| `NOTICE` | nick | target | text | Notice received |
| `CTCP` | nick | type | args | CTCP request received |
| `PUBLIC` | nick | channel | text | Public channel message |
| `MSG` | nick | text | — | Private message |
| `NICKINUSE` | nick | — | — | Nick already in use (433) |
| `INVITE` | nick | channel | — | Invited to a channel |
| `CONNECT` | — | — | — | Connected to server |
| `CAP` | subcommand | data | — | IRCv3 CAP event |
| `CHGHOST` | nick | new_user | new_host | User changed host (chghost) |
| `CONTEXT` | old_channel | new_channel | — | Window context changed (epic6) |

#### Example: Welcome Message on Join

```lua
void.on("JOIN", "welcome_new_user")
function welcome_new_user(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    if nick ~= void.nick() then
        void.msg(channel, "Welcome to " .. channel .. ", " .. nick .. "!")
    end
end
```

#### Example: Auto-Respond to CTCP

```lua
void.on("CTCP", "handle_ctcp")
function handle_ctcp(args)
    local nick = args[1] or ""
    local ctcp_type = (args[2] or ""):upper()
    if ctcp_type == "VERSION" then
        void.notice(nick, "\001VERSION My Custom Client v1.0\001")
    elseif ctcp_type == "PING" then
        void.notice(nick, "\001PING " .. (args[3] or "") .. "\001")
    end
end
```

#### Example: Log All Messages

```lua
void.on("PUBLIC", "log_all_messages")
function log_all_messages(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    local text = args[3] or ""
    local timestamp = os.date("%Y-%m-%d %H:%M:%S")
    void.file_append("~/.void/chatlog.txt",
        timestamp .. " " .. channel .. " <" .. nick .. "> " .. text .. "\n")
end
```

### 5.5 Registering Custom Commands

Use `void.register_command(name, function_name)` to register a slash command. The function receives an `args` table (array of strings).

```lua
void.register_command("GREET", "cmd_greet")
function cmd_greet(args)
    local target = args[1] or "everyone"
    local channel = void.channel()
    if channel ~= "" then
        void.msg(channel, "Hello, " .. target .. "! 👋")
    else
        void.echo("Hello, " .. target .. "!")
    end
end
```

Usage: `/greet friendname`

### 5.6 Complete Script Example

```lua
-- advanced_bot.lua — A feature-rich Lua script example

-- Configuration
local bot_config = {
    owner = "mynick",
    auto_greet = true,
    log_messages = true,
}

-- Track user join times
local join_times = {}

-- Command: /uptime — show how long we've been connected
local start_time = os.time()
void.register_command("UPTIME", "cmd_uptime")
function cmd_uptime(args)
    local elapsed = os.time() - start_time
    local hours = math.floor(elapsed / 3600)
    local mins = math.floor((elapsed % 3600) / 60)
    local secs = elapsed % 60
    void.echo("-!- Uptime: " .. hours .. "h " .. mins .. "m " .. secs .. "s")
end

-- Command: /calc <expr> — simple calculator
void.register_command("CALC", "cmd_calc")
function cmd_calc(args)
    if #args == 0 then
        void.echo("-!- Usage: /calc <expression>")
        return
    end
    local expr = table.concat(args, " ")
    -- Only allow safe math operations
    local fn = load("return " .. expr)
    if fn then
        local ok, result = pcall(fn)
        if ok then
            void.echo("-!- " .. expr .. " = " .. tostring(result))
        else
            void.echo("-!- Error: " .. tostring(result))
        end
    else
        void.echo("-!- Invalid expression")
    end
end

-- Command: /dice [NdS] — roll dice
void.register_command("DICE", "cmd_dice")
function cmd_dice(args)
    local spec = args[1] or "1d6"
    local n, s = spec:match("(%d+)d(%d+)")
    n = tonumber(n) or 1
    s = tonumber(s) or 6
    local total = 0
    local rolls = {}
    for i = 1, n do
        local roll = void.random(1, s)
        total = total + roll
        table.insert(rolls, roll)
    end
    local channel = void.channel()
    local msg = "Rolled " .. spec .. ": " .. table.concat(rolls, "+") .. " = " .. total
    if channel ~= "" then
        void.msg(channel, msg)
    else
        void.echo("-!- " .. msg)
    end
end

-- Track joins
void.on("JOIN", "track_joins")
function track_joins(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    if nick ~= void.nick() then
        join_times[nick .. "@" .. channel] = os.time()
    end
end

-- Command: /seen <nick> — when did a user last join?
void.register_command("SEEN", "cmd_seen")
function cmd_seen(args)
    if #args == 0 then
        void.echo("-!- Usage: /seen <nick>")
        return
    end
    local nick = args[1]
    local channel = void.channel()
    local key = nick .. "@" .. channel
    if join_times[key] then
        local ago = os.time() - join_times[key]
        void.echo("-!- " .. nick .. " joined " .. channel .. " " .. ago .. " seconds ago")
    else
        void.echo("-!- Haven't seen " .. nick .. " join " .. channel)
    end
end

void.echo("-!- advanced_bot.lua loaded")
```

---

## 6. Modules Guide

Void ships with **82 modules** total: 25 core LiCe5-compatible Lua modules, 7 color themes, and 50 additional feature modules. Load them all at once with:

```
/load modules/init.lua
```

Or add to your `config.lua`:

```lua
dofile("modules/init.lua")
```

### 6.1 ignore — Enhanced Ignore System

**File:** `modules/ignore.lua`  
**Commands:** `/ig`, `/ignore`

Enhanced ignore with pattern matching, flags, reasons, and timeouts.

| Command | Syntax | Description |
|---|---|---|
| `/ig` | `/ig <pattern> [flags] [reason "text"] [timeout N]` | Add/toggle ignore |
| `/ig` | `/ig` | Show ignore list |

**Flags:** `ALL`, `PUBLIC`, `MSG`, `NOTICE`, `CTCP`

**Examples:**

```
/ig *!*@spammer.com ALL
/ig annoyinguser MSG reason "keeps messaging me" timeout 3600
/ig trollbot PUBLIC
/ig *!*@*.spam.net ALL reason "known spammer" timeout 86400
```

### 6.2 gone — Away System with Random Reasons

**File:** `modules/gone.lua`  
**Commands:** `/gone`, `/back`, `/autoaway`

Set away status with random or custom reasons. Supports auto-away on idle.

| Command | Syntax | Description |
|---|---|---|
| `/gone` | `/gone [message]` | Set away with random or custom reason |
| `/gone` | `/gone off` or `/gone back` | Return from away |
| `/back` | `/back [message]` | Return from away with optional message |
| `/autoaway` | `/autoaway [seconds]` | Toggle/set auto-away timeout |

**Reason files:** `modules/gone.reasons`, `modules/back.reasons`

**Examples:**

```
/gone                        -- random away reason
/gone Gone for lunch         -- custom away reason
/back                        -- return with random back message
/back I'm back!              -- return with custom message
/autoaway 600                -- auto-away after 10 minutes
/autoaway on                 -- enable auto-away
/autoaway off                -- disable auto-away
```

### 6.3 kick — Enhanced Kick/Kickban

**File:** `modules/kick.lua`  
**Commands:** `/k`, `/kb`, `/rk`

Kick and kickban with random reasons from `modules/kick.reasons`.

| Command | Syntax | Description |
|---|---|---|
| `/k` | `/k <nick> [reason]` | Kick with optional random reason |
| `/kb` | `/kb <nick> [reason]` | Kickban with optional random reason |
| `/rk` | `/rk <nick>` | Kick with a random reason |

**Examples:**

```
/k spammer                   -- kick with random reason
/k troll Stop it!            -- kick with custom reason
/kb baduser                  -- ban and kick with random reason
/rk annoyinguser             -- random kick reason
```

### 6.4 mass — Mass Mode Commands

**File:** `modules/mass.lua`  
**Commands:** `/massop`, `/massdeop`, `/massvoice`, `/massdevoice`, `/masskick`, `/massban`

Apply mode changes to multiple users at once.

| Command | Syntax | Description |
|---|---|---|
| `/massop` | `/massop [pattern]` | Mass op users matching pattern |
| `/massdeop` | `/massdeop [pattern]` | Mass deop users matching pattern |
| `/massvoice` | `/massvoice [pattern]` | Mass voice users matching pattern |
| `/massdevoice` | `/massdevoice [pattern]` | Mass devoice users matching pattern |
| `/masskick` | `/masskick [reason]` | Mass kick (use with caution) |
| `/massban` | `/massban [pattern]` | Mass ban (use with caution) |

**Examples:**

```
/massop                      -- op everyone
/massvoice *                 -- voice everyone
/massdeop                    -- deop everyone
```

### 6.5 userlist — Bot-Style Auto-Op/Voice

**File:** `modules/userlist.lua`  
**Commands:** `/ul`, `/userlist`

Persistent user database with access levels. Automatically ops/voices users when they join.

**Access Levels:** `OWNER` (100), `ADMIN` (90), `OP` (80), `HALFOP` (70), `VOICE` (60), `FRIEND` (50), `NONE` (0)

| Command | Syntax | Description |
|---|---|---|
| `/ul` | `/ul` or `/ul list` | Show userlist |
| `/ul` | `/ul add <nick> [host] [level]` | Add user to userlist |
| `/ul` | `/ul del <nick>` | Remove user from userlist |
| `/ul` | `/ul op <nick>` | Set user level to OP |
| `/ul` | `/ul voice <nick>` | Set user level to VOICE |

**Examples:**

```
/ul add frienduser *!*@friend.host.com OP
/ul add vipuser *!*@vip.host VOICE
/ul del olduser
/ul list
```

### 6.6 alarm — Timer/Reminder System

**File:** `modules/alarm.lua`  
**Commands:** `/alarm`

Set named timers and reminders.

| Command | Syntax | Description |
|---|---|---|
| `/alarm` | `/alarm` | List active alarms |
| `/alarm` | `/alarm <seconds> <command>` | Set unnamed alarm |
| `/alarm` | `/alarm <name> <seconds> <command>` | Set named alarm |
| `/alarm` | `/alarm cancel <name\|id>` | Cancel an alarm |

**Examples:**

```
/alarm tea 300 /msg #channel Time for tea!
/alarm 60 /echo One minute has passed
/alarm cancel tea
```

### 6.7 reconnect — Auto-Reconnect with Channel Rejoin

**File:** `modules/reconnect.lua`  
**Commands:** `/reconnect`, `/rejoin`

Tracks joined channels and rejoins them after reconnect.

| Command | Syntax | Description |
|---|---|---|
| `/reconnect` | `/reconnect` | Reconnect and rejoin saved channels |
| `/rejoin` | `/rejoin` | Rejoin saved channels |

### 6.8 paste — Multi-Line Paste Mode

**File:** `modules/paste.lua`  
**Commands:** `/paste`

Buffer multiple lines before sending them all at once.

| Command | Syntax | Description |
|---|---|---|
| `/paste` | `/paste` | Start paste mode |
| `/paste` | `/paste <text>` | Add a line to the paste buffer |
| `/paste` | `/paste send` | Send all buffered lines |
| `/paste` | `/paste cancel` | Cancel and discard |
| `/paste` | `/paste show` | Show buffered lines |

**Examples:**

```
/paste
/paste Line 1 of code
/paste Line 2 of code
/paste Line 3 of code
/paste send
```

### 6.9 logman — Per-Channel Log Management

**File:** `modules/logman.lua`  
**Commands:** `/logman`

Automatic per-channel log file management with date-based filenames.

| Command | Syntax | Description |
|---|---|---|
| `/logman` | `/logman` | Show log manager status |
| `/logman` | `/logman on\|start [#channel]` | Start logging |
| `/logman` | `/logman off\|stop [#channel]` | Stop logging |
| `/logman` | `/logman auto` | Toggle auto-logging on join |
| `/logman` | `/logman dir [path]` | Show/set log directory |

**Examples:**

```
/logman on
/logman auto
/logman dir ~/.void/mylogs
```

### 6.10 autovoice — Auto-Voice on Join

**File:** `modules/autovoice.lua`  
**Commands:** `/autovoice`

Automatically gives voice (+v) to users when they join a channel.

| Command | Syntax | Description |
|---|---|---|
| `/autovoice` | `/autovoice [#channel]` | Toggle auto-voice for a channel |

**Examples:**

```
/autovoice #welcome           -- enable auto-voice for #welcome
/autovoice                    -- toggle for current channel
```

### 6.11 anti_flood — Anti-Flood Protection

**File:** `modules/anti_flood.lua`  
**Commands:** `/antiflood`

Detects and mitigates flood attacks from users.

| Command | Syntax | Description |
|---|---|---|
| `/antiflood` | `/antiflood [on\|off] [threshold]` | Toggle/configure anti-flood |

**Examples:**

```
/antiflood on
/antiflood 10                 -- set threshold to 10 messages
/antiflood off
```

### 6.12 highlight — Nick/Pattern Highlight

**File:** `modules/highlight.lua`  
**Commands:** `/lice_highlight`

Highlight messages containing your nick or custom patterns.

| Command | Syntax | Description |
|---|---|---|
| `/lice_highlight` | `/lice_highlight` | Show highlight patterns |
| `/lice_highlight` | `/lice_highlight <pattern> [color]` | Add/toggle highlight |

**Examples:**

```
/lice_highlight TODO yellow
/lice_highlight urgent red
/lice_highlight myproject
```

### 6.13 ctcp — Enhanced CTCP Replies

**File:** `modules/ctcp.lua`

Automatically responds to CTCP VERSION, USERINFO, CLIENTINFO, PING, and TIME requests.

**Configuration (in Lua):**

```lua
lice5.ctcp.version = "My Custom Client v2.0"
lice5.ctcp.userinfo = "I'm a friendly bot"
```

### 6.14 nickserv — NickServ Auto-Identify

**File:** `modules/nickserv.lua`  
**Commands:** `/ns`, `/nickserv`

Automatically identifies with NickServ on connect. Handles nick collision (GHOST + recover).

| Command | Syntax | Description |
|---|---|---|
| `/ns` | `/ns <password> [nick]` | Set NickServ password |
| `/ns` | `/ns on` | Enable auto-identify |
| `/ns` | `/ns off` | Disable auto-identify |
| `/ns` | `/ns` | Show NickServ status |

**Examples:**

```
/ns mysecretpassword
/ns mysecretpassword mynick
/ns off
```

### 6.15 channel_protect — Channel Protection

**File:** `modules/channel_protect.lua`  
**Commands:** `/protect`

Anti-kick, anti-deop, and channel guard features.

| Command | Syntax | Description |
|---|---|---|
| `/protect` | `/protect [#channel]` | Toggle protection for a channel |

When protection is enabled:
- **Anti-kick:** Automatically rejoins if kicked
- **Anti-deop:** Alerts when deoped

**Examples:**

```
/protect #mychannel
/protect                      -- toggle for current channel
```

### 6.16 invite — Invite Management

**File:** `modules/invite.lua`  
**Commands:** `/invlist`

Track and manage channel invites.

| Command | Syntax | Description |
|---|---|---|
| `/invlist` | `/invlist` or `/invlist list` | Show pending invites |
| `/invlist` | `/invlist accept [#channel]` | Accept an invite |
| `/invlist` | `/invlist reject [#channel]` | Reject an invite |

**Examples:**

```
/invlist                      -- show pending invites
/invlist accept #channel      -- accept invite to #channel
/invlist reject               -- reject all pending invites
```

### 6.17 dns — DNS Lookup

**File:** `modules/dns.lua`  
**Commands:** `/dns`

DNS resolution via WHOIS.

| Command | Syntax | Description |
|---|---|---|
| `/dns` | `/dns <nick\|host\|ip>` | Perform DNS lookup |

**Example:**

```
/dns someuser
/dns irc.libera.chat
```

### 6.18 signoff — Random Quit Messages

**File:** `modules/signoff.lua`  
**Commands:** `/signoff`

Quit with a random message from `modules/quit.reasons`.

| Command | Syntax | Description |
|---|---|---|
| `/signoff` | `/signoff [reason]` | Quit with random or custom reason |

**Examples:**

```
/signoff                      -- random quit message
/signoff Goodbye everyone!    -- custom quit message
```

### 6.19 wall — Broadcast to Channels

**File:** `modules/wall.lua`  
**Commands:** `/wall`

Send a notice to the current channel (wall-style broadcast).

| Command | Syntax | Description |
|---|---|---|
| `/wall` | `/wall <message>` | Send a wall notice |

**Example:**

```
/wall Server maintenance in 10 minutes!
```

### 6.20 finger — User Info Lookup

**File:** `modules/finger.lua`  
**Commands:** `/finger`

User information lookup (uses WHOIS).

| Command | Syntax | Description |
|---|---|---|
| `/finger` | `/finger <nick>` | Look up user info |

### 6.21 memo — Offline Memo System

**File:** `modules/memo.lua`  
**Commands:** `/memo`

Send offline messages to users (stored locally).

| Command | Syntax | Description |
|---|---|---|
| `/memo` | `/memo` | List all memos |
| `/memo` | `/memo send <nick> <message>` | Send a memo |
| `/memo` | `/memo check [nick]` | Check memos for a user |
| `/memo` | `/memo list` | List all memos |
| `/memo` | `/memo <nick> <message>` | Shorthand to send a memo |

**Examples:**

```
/memo send friendname Hey, check out this link!
/memo check
/memo list
```

### 6.22 note — Quick Notes

**File:** `modules/note.lua`  
**Commands:** `/note`

Quick note-taking system.

| Command | Syntax | Description |
|---|---|---|
| `/note` | `/note` | List all notes |
| `/note` | `/note add <text>` | Add a note |
| `/note` | `/note list` | List all notes |
| `/note` | `/note clear` | Clear all notes |
| `/note` | `/note <text>` | Shorthand to add a note |

**Examples:**

```
/note Remember to update the topic
/note TODO: fix the bot script
/note list
/note clear
```

### 6.23 party — Party Mode

**File:** `modules/party.lua`  
**Commands:** `/party`, `/disco`, `/dance`

Fun party commands with disco colors and random dance moves.

| Command | Syntax | Description |
|---|---|---|
| `/party` | `/party` | Activate party mode / send random party line |
| `/party` | `/party on\|off` | Toggle party mode |
| `/party` | `/party disco <text>` | Send text with disco rainbow colors |
| `/disco` | `/disco <text>` | Apply rainbow disco colors to text |
| `/dance` | `/dance` | Send random dance moves |

**Examples:**

```
/party on
/party disco Hello everyone!
/disco This text is rainbow!
/dance
/party off
```

### 6.24 sensors — Channel Activity Monitoring

**File:** `modules/sensors.lua`  
**Commands:** `/sensors`

Monitor and report channel activity (joins, parts, kicks, bans, messages).

| Command | Syntax | Description |
|---|---|---|
| `/sensors` | `/sensors` | Show sensor report for current channel |
| `/sensors` | `/sensors enable [#channel]` | Enable monitoring |
| `/sensors` | `/sensors disable [#channel]` | Disable monitoring |
| `/sensors` | `/sensors report [#channel]` | Show activity report |

**Examples:**

```
/sensors enable
/sensors report
/sensors disable
```

### 6.25 help — Enhanced Help System

**File:** `modules/help.lua`  
**Commands:** `/lice_help`

Categorized help system for all commands.

| Command | Syntax | Description |
|---|---|---|
| `/lice_help` | `/lice_help` | Show help categories |
| `/lice_help` | `/lice_help <category>` | Show commands in a category |

**Categories:** Channel, Message, Server, Window, Config, System, LiCe5

### 6.26 Theme System

Void includes 16 built-in color themes that change the entire look and feel of the client, featuring truecolor hex RGB support, high-contrast readability, custom role-based nick colors, and dynamic chat nick palettes.

**Commands:** `/theme`

| Command | Syntax | Description |
|---|---|---|
| `/theme` | `/theme` | Show current theme and available themes |
| `/theme` | `/theme <name>` | Apply a theme |
| `/theme` | `/theme list` | List all available themes grouped by dark/light |
| `/theme` | `/theme info <name>` | Show detailed theme info and color specifications |
| `/theme` | `/theme random` | Apply a random theme |

**Available Themes:**

| Theme | Type | Description |
|---|---|---|
| **Catppuccin** | Dark | Soothing pastel palette (Mocha variant) |
| **CatppuccinLatte** | Light | Soothing warm pastel light palette (Latte variant) |
| **Dracula** | Dark | Iconic dark theme with vibrant neon pink, purple, cyan, and green accents |
| **Nord** | Dark | Arctic north-bluish clean aesthetic (Polar Night + Frost + Aurora) |
| **Gruvbox** | Dark | Retro groove warm dark earth colors |
| **GruvboxLight** | Light | Retro warm parchment light palette |
| **Solarized** | Dark | Precision color palette by Ethan Schoonover (Dark variant) |
| **SolarizedLight** | Light | Precision low-contrast cream light palette |
| **TokyoNight** | Dark | Dark theme inspired by Tokyo's neon night lights |
| **Matrix** | Dark | Green phosphor CRT hacker terminal aesthetic |
| **Cyberpunk** | Dark | 80s retro-futuristic synthwave & cyberpunk neon |
| **Monokai** | Dark | Iconic developer high-contrast palette (Monokai Pro) |
| **OneDark** | Dark | Atom's classic balanced dark theme |
| **RosePine** | Dark | Soho vibes with muted rose, pine, and gold |
| **Irssi** | Dark | Nostalgic Irssi-style classic IRC theme with blue statusbar |
| **BitchX** | Dark | 90s legendary BitchX hacker aesthetic with red & cyan accents |

**Examples:**

```
/theme                    -- show current theme
/theme list               -- list available themes
/theme dracula            -- apply Dracula theme
/theme nord               -- apply Nord theme
/theme catppuccin         -- apply Catppuccin theme
/theme info tokyonight    -- show detailed info for TokyoNight
/theme random             -- switch to a random theme
```

### 6.27 banlist — Ban List Management

**File:** `modules/banlist.lua`
**Commands:** `/banlist`

View and manage the channel ban list.

| Command | Syntax | Description |
|---|---|---|
| `/banlist` | `/banlist` | Show bans for the current channel |
| `/banlist` | `/banlist [#channel]` | Show bans for a specific channel |

### 6.28 exclist — Exception List Management

**File:** `modules/exclist.lua`
**Commands:** `/exclist`

View and manage ban exception lists (+e).

| Command | Syntax | Description |
|---|---|---|
| `/exclist` | `/exclist` | Show ban exceptions for the current channel |
| `/exclist` | `/exclist [#channel]` | Show ban exceptions for a specific channel |

### 6.29 joinlist — Invite Exception List

**File:** `modules/joinlist.lua`
**Commands:** `/joinlist`

View and manage invite exception lists (+I).

| Command | Syntax | Description |
|---|---|---|
| `/joinlist` | `/joinlist` | Show invite exceptions for the current channel |
| `/joinlist` | `/joinlist [#channel]` | Show invite exceptions for a specific channel |

### 6.30 serverignore — Server-Level Ignore

**File:** `modules/serverignore.lua`
**Commands:** `/serverignore`

Manage server-wide ignore patterns (applied across all channels).

| Command | Syntax | Description |
|---|---|---|
| `/serverignore` | `/serverignore` | Show server ignore list |
| `/serverignore` | `/serverignore add <pattern> [flags]` | Add server-level ignore |
| `/serverignore` | `/serverignore del <pattern>` | Remove server-level ignore |

### 6.31 chanlog — Channel Log Viewer

**File:** `modules/chanlog.lua`
**Commands:** `/chanlog`

View and search channel log files.

| Command | Syntax | Description |
|---|---|---|
| `/chanlog` | `/chanlog [#channel]` | Show today's log for a channel |
| `/chanlog` | `/chanlog search <pattern>` | Search logs for a pattern |
| `/chanlog` | `/chanlog date <YYYY-MM-DD>` | Show log for a specific date |

### 6.32 news — News/Announcement System

**File:** `modules/news.lua`
**Commands:** `/news`

Read and manage news items and announcements.

| Command | Syntax | Description |
|---|---|---|
| `/news` | `/news` | Show latest news items |
| `/news` | `/news add <text>` | Add a news item |
| `/news` | `/news read [id]` | Read a specific news item |
| `/news` | `/news delete <id>` | Delete a news item |

### 6.33 update — Self-Update Check

**File:** `modules/update.lua`
**Commands:** `/update`

Check for Void client updates.

| Command | Syntax | Description |
|---|---|---|
| `/update` | `/update` | Check for available updates |

### 6.34 oops — Quick Correction

**File:** `modules/oops.lua`
**Commands:** `/oops`

Quickly correct your last message.

| Command | Syntax | Description |
|---|---|---|
| `/oops` | `/oops <correction>` | Send a correction for your last message |

**Example:**

```
Hello wrold!
/oops world
-- Sends: "Hello world! (was: wrold)"
```

### 6.35 splitlist — Split Screen List Management

**File:** `modules/splitlist.lua`
**Commands:** `/splitlist`

Manage saved split-screen configurations.

| Command | Syntax | Description |
|---|---|---|
| `/splitlist` | `/splitlist` | Show saved split configurations |
| `/splitlist` | `/splitlist save <name>` | Save current split layout |
| `/splitlist` | `/splitlist load <name>` | Restore a saved split layout |
| `/splitlist` | `/splitlist del <name>` | Delete a saved split layout |

### 6.36 showlist — Show List

**File:** `modules/showlist.lua`
**Commands:** `/showlist`

Display various internal lists (ignores, highlights, notifies, etc.).

| Command | Syntax | Description |
|---|---|---|
| `/showlist` | `/showlist <type>` | Show a list by type |

**Types:** `ignore`, `highlight`, `notify`, `userlist`, `ban`, `except`, `invite`

### 6.37 rmlist — Remove from List

**File:** `modules/rmlist.lua`
**Commands:** `/rmlist`

Remove entries from various internal lists.

| Command | Syntax | Description |
|---|---|---|
| `/rmlist` | `/rmlist <type> <pattern>` | Remove an entry from a list |

### 6.38 refriend — Re-Friend User

**File:** `modules/refriend.lua`
**Commands:** `/refriend`

Re-add a user to the userlist after removal.

| Command | Syntax | Description |
|---|---|---|
| `/refriend` | `/refriend <nick>` | Re-add user as friend |

### 6.39 rel — Release (Unban) User

**File:** `modules/rel.lua`
**Commands:** `/rel`

Quick unban/release of a user from the channel.

| Command | Syntax | Description |
|---|---|---|
| `/rel` | `/rel <nick>` | Remove all bans matching a user |

### 6.40 noig — Temporary Unignore

**File:** `modules/noig.lua`
**Commands:** `/noig`

Temporarily remove an ignore for a specific user.

| Command | Syntax | Description |
|---|---|---|
| `/noig` | `/noig <nick>` | Temporarily unignore a user |

### 6.41 pager — Pager System

**File:** `modules/pager.lua`
**Commands:** `/pager`

Pager/notification system for when you're away.

| Command | Syntax | Description |
|---|---|---|
| `/pager` | `/pager` | Show pager status |
| `/pager` | `/pager on\|off` | Toggle pager |
| `/pager` | `/pager read` | Read paged messages |

### 6.42 wget — Web Fetch

**File:** `modules/wget.lua`
**Commands:** `/wget`

Fetch content from a URL and display it.

| Command | Syntax | Description |
|---|---|---|
| `/wget` | `/wget <url>` | Fetch and display URL content |

### 6.43 trans — Translation

**File:** `modules/trans.lua`
**Commands:** `/trans`

Translate text between languages.

| Command | Syntax | Description |
|---|---|---|
| `/trans` | `/trans <lang> <text>` | Translate text to target language |
| `/trans` | `/trans <src>-<dst> <text>` | Translate between specific languages |

### 6.44 define — Dictionary Lookup

**File:** `modules/define.lua`
**Commands:** `/define`

Look up word definitions.

| Command | Syntax | Description |
|---|---|---|
| `/define` | `/define <word>` | Look up a word's definition |

### 6.45 sc — Screen Commands

**File:** `modules/sc.lua`
**Commands:** `/sc`

Screen/display manipulation shortcuts.

| Command | Syntax | Description |
|---|---|---|
| `/sc` | `/sc <action>` | Execute a screen action |

### 6.46 mk — Mark/Bookmark

**File:** `modules/mk.lua`
**Commands:** `/mk`

Mark or bookmark positions in scrollback.

| Command | Syntax | Description |
|---|---|---|
| `/mk` | `/mk [name]` | Set a mark at current position |
| `/mk` | `/mk goto <name>` | Jump to a named mark |

### 6.47 mme — Mass Me

**File:** `modules/mme.lua`
**Commands:** `/mme`

Send action messages to multiple channels.

| Command | Syntax | Description |
|---|---|---|
| `/mme` | `/mme <action>` | Send /me to all joined channels |

### 6.48 msay — Mass Say

**File:** `modules/msay.lua`
**Commands:** `/msay`

Send messages to multiple channels at once.

| Command | Syntax | Description |
|---|---|---|
| `/msay` | `/msay <text>` | Send text to all joined channels |

### 6.49 mtog — Mass Toggle

**File:** `modules/mtog.lua`
**Commands:** `/mtog`

Toggle modes across multiple channels.

| Command | Syntax | Description |
|---|---|---|
| `/mtog` | `/mtog <mode>` | Toggle a mode on all joined channels |

### 6.50 ctog — Channel Toggle

**File:** `modules/ctog.lua`
**Commands:** `/ctog`

Toggle channel modes.

| Command | Syntax | Description |
|---|---|---|
| `/ctog` | `/ctog <mode>` | Toggle a channel mode |

### 6.51 dtog — Deop Toggle

**File:** `modules/dtog.lua`
**Commands:** `/dtog`

Toggle deop protection.

| Command | Syntax | Description |
|---|---|---|
| `/dtog` | `/dtog` | Toggle deop protection |

### 6.52 wtog — Window Toggle

**File:** `modules/wtog.lua`
**Commands:** `/wtog`

Toggle window-level settings.

| Command | Syntax | Description |
|---|---|---|
| `/wtog` | `/wtog <setting>` | Toggle a window setting |

### 6.53 tog — General Toggle

**File:** `modules/tog.lua`
**Commands:** `/tog`

General-purpose toggle for various settings.

| Command | Syntax | Description |
|---|---|---|
| `/tog` | `/tog <setting>` | Toggle a setting on/off |

### 6.54 dom — Domain Lookup

**File:** `modules/dom.lua`
**Commands:** `/dom`

Domain/WHOIS lookup for IRC users.

| Command | Syntax | Description |
|---|---|---|
| `/dom` | `/dom <nick>` | Look up user's domain via WHOIS |

### 6.55 dump — Dump State

**File:** `modules/dump.lua`
**Commands:** `/dump`

Dump internal state for debugging.

| Command | Syntax | Description |
|---|---|---|
| `/dump` | `/dump [type]` | Dump internal state |

**Types:** `settings`, `aliases`, `hooks`, `vars`, `all`

### 6.56 ulsave — Userlist Save

**File:** `modules/ulsave.lua`
**Commands:** `/ulsave`

Manually save the userlist database.

| Command | Syntax | Description |
|---|---|---|
| `/ulsave` | `/ulsave` | Force-save the userlist to disk |

### 6.57 ulw_* — Userlist Window Commands

**File:** `modules/userlist.lua`
**Commands:** `/ulw_chat`, `/ulw_help`, `/ulw_ident`, `/ulw_invite`, `/ulw_op`, `/ulw_voice`, `/ulw_unban`, `/ulw_whoami`, `/ulw_pass`

Quick userlist operations via window commands:

| Command | Syntax | Description |
|---|---|---|
| `/ulw_chat` | `/ulw_chat <nick>` | Open a chat window with a userlist entry |
| `/ulw_help` | `/ulw_help` | Show userlist window help |
| `/ulw_ident` | `/ulw_ident <nick>` | Identify a user from the userlist |
| `/ulw_invite` | `/ulw_invite <nick> [#channel]` | Invite a userlist entry to a channel |
| `/ulw_op` | `/ulw_op <nick>` | Op a userlist entry |
| `/ulw_voice` | `/ulw_voice <nick>` | Voice a userlist entry |
| `/ulw_unban` | `/ulw_unban <nick>` | Unban a userlist entry |
| `/ulw_whoami` | `/ulw_whoami` | Show your userlist entry info |
| `/ulw_pass` | `/ulw_pass <password>` | Set userlist password |

### 6.58 tabcomp — Tab Completion Config

**File:** `modules/tabcomp.lua`
**Commands:** `/tabcomp`

Configure nick/tab completion behavior.

| Command | Syntax | Description |
|---|---|---|
| `/tabcomp` | `/tabcomp` | Show tab completion settings |
| `/tabcomp` | `/tabcomp <setting> <value>` | Change tab completion setting |

### 6.59 bword — Bad Word Filter

**File:** `modules/bword.lua`
**Commands:** `/bword`

Filter messages containing bad words.

| Command | Syntax | Description |
|---|---|---|
| `/bword` | `/bword` | Show bad word list |
| `/bword` | `/bword add <word>` | Add a bad word |
| `/bword` | `/bword del <word>` | Remove a bad word |
| `/bword` | `/bword on\|off` | Toggle bad word filter |

### 6.60 binds — Key Binding Presets

**File:** `modules/binds.lua`
**Commands:** `/binds`

Manage preset key binding configurations.

| Command | Syntax | Description |
|---|---|---|
| `/binds` | `/binds` | Show current bindings |
| `/binds` | `/binds <preset>` | Apply a binding preset |

### 6.61 defaults — Default Settings

**File:** `modules/defaults.lua`
**Commands:** `/defaults`

Reset settings to defaults.

| Command | Syntax | Description |
|---|---|---|
| `/defaults` | `/defaults` | Show default settings |
| `/defaults` | `/defaults reset` | Reset all settings to defaults |
| `/defaults` | `/defaults reset <setting>` | Reset a specific setting |

### 6.62 imail — Internal Mail

**File:** `modules/imail.lua`
**Commands:** `/imail`

Internal mail system for offline messages.

| Command | Syntax | Description |
|---|---|---|
| `/imail` | `/imail` | Check for new mail |
| `/imail` | `/imail send <nick> <message>` | Send internal mail |
| `/imail` | `/imail read` | Read unread mail |
| `/imail` | `/imail list` | List all mail |

### 6.63 floodlist — Flood List Management

**File:** `modules/floodlist.lua`
**Commands:** `/floodlist`

View and manage the flood protection list.

| Command | Syntax | Description |
|---|---|---|
| `/floodlist` | `/floodlist` | Show flood list |
| `/floodlist` | `/floodlist clear` | Clear flood list |

### 6.64 looplist — Loop List Management

**File:** `modules/looplist.lua`
**Commands:** `/looplist`

View and manage active timer loops.

| Command | Syntax | Description |
|---|---|---|
| `/looplist` | `/looplist` | Show active loops |
| `/looplist` | `/looplist stop <id>` | Stop a specific loop |

### 6.65 pic — ASCII Art

**File:** `modules/pic.lua`
**Commands:** `/pic`

Display ASCII art pictures.

| Command | Syntax | Description |
|---|---|---|
| `/pic` | `/pic [name]` | Display an ASCII art picture |

### 6.66 ppl — People/User Info

**File:** `modules/ppl.lua`
**Commands:** `/ppl`

Quick user information display.

| Command | Syntax | Description |
|---|---|---|
| `/ppl` | `/ppl [nick]` | Show info about people in channel |

### 6.67 chanst — Channel Status

**File:** `modules/chanst.lua`
**Commands:** `/chanst`

Show detailed channel status information.

| Command | Syntax | Description |
|---|---|---|
| `/chanst` | `/chanst [#channel]` | Show channel status details |

### 6.68 cwho — Channel Who

**File:** `modules/cwho.lua`
**Commands:** `/cwho`

Enhanced WHO for channels with filtering.

| Command | Syntax | Description |
|---|---|---|
| `/cwho` | `/cwho [pattern]` | WHO with channel context |

### 6.69 et — Elapsed Time

**File:** `modules/et.lua`
**Commands:** `/et`

Show elapsed time / uptime tracking.

| Command | Syntax | Description |
|---|---|---|
| `/et` | `/et` | Show elapsed time since connect |

### 6.70 db — Database Commands

**File:** `modules/db.lua`
**Commands:** `/db`

Direct database query commands.

| Command | Syntax | Description |
|---|---|---|
| `/db` | `/db <query>` | Execute a database query |
| `/db` | `/db tables` | List database tables |

### 6.71 fkey — Function Keys

**File:** `modules/fkey.lua`
**Commands:** `/fkey`

Configure function key bindings (F1-F12).

| Command | Syntax | Description |
|---|---|---|
| `/fkey` | `/fkey` | Show function key bindings |
| `/fkey` | `/fkey <N> <action>` | Set function key N binding |

### 6.72 boot — Boot/Eject User

**File:** `modules/boot.lua`
**Commands:** `/boot`

Boot a user from the channel (kick + ban + timed unban).

| Command | Syntax | Description |
|---|---|---|
| `/boot` | `/boot <nick> [seconds] [reason]` | Boot user with timed unban |

**Example:**

```
/boot spammer 300 Stop spamming
-- Kicks and bans for 5 minutes, then auto-unbans
```

---

## 7. Configuration

### 7.1 Configuration Files

Void uses three configuration mechanisms:

| File | Format | Purpose |
|---|---|---|
| `config.lua` | Lua script | Primary configuration (loaded on startup) |
| `~/.void/void.db` | SQLCipher SQLite | Persistent encrypted storage |
| `~/.void/void.conf` | INI-style text | Text backup of settings |

**Loading order:**

1. Default settings (hardcoded in Rust)
2. `~/.void/void.db` (SQLite database)
3. `~/.void/void.conf` (text config)
4. `config.lua` (Lua configuration)
5. CLI arguments (override everything)

### 7.2 config.lua Structure

```lua
-- Server connection (used if no CLI args)
config = {
    nickname = "my_nick",           -- Default nickname
    server = "irc.libera.chat",     -- Default server
    channels = {"#channel1", "#ch2"} -- Channels to auto-join
}

-- Load modules
dofile("modules/init.lua")

-- Register custom commands
void.register_command("MYCMD", "my_handler")
function my_handler(args)
    void.echo("Custom command!")
end

-- Set up event hooks
void.on("JOIN", "my_join_hook")
function my_join_hook(args)
    -- Handle join events
end

-- Configure settings
void.set("BEEP_ON_MSG", "ON")
void.set("SCROLLBACK", "2000")
```

### 7.3 Settings Reference (`/set`)

All settings are accessible via `/set <variable> <value>`. Values are case-insensitive for boolean settings (`ON`/`OFF`).

#### Display Settings

| Variable | Default | Description |
|---|---|---|
| `SCROLL_LINES` | `1` | Number of lines to scroll per step |
| `SCROLLBACK` | `500` | Maximum scrollback buffer size per window |
| `SHOW_TIMESTAMPS` | `ON` | Show timestamps on messages |
| `TIMESTAMP_FORMAT` | `%H:%M` | Timestamp format (strftime) |
| `CLOCK_24HOUR` | `ON` | Use 24-hour clock format |
| `SHOW_CHANNEL_NAMES` | `ON` | Show channel names in status bar |
| `SHOW_STATUS_ALL` | `ON` | Show status bar for all windows |
| `STATUS_FORMAT` | ` [ $N ] [ $C ] [ $T ] ` | Status bar format template |
| `INPUT_PROMPT` | `> ` | Input line prompt |
| `SHOW_NICKLIST` | `ON` | Show/hide nick list panel |
| `SHOW_STATUSBAR` | `ON` | Show/hide status bar |
| `SHOW_USER_COUNT` | `ON` | Show/hide user counts in nick list headers |
| `MOUSE` | `OFF` | Enable/disable mouse capture |
| `NICK_WIDTH` | `18` | Nick list panel width (12-40) |
| `SSL_VERIFY` | `OFF` | Verify SSL certificates |
| `DEBUG` | `OFF` | Debug mode (enables raw log) |

#### Notification Settings

| Variable | Default | Description |
|---|---|---|
| `BEEP_ON_MSG` | `OFF` | Beep on highlight messages |

#### Logging Settings

| Variable | Default | Description |
|---|---|---|
| `LOG` | `OFF` | Global logging toggle |
| `LOG_FILE` | `void.log` | Default log file path |

#### Network Settings

| Variable | Default | Description |
|---|---|---|
| `AUTO_RECONNECT` | `ON` | Auto-reconnect on disconnect |
| `AUTO_RECONNECT_DELAY` | `15` | Seconds before reconnect attempt |
| `SSL_VERIFY` | `OFF` | Verify TLS certificates |
| `CTCP_REPLY` | `ON` | Respond to CTCP requests |

#### Flood Protection

| Variable | Default | Description |
|---|---|---|
| `FLOOD_PROTECTION` | `ON` | Enable outgoing flood protection |
| `FLOOD_RATE` | `4` | Max messages per window |
| `FLOOD_RATE_PER` | `2` | Time window in seconds |

#### DCC Settings

| Variable | Default | Description |
|---|---|---|
| `DCC_DOWNLOAD_DIR` | `~/dcc` | Directory for DCC file downloads |

#### Debug Settings

| Variable | Default | Description |
|---|---|---|
| `DEBUG` | `OFF` | Enable debug mode |

#### Status Bar Variables

The `STATUS_FORMAT` string supports these variables:

| Variable | Description |
|---|---|
| `$N` | Current nickname |
| `$C` | Current channel |
| `$T` | Channel topic |
| `$H` | Server hostname |
| `$S` | Server name |

### 7.4 SQLite Database

The database at `~/.void/void.db` is encrypted with **SQLCipher** (AES-256). The passphrase is derived from the system hostname and username by default, or set via `--db-pass`.

#### Database Schema

| Table | Columns | Description |
|---|---|---|
| `settings` | `key TEXT PK`, `value TEXT` | Client settings |
| `aliases` | `name TEXT PK`, `body TEXT` | Command aliases |
| `highlights` | `pattern TEXT PK`, `color TEXT` | Highlight patterns |
| `key_bindings` | `key TEXT PK`, `action TEXT` | Custom key bindings |
| `servers` | `host TEXT PK`, `port INT`, `tls INT`, `nick TEXT`, `password TEXT`, `nickserv_pass TEXT`, `auto_join TEXT` | Saved server connections |
| `notify_list` | `nick TEXT PK` | Notify list entries |
| `ignore_list` | `pattern TEXT PK`, `flags TEXT` | Ignore list entries |

The database is auto-saved every 5 minutes and on clean exit.

### 7.5 void.conf Format

The text config file uses INI-style sections:

```ini
# Void IRC Client configuration
# Auto-generated by /save

[settings]
AUTO_RECONNECT=ON
AUTO_RECONNECT_DELAY=15
BEEP_ON_MSG=OFF
SCROLLBACK=500

[aliases]
HI=/me says hello to $0!
J=/join $0
```

---

## 8. IRCv3 Features

Void supports several IRCv3 specifications for modern IRC functionality.

### 8.1 CAP Negotiation

Void automatically negotiates IRCv3 capabilities with the server during connection. The client requests capabilities and processes server responses.

**Supported CAP subcommands:** `LS`, `LIST`, `REQ`, `ACK`, `NAK`, `NEW`, `DEL`

**Lua CAP Hooks:** Use `void.on("CAP", handler)` to react to CAP events in scripts:

```lua
void.on("CAP", "on_cap")
function on_cap(args)
    local subcommand = args[1] or ""
    local data = args[2] or ""
    if subcommand == "ACK" then
        void.echo("-!- Server acknowledged capabilities: " .. data)
    elseif subcommand == "NAK" then
        void.echo("-!- Server rejected capabilities: " .. data)
    end
end
```

### 8.2 SASL Authentication

Void supports three SASL mechanisms:

#### SASL PLAIN

The most common mechanism. Sends `nick\0nick\0password` encoded in base64.

```bash
# Via CLI
void -c irc.libera.chat -n mynick --sasl "mynick:password"
```

#### SASL EXTERNAL

Authenticates using a client TLS certificate. Requires the server to have your certificate fingerprint registered.

```bash
# Via CLI
void -c irc.libera.chat -n mynick --sasl EXTERNAL
```

#### SASL SCRAM-SHA-512

A challenge-response mechanism using PBKDF2 and HMAC-SHA-512. Void implements the full SCRAM-SHA-512 state machine:

1. **Client-first:** Sends `n,,n=<nick>,r=<nonce>`
2. **Server-first:** Receives `r=<nonce>,s=<salt>,i=<iterations>`
3. **Client-final:** Computes `SaltedPassword`, `ClientKey`, `AuthKey`, `ClientSignature`, `ClientProof`
4. **Server-final:** Verifies `ServerSignature`

```bash
# Via CLI (same as PLAIN format)
void -c irc.libera.chat -n mynick --sasl "mynick:password"
```

The server and client negotiate the strongest available mechanism automatically.

### 8.3 MONITOR

The MONITOR extension allows tracking when users go online or offline without polling. When the server supports `MONITOR`, Void uses it instead of `ISON` for the notify list.

```
/notify friendname    -- adds to MONITOR list
```

The client sends `MONITOR + nick1,nick2,...` and receives notifications when users connect/disconnect.

### 8.4 away-notify

When the server supports `away-notify`, Void receives real-time notifications when users set or unset their away status, without needing to poll.

### 8.5 Other Supported Features

| Feature | Description |
|---|---|
| `message-tags` | Message tag support |
| `server-time` | Server timestamp on messages |
| `batch` | Batched message processing |
| `echo-message` | Server echoes back your messages |
| `account-notify` | Account change notifications |
| `chghost` | Host/user change notifications (user@host changes) |
| `extended-join` | JOIN messages include account and realname |
| `multi-prefix` | Multiple nick prefixes in NAMES |

### 8.6 ISUPPORT (005)

Void parses RPL_ISUPPORT (005) tokens to learn server capabilities:

| Token | Stored In | Description |
|---|---|---|
| `NETWORK` | `server_info.network` | Network name |
| `CHANTYPES` | `server_info.chantypes` | Channel type prefixes (e.g., `#&+!`) |
| `PREFIX` | `server_info.prefix_modes` | Nick prefix modes (e.g., `(qaohv)~&@%+`) |
| `CHANMODES` | `server_info.chanmodes` | Channel mode categories |
| `NICKLEN` | `server_info.nicklen` | Maximum nickname length |
| `TOPICLEN` | `server_info.topiclen` | Maximum topic length |
| `CHANNELLEN` | `server_info.channellen` | Maximum channel name length |
| `MODES` | `server_info.modes` | Max modes per line |

### 8.7 epic6 Features

Void incorporates several features from the **epic6** IRC client:

#### Message Breaking

Long messages are automatically split at word boundaries to fit within the IRC protocol limit (512 bytes). Void intelligently breaks messages to avoid splitting words, URLs, or code blocks.

#### `/ON CONTEXT` Hooks

The `/ON CONTEXT` event fires when the user switches window context (changes the active window/channel). This allows scripts to perform actions when the user navigates between windows.

```lua
void.on("CONTEXT", "on_context_change")
function on_context_change(args)
    local old_channel = args[1] or ""
    local new_channel = args[2] or ""
    void.echo("Switched from " .. old_channel .. " to " .. new_channel)
end
```

#### `/SHH` Command

The `/SHH` command suppresses the next display output. This is useful for scripting when you want to execute a command silently without showing its output to the user.

```
/SHH /whois someuser    -- executes WHOIS but suppresses output
```

#### POLICY State

Void tracks server POLICY state information, which describes server-enforced policies such as message rate limits, nick change throttles, and channel join limits. This information is used internally for flood protection and rate limiting.

#### Scrollback Indicator

When scrolled back in the scrollback buffer, a visual indicator shows that the current view is not at the bottom. New messages still arrive but the view stays at the scrolled position until the user scrolls to the bottom or presses `Ctrl+L`.

#### SCRAM-SHA-512

Full SCRAM-SHA-512 SASL authentication is implemented as described in [Section 8.2](#82-sasl-authentication). This is the strongest SASL mechanism available and is negotiated automatically when the server supports it.

---

## 9. Keyboard Shortcuts

### 9.1 Navigation

| Key | Action |
|---|---|
| `Ctrl+N` | Next window |
| `Ctrl+P` | Previous window |
| `Ctrl+X` | Cycle windows (epic5 default) |
| `Alt+1` through `Alt+9` | Jump to window by number |
| `PageUp` | Scroll up (10 lines × SCROLL_LINES) |
| `PageDown` | Scroll down |
| `Mouse Scroll Up` | Scroll up (split-aware) |
| `Mouse Scroll Down` | Scroll down (split-aware) |
| `Mouse Click` (status bar) | Switch to clicked window |

### 9.2 Input Editing

| Key | Action |
|---|---|
| `Left` / `Right` | Move cursor |
| `Home` | Move to start of line |
| `End` | Move to end of line |
| `Ctrl+A` | Move to start of line |
| `Ctrl+E` | Move to end of line |
| `Ctrl+U` | Clear entire line |
| `Ctrl+K` | Kill from cursor to end of line |
| `Ctrl+W` | Delete word before cursor |
| `Ctrl+R` | Reverse search in history |
| `Alt+B` | Move cursor one word left |
| `Alt+F` | Move cursor one word right |
| `Alt+D` | Delete word after cursor |
| `Backspace` | Delete character before cursor |
| `Delete` | Delete character at cursor |
| `Tab` | Nick completion |
| `Up` | Previous command in history |
| `Down` | Next command in history |
| `Enter` | Send message / execute command |

### 9.3 IRC Formatting Codes

These key combinations insert mIRC formatting control characters into the input:

| Key | Code | Description |
|---|---|---|
| `Alt+K` | `\x03` | Color code (followed by fg,bg numbers) |
| `Alt+B` | `\x02` | Bold toggle |
| `Alt+U` | `\x1F` | Underline toggle |
| `Alt+I` | `\x1D` | Italic toggle |
| `Alt+R` | `\x16` | Reverse toggle |
| `Alt+O` | `\x0F` | Reset all formatting |

**Color code usage:** After pressing `Alt+K`, type the color number:

```
Alt+K 4Hello Alt+K    -- "Hello" in red
Alt+K 4,7Hello Alt+K  -- "Hello" in red on white
```

**Standard mIRC color numbers:**

| Number | Color | Number | Color |
|---|---|---|---|
| 0 | White | 8 | Yellow |
| 1 | Black | 9 | Light Green |
| 2 | Blue | 10 | Cyan |
| 3 | Green | 11 | Light Cyan |
| 4 | Red | 12 | Light Blue |
| 5 | Brown | 13 | Pink |
| 6 | Purple | 14 | Grey |
| 7 | Orange | 15 | Light Grey |

### 9.4 Other

| Key | Action |
|---|---|
| `Ctrl+C` | Quit the client |
| `Ctrl+L` | Refresh screen / reset scroll |

---

## 10. IRC Proxy/Bouncer

Void can act as an IRC bouncer server, allowing other IRC clients to connect and share the same IRC sessions.

### 10.1 Starting the Bouncer

```
/bouncer start <port> [password]
```

Example:
```
/bouncer start 6667 mypassword
```

### 10.2 Connecting to the Bouncer

From another IRC client:
```
/server localhost 6667 mypassword
```

### 10.3 Bouncer Commands

| Command | Description |
|---------|-------------|
| `/bouncer start <port> [password]` | Start bouncer on specified port |
| `/bouncer stop` | Stop bouncer |
| `/bouncer status` | Show connected clients |

---

## 11. irssi Parity Features

Void implements all high and medium priority features from irssi:

### 11.1 Lag Meter
- Real-time latency measurement via PING/PONG
- Displayed in status bar with color coding (green <100ms, yellow <300ms, red >300ms)
- `%L` format variable

### 11.2 Raw Log Viewer
- `/rawlog on|off|show|save|clear`
- Records all raw IRC protocol messages
- Limit 1000 entries

### 11.3 Lastlog Search
- `/lastlog <pattern>` — case-insensitive search
- `/lastlog /regex/` — regex pattern match
- `/lastlog -level msg|action|notice|system|error` — filter by type
- `/lastlog -window #channel` — search specific buffer

### 11.4 Netsplit Detection
- Detects netsplits from QUIT reasons containing `*.net *.split`
- Tracks lost nicks during netsplit
- Announces recovery when nicks rejoin

### 11.5 Session Save/Restore
- Saves current buffer list to SQLite
- Auto-joins saved channels on reconnect

### 11.6 Ban List Tracking
- Tracks bans per channel from RPL_BANLIST
- Persistent during session

### 11.7 Window Layout Persistence
- Saves split state and direction to SQLite

### 11.8 Chatnet/Network Config
- `/chatnet add <name> <servers> [port] [tls]` — create network config
- `/chatnet del <name>` — remove
- `/chatnet list` — show all

### 11.9 Notify List with WHOIS Verification
- ISON polling every 60 seconds
- WHOIS verification for newly online nicks
- Tracks userhost, channels, last_seen

### 11.10 Massjoin Batching
- Detects rapid JOINs within 500ms window
- Batches and displays as single message

### 11.11 Nickmatch Cache
- Caches pattern matching results for performance
- Limit 1000 entries

### 11.12 DCC Chat
- `/dcc chat <nick>` — send DCC CHAT request
- DccChatSession tracking

### 11.13 DCC Resume
- `resume_send()` — resume interrupted transfers

### 11.14 Server Redirect Tracking
- Correlates responses to requests

### 11.15 Character Encoding
- `/charset <encoding>` — set per-buffer charset
- Supports UTF-8, ISO-8859-1/2/15, WINDOWS-1252/1250/1251, ASCII

---

## 12. Architecture

### 10.1 Source Code Structure

Void is organized into the following Rust modules:

```
src/
├── main.rs           # Entry point, CLI parsing, main event loop
├── lib.rs            # Module declarations and re-exports
├── app.rs            # Core application state (App struct)
├── commands/
│   └── registry.rs   # Command registry with 100+ built-in commands
├── irc/
│   ├── connection.rs # IRC connection management, SASL, proxy
│   └── proto.rs      # IRC protocol message parsing and handling
├── scripting/
│   ├── api.rs        # Lua API (void.* table with 50+ functions)
│   └── engine.rs     # Lua engine init, script loading
├── ui/
│   ├── input.rs      # Keyboard input handling, nick completion
│   ├── renderer.rs   # TUI rendering with ratatui
│   ├── statusbar.rs  # Status bar rendering
│   ├── scrollback.rs # Scrollback buffer management
│   └── handler.rs    # UI event handler
├── storage.rs        # SQLCipher encrypted SQLite persistence
├── logging.rs        # Log file management with rotation
├── flood.rs          # Outgoing flood protection
├── dcc.rs            # DCC file transfer manager
└── motd.rs           # ASCII art MOTD generator
```

### 10.2 Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                        Main Event Loop                       │
│                     (tokio::select! in main.rs)              │
├─────────┬──────────┬──────────┬──────────┬──────────────────┤
│ Terminal│   IRC    │   Lua    │  Timer   │     Mouse        │
│  Input  │  Events  │ Commands │  Ticks   │    Events        │
│  (key)  │  (msg)   │  (cmd)   │  (1s)    │   (scroll)       │
└────┬────┴────┬─────┴────┬─────┴────┬─────┴───────┬──────────┘
     │         │          │          │             │
     ▼         ▼          ▼          ▼             ▼
┌─────────┐ ┌────────┐ ┌────────┐ ┌─────────┐ ┌──────────┐
│handle_key│ │handle_ │ │Execute │ │ Check   │ │ Scroll/  │
│ (input.rs)│ │irc_msg │ │as /cmd│ │ timers  │ │ Click    │
│          │ │(proto) │ │        │ │         │ │          │
└────┬─────┘ └───┬────┘ └───┬────┘ └────┬────┘ └─────┬────┘
     │           │          │           │            │
     ▼           ▼          ▼           ▼            ▼
┌─────────────────────────────────────────────────────────────┐
│                      App State (app.rs)                      │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌───────────────┐  │
│  │ Buffers  │ │ Servers  │ │ Settings │ │ Lua Hooks     │  │
│  │ (windows)│ │ (multi)  │ │ (/set)   │ │ (commands +   │  │
│  │          │ │          │ │          │ │  events)      │  │
│  └──────────┘ └──────────┘ └──────────┘ └───────────────┘  │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌───────────────┐  │
│  │ Aliases  │ │ Notify   │ │ Ignore   │ │ Timers        │  │
│  │          │ │ List     │ │ List     │ │               │  │
│  └──────────┘ └──────────┘ └──────────┘ └───────────────┘  │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────┐
│                    Persistence Layer                          │
│  ┌──────────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ SQLCipher (SQLite)│  │ void.conf    │  │ Log Files    │  │
│  │ ~/.void/void.db  │  │ (text backup)│  │ (per-channel)│  │
│  └──────────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### 10.3 Module Interaction

#### Command Resolution Order

When the user types a command (e.g., `/hello args`):

1. **Built-in commands** — checked first via `CommandRegistry::find()`
2. **Aliases** — checked via `App::resolve_alias()` with variable expansion
3. **Lua commands** — checked via `LuaHooks::commands` (registered with `void.register_command()`)
4. **Error** — "Unknown command" if none match

#### Event Dispatch

When an IRC message arrives:

1. **IRC protocol handler** (`proto.rs`) processes the raw message
2. **Lua event hooks** are fired via `fire_event()` for the event type
3. **App state** is updated (buffers, nick lists, etc.)
4. **UI** is redrawn on the next frame

#### Lua Communication

Lua scripts communicate with the main application through:

- **`LuaCommand` channel** — Lua functions send commands (MSG, RAW, ECHO, SET) via an `mpsc::channel`
- **`LuaHooks`** — Shared state (via `Arc<Mutex<>>`) storing registered commands and event handlers
- **`LuaContext`** — Shared state with current nick, channel, server, and connection status

### 10.4 Key Design Decisions

| Decision | Rationale |
|---|---|
| **Rust + Lua** | Rust for performance and safety; Lua for extensibility |
| **ratatui TUI** | Modern, maintained TUI framework with rich widget support |
| **SQLCipher** | Encrypted storage protects passwords and private data |
| **tokio async** | Non-blocking I/O for network, timers, and input |
| **epic5 compatibility** | Familiar command set for IRC power users |
| **LiCe5 modules** | Proven module system with 82 modules (25 core + 7 themes + 50 additional) |
| **IRCv3 support** | Modern IRC features (SASL, MONITOR, away-notify) |
| **Message breaking** | Automatic splitting of long messages at word boundaries |

---

## 11. Troubleshooting

### 11.1 Connection Issues

#### "Connection refused" or "Connection timed out"

```
-!- IRC Error: Connection refused
```

**Solutions:**
- Verify the server hostname and port: `/server irc.libera.chat 6697`
- Check if TLS is required (most servers on port 6697 require TLS)
- Try `--no-tls` if the server doesn't support TLS
- Check your firewall settings
- Try a different server port (6667 for plain, 6697 for TLS)

#### "TLS handshake failed"

**Solutions:**
- The server may have an invalid certificate. Try `SET SSL_VERIFY OFF`
- Update your system's CA certificates
- Check if the server requires a specific TLS version

#### "Nickname already in use" (ERR_NICKNAMEINUSE 433)

**Solutions:**
- Use `--nickserv` to auto-identify and ghost: `void --nickserv "password"`
- Use the NickServ module: `/ns password mynick`
- Choose a different nickname: `/nick alternatename`

### 11.2 SASL Issues

#### SASL authentication fails

**Solutions:**
- Verify credentials: `--sasl "nick:password"` (note the colon separator)
- For EXTERNAL, ensure your client certificate fingerprint is registered with NickServ
- Check if the server supports the SASL mechanism you're using
- Enable debug mode: `/debug on` to see CAP negotiation

### 11.3 Database Issues

#### "Cannot open database" or encryption errors

**Solutions:**
- Ensure `~/.void/` directory exists and is writable
- If you get encryption errors, the passphrase may have changed. Try: `--db-pass "your-passphrase"`
- Delete `~/.void/void.db` to start fresh (you'll lose saved settings)
- Default passphrase is derived from `hostname-username-salt2026`

### 11.4 Scripting Issues

#### "Error loading config.lua"

**Solutions:**
- Check Lua syntax errors in `config.lua`
- Ensure all referenced files exist (e.g., `modules/init.lua`)
- Use `/reload` to reload scripts after fixing errors
- Check the error message for the specific line number

#### Lua commands not working

**Solutions:**
- Ensure `void.register_command()` is called before the command is used
- Check that the function name matches the second argument to `register_command`
- Function names are case-sensitive
- Use `/debug on` to see Lua errors

### 11.5 Display Issues

#### Screen looks corrupted after resize

**Solution:** Press `Ctrl+L` or type `/repaint` to force a screen redraw.

#### Colors not showing

**Solution:** Ensure your terminal supports 256 colors. Most modern terminals do. Try:
```bash
export TERM=xterm-256color
```

#### Nick list not appearing

**Solution:** The nick list appears for channel windows. Make sure you're in a channel window (not the Status window). The NAMES reply populates the nick list.

### 11.6 Performance Issues

#### High memory usage with large scrollback

**Solution:** Reduce the scrollback limit:
```
/set SCROLLBACK 200
```

#### Slow rendering

**Solutions:**
- Reduce scrollback buffer size
- Close unused windows: `/window close`
- Disable split screen: `/window unsplit`

### 11.7 Common Error Messages

| Message | Cause | Solution |
|---|---|---|
| `-!- Not connected to server.` | Command requires active connection | Connect first: `/server host` |
| `-!- Not in a channel.` | Command requires a channel | Join a channel: `/join #channel` |
| `-!- Unknown command: /xyz` | Command not recognized | Check spelling, use `/help` |
| `-!- Cannot part the status window.` | Tried to /part the Status window | Switch to a channel window first |
| `-!- Cannot send text in Status window.` | Tried to type text in Status | Use `/join #channel` first |
| `-!- Error: Usage: /cmd <args>` | Missing required arguments | Check `/help cmd` for syntax |

---

## 12. Credits

### 12.1 Inspirations

| Project | Description | Source |
|---|---|---|
| **epic5** | IRC client that inspired Void's architecture and command set | https://github.com/epicsol/epic5 |
| **LiCe5** | Script pack for epic5 that provided the module system | https://github.com/tjbh/lice |
| **epic6** | Next-generation IRC client with modern features | https://github.com/epicsol/epic6 |

**epic5 / LiCe5 Copyright:**
Copyright (C) 1993-2000 SrfRoG, 2008-2015 tjh, whitefang  
Licensed under GPL v2+

**Features ported from epic6:**
SCRAM-SHA-512, MONITOR, `/ON CONTEXT`, `/SHH`, POLICY state, scrollback indicator, message breaking, destructive tokenizer (`void.token`), `void.coalesce`, `void.xform` (Base85), `chghost`, `account-notify`, `extended-join`

### 12.2 Rust Crates

| Crate | Version | Purpose | Link |
|---|---|---|---|
| `irc` | 1.1.0 | IRC protocol library with proxy support | https://crates.io/crates/irc |
| `ratatui` | 0.30.2 | Terminal UI framework | https://crates.io/crates/ratatui |
| `mlua` | 0.12.0 | Lua 5.4 bindings (vendored) | https://crates.io/crates/mlua |
| `ring` | 0.17 | Cryptography (SHA, HMAC, PBKDF2) | https://crates.io/crates/ring |
| `rusqlite` | 0.35 | SQLite bindings with SQLCipher | https://crates.io/crates/rusqlite |
| `tokio` | 1.53.1 | Async runtime | https://crates.io/crates/tokio |
| `crossterm` | 0.29.0 | Cross-platform terminal manipulation | https://crates.io/crates/crossterm |
| `clap` | 4.6.6 | Command-line argument parsing | https://crates.io/crates/clap |
| `chrono` | 0.4.45 | Date/time formatting | https://crates.io/crates/chrono |
| `anyhow` | 1.0.104 | Error handling | https://crates.io/crates/anyhow |
| `futures` | 0.3.34 | Async utilities | https://crates.io/crates/futures |
| `hostname` | 0.4 | System hostname detection | https://crates.io/crates/hostname |
| `shellexpand` | 3.1.2 | Shell path expansion (`~`) | https://crates.io/crates/shellexpand |
| `base64` | (custom impl) | Base64/Base85 encoding | Built-in |

### 12.3 License

Void IRC Client is released under the **MIT License**.

```
MIT License

Copyright (c) 2026 pshq

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## Appendix A: Quick Command Cheat Sheet

```
CONNECTION          CHANNELS            MESSAGES           WINDOWS
─────────────       ─────────────       ─────────────      ─────────────
/server <host>      /join <#ch>         /msg <t> <text>    /window next
/disconnect         /part [#ch]         /me <action>       /window prev
/reconnect          /topic [text]       /notice <t> <txt>  /window goto N
/quit [reason]      /names [#ch]        /say <text>        /window close
/nick <newnick>     /kick <n> [r]       /query <nick>      /window split
/away [msg]         /mode <t> <m>       /ctcp <n> <type>   /clear
/whois <nick>       /ban <n>            /ping <nick>       /lastlog [pat]
                    /op <nick>          /msay <text>       /scroll up|down
                    /voice <nick>       /mme <action>

CONFIG              SYSTEM              LiCe5 MODULES      THEME
─────────────       ─────────────       ─────────────      ─────────────
/set [var] [val]    /help [cmd]         /gone [msg]        /theme
/alias [n] [body]   /raw <text>         /back [msg]        /theme <name>
/unalias <name>     /exec <cmd>         /autoaway [sec]    /theme list
/highlight [p] [c]  /log on|off         /alarm [n] <s> <c>
/bind [key] [act]   /load <file>        /paste [send|cxl]
/format [t] [tmpl]  /save               /ns <pass> [nick]
/status [fmt]       /debug on|off       /signoff [reason]
                    /timer <s> <r> <c>  /party [on|off]
                    /notify [nick]      /sensors [en|dis]

LiCe5 (cont.)       USERLIST            LISTS              FUN
─────────────       ─────────────       ─────────────      ─────────────
/ig <pat> [fl]      /ul [add|del]       /banlist           /dance
/k <n> [r]          /ulsave             /exclist           /disco <text>
/kb <n> [r]         /ulw_op <nick>      /joinlist          /pic [name]
/rk <nick>          /ulw_voice <nick>   /floodlist         /oops <text>
/protect [#ch]      /ulw_unban <nick>   /looplist
/memo [send|chk]    /ulw_pass <pass>    /splitlist
/note [add|list]    /ulw_chat <nick>    /showlist
/wall <msg>         /ulw_help           /rmlist
/boot <n> [sec]     /ulw_whoami
```

---

## Appendix B: mIRC Color Code Reference

```
Code    Color           Code    Color
────    ─────           ────    ─────
0       White           8       Yellow
1       Black           9       Light Green
2       Blue (navy)     10      Cyan (teal)
3       Green           11      Light Cyan
4       Red             12      Light Blue
5       Brown           13      Pink (magenta)
6       Purple          14      Grey
7       Orange          15      Light Grey

Formatting codes:
\x02    Bold            \x1D    Italic
\x03    Color           \x16    Reverse
\x0F    Reset all       \x1F    Underline
```

---

## Appendix D: Variable Expansion Reference

Available in aliases, format templates, and conditional expressions:

| Variable | Expands To | Context |
|---|---|---|
| `$0` | First argument | Aliases |
| `$1` - `$9` | Nth argument | Aliases |
| `$*` | All arguments joined | Aliases |
| `$N` | Current nickname | Any |
| `$C` | Current channel | Any |
| `$S` | Current server | Any |
| `$$` | Literal `$` | Any |

**Format template variables** (for `/format`):

| Template | Variables | Default |
|---|---|---|
| `JOIN` | `$0`=nick, `$1`=channel | `* $0 has joined $1` |
| `PART` | `$0`=nick, `$1`=channel, `$2`=reason | `* $0 has left $1 ($2)` |
| `QUIT` | `$0`=nick, `$1`=reason | `* $0 has quit IRC ($1)` |
| `KICK` | `$0`=kicked, `$1`=channel, `$2`=kicker, `$3`=reason | `* $0 was kicked from $1 by $2 ($3)` |
| `NICK` | `$0`=old, `$1`=new | `* $0 is now known as $1` |
| `MODE` | `$0`=target, `$1`=modes | `* $0 sets mode: $1` |
| `TOPIC` | `$0`=nick, `$1`=topic | `* $0 set topic to: $1` |
| `MSG` | `$0`=nick, `$1`=text | `<$0> $1` |
| `ACTION` | `$0`=nick, `$1`=text | `* $0 $1` |
| `NOTICE` | `$0`=nick, `$1`=text | `-$0- $1` |

---

*Documentation generated for Void IRC Client v0.3.0*
