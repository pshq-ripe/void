# Void IRC Client

A modern, Lua-scriptable IRC client written in Rust, inspired by **epic5** with **LiCe5** scripts.

## Features

- **Full IRC protocol** — RFC2812 + irc2.11.2p3 compatibility
- **IRCv3** — CAP negotiation, SASL (PLAIN/EXTERNAL/SCRAM-SHA-512), MONITOR, away-notify
- **Lua scripting** — 40+ API functions, event hooks, custom commands
- **LiCe5 compatibility** — 25 modules ported to Lua (see `modules/`)
- **Multi-server** — simultaneous connections to multiple IRC servers
- **Split screen** — view two buffers at once with independent scroll
- **SQLCipher** — AES-256 encrypted SQLite storage for settings, aliases, highlights
- **TUI** — ratatui-based terminal UI with nick list, status bar, mouse support
- **60+ commands** — epic5 + epic6 features
- **mIRC formatting** — color codes, bold, italic, underline, reverse
- **Nick coloring** — hash-based consistent colors per nick
- **URL detection** — automatic highlighting of URLs in messages
- **DCC SEND** — file transfer receive
- **SOCKS5 proxy** — connect through proxy servers
- **Auto-reconnect** — with channel rejoin tracking

## Quick Start

```bash
cargo build --release
./target/release/void -c irc.example.com -n mynick -j "#mychannel"
```

## CLI Options

| Flag | Description |
|------|-------------|
| `-c` | IRC server hostname |
| `-n` | Nickname |
| `-j` | Channel to auto-join |
| `-p` | Server password |
| `-P` | Port (default: 6697) |
| `--no-tls` | Disable TLS |
| `--nickserv` | NickServ password (auto-identify) |
| `--sasl` | SASL credentials (`nick:password` or `EXTERNAL`) |
| `--proxy-type` | Proxy type (`socks5`) |
| `--proxy-server` | Proxy hostname |
| `--proxy-port` | Proxy port |
| `--db-pass` | Database encryption passphrase |
| `--ipv6` | Force IPv6 |

## Modules (LiCe5 Compatibility)

Load all modules: `/load modules/init.lua`

| Module | Commands | Description |
|--------|----------|-------------|
| `ignore` | `/ig`, `/ignore` | Enhanced ignore with patterns, timeouts |
| `gone` | `/gone`, `/back`, `/autoaway` | Away system with random reasons |
| `kick` | `/k`, `/kb`, `/rk` | Enhanced kick/kickban with random reasons |
| `mass` | `/massop`, `/massdeop`, `/massvoice` | Mass mode commands |
| `userlist` | `/ul`, `/userlist` | Bot-style auto-op/voice |
| `alarm` | `/alarm` | Timer/reminder system |
| `reconnect` | `/reconnect`, `/rejoin` | Auto-reconnect with channel rejoin |
| `paste` | `/paste` | Multi-line paste mode |
| `logman` | `/logman` | Per-channel log management |
| `autovoice` | `/autovoice` | Auto-voice on join |
| `anti_flood` | `/antiflood` | Anti-flood protection |
| `highlight` | `/lice_highlight` | Nick/pattern highlight |
| `ctcp` | (hooks) | Enhanced CTCP replies |
| `nickserv` | `/ns`, `/nickserv` | NickServ auto-identify + ghost |
| `channel_protect` | `/protect` | Anti-kick, anti-deop |
| `invite` | `/invlist` | Invite tracking |
| `dns` | `/dns` | DNS lookup |
| `signoff` | `/signoff` | Random quit messages |
| `wall` | `/wall` | Broadcast to channels |
| `finger` | `/finger` | User info lookup |
| `memo` | `/memo` | Offline memo system |
| `note` | `/note` | Quick notes |
| `party` | `/party`, `/disco`, `/dance` | Party mode with disco colors |
| `sensors` | `/sensors` | Channel activity monitoring |
| `help` | `/lice_help` | Enhanced help system |

## Lua API

```lua
-- Register a command
void.register_command("hello", "my_hello")
function my_hello(args)
    void.echo("Hello, " .. (args[1] or "world") .. "!")
end

-- Hook into IRC events
void.on("JOIN", "on_join")
function on_join(args)
    local nick = args[1]
    local channel = args[2]
    void.echo(nick .. " joined " .. channel)
end

-- Available functions:
-- void.echo(text)          -- Display text
-- void.msg(target, text)   -- Send private message
-- void.notice(target, text)-- Send notice
-- void.join(channel)       -- Join channel
-- void.part(channel)       -- Leave channel
-- void.op(channel, nick)   -- Give operator
-- void.voice(channel, nick)-- Give voice
-- void.ban(channel, mask)  -- Ban user
-- void.kick(channel, nick, reason) -- Kick user
-- void.mode(channel, modes)-- Set modes
-- void.topic(channel, text)-- Set topic
-- void.whois(nick)         -- WHOIS lookup
-- void.nick()              -- Get current nick
-- void.channel()           -- Get current channel
-- void.server()            -- Get current server
-- void.connected()         -- Check connection status
-- void.match(pattern, text)-- Pattern matching
-- void.strip(text)         -- Remove IRC formatting
-- void.sha256(text)        -- SHA-256 hash
-- void.sha512(text)        -- SHA-512 hash
-- void.hmac_sha256(key, text) -- HMAC-SHA-256
-- void.pbkdf2(pass, salt, iter) -- PBKDF2 key derivation
-- void.base64_encode(text) -- Base64 encode
-- void.base64_decode(text) -- Base64 decode
-- void.xform("+B85", text) -- Base85 encode
-- void.xform("-B85", text) -- Base85 decode
-- void.xform("+URL", text) -- URL encode
-- void.xform("-URL", text) -- URL decode
-- void.token(text, delim)  -- Destructive string tokenizer
-- void.coalesce(...)       -- First non-empty argument
-- void.random(min, max)    -- Random number
-- void.file_read(path)     -- Read file
-- void.file_write(path, content) -- Write file
-- void.file_append(path, content) -- Append to file
-- void.timer(seconds, fn)  -- Timer
-- void.send(raw)           -- Send raw IRC
-- void.quit(reason)        -- Quit
```

## Configuration

- **config.lua** — Lua configuration (loaded on startup)
- **~/.void/void.db** — SQLCipher encrypted SQLite database
- **~/.void/void.conf** — Text backup of settings

## Building

```bash
# Development
cargo build

# Release (optimized)
cargo build --release

# Run tests
cargo test --test lua_integration -- --nocapture
```

## Credits

- **epic5** — IRC client that inspired Void's architecture and command set
  - Source: https://github.com/epicsol/epic5
  - Copyright (C) 1993-2000 SrfRoG, 2008-2015 tjh, whitefang
- **LiCe5** — Script pack for epic5 that provided the module system
  - Source: https://github.com/tjbh/lice
  - Copyright (C) 1993-2000 SrfRoG, 2008-2015 tjh, whitefang
  - Licensed under GPL v2+
- **epic6** — Next generation IRC client with modern features
  - Source: https://github.com/epicsol/epic6
  - Features ported: SCRAM-SHA-512, MONITOR, /ON CONTEXT, /SHH, POLICY state, scrollback indicator, message breaking
- **irc crate** — Rust IRC protocol library
  - https://crates.io/crates/irc
- **ratatui** — Rust TUI framework
  - https://crates.io/crates/ratatui
- **mlua** — Rust Lua bindings
  - https://crates.io/crates/mlua
- **ring** — Rust cryptography library
  - https://crates.io/crates/ring
- **rusqlite** — Rust SQLite bindings (with SQLCipher)
  - https://crates.io/crates/rusqlite

## License

MIT
