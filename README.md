# Void IRC Client v0.3.0

A modern, Lua-scriptable IRC client written in Rust, inspired by **epic5** with **LiCe5** scripts and **epic6** features.

## Features

- **Full IRC protocol** — RFC2812 + irc2.11.2p3 compatibility
- **IRCv3** — CAP (NEW/DEL/LS/REQ/ACK/NAK), SASL (PLAIN/EXTERNAL/SCRAM-SHA-512/SCRAM-SHA-256/ECDSA), MONITOR, labeled-response, chathistory, server-time, away-notify, account-notify, chghost, extended-join
- **Lua scripting** — 68 API functions, 102 registered commands, 10 event hooks
- **LiCe5 compatibility** — 91 modules ported to Lua (see `modules/`)
- **16 themes** — Catppuccin (Mocha/Latte), Dracula, Nord, Gruvbox (Dark/Light), Solarized (Dark/Light), TokyoNight, Matrix, Cyberpunk, Monokai, OneDark, RosePine, Irssi, BitchX
- **Multi-server** — simultaneous connections to multiple IRC servers
- **Split screen** — vertical/horizontal split with independent scroll
- **SQLCipher** — AES-256 encrypted SQLite storage
- **TUI** — ratatui-based terminal UI with nick list, status bar, mouse support
- **90+ native commands** — epic5 + epic6 features
- **256-color support** — extended mIRC color codes (0-255)
- **Nick coloring** — hash-based consistent colors per nick
- **URL detection** — automatic highlighting of URLs in messages
- **DCC SEND** — file transfer receive
- **DCC Chat** — peer-to-peer messaging
- **DCC Resume** — resume interrupted transfers
- **SOCKS5 proxy** — connect through proxy servers
- **Auto-reconnect** — with channel rejoin tracking
- **Message breaking** — automatic word-boundary splitting at 490 bytes
- **Labeled-response** — IRCv3 message correlation
- **IRC Proxy/Bouncer** — act as a bouncer server for other clients
- **Lag meter** — real-time latency measurement
- **Raw log viewer** — inspect raw IRC protocol
- **Lastlog search** — regex search with level filtering
- **Netsplit detection** — track splits and recoveries
- **Session save/restore** — auto-join saved channels on reconnect
- **Ban list tracking** — persistent per-channel ban lists
- **Window layout persistence** — save/restore split state
- **Chatnet/network config** — group servers by IRC network
- **Notify list** — ISON polling with WHOIS verification
- **Massjoin batching** — batch rapid joins during netsplit
- **Nickmatch cache** — performance optimization for large nick lists
- **Character encoding** — per-buffer charset support (UTF-8, ISO-8859-1, etc.)
- **Format string engine** — configurable status bar and event formats

## Quick Start

```bash
# Build
cargo build --release

# Install
cp target/release/void ~/.local/bin/void

# Connect
void -c irc.spadhausen.com -n mynick -j "#mychannel"

# With SASL
void -c irc.libera.chat -n mynick --sasl nick:password

# With vhost
void -c irc.example.com -n mynick --vhost 10.0.0.1

# With proxy
void -c irc.example.com -n mynick --proxy-type socks5 --proxy-server 127.0.0.1 --proxy-port 1080
```

## CLI Options

| Flag | Description |
|------|-------------|
| `-c` | IRC server hostname |
| `-n` | Nickname |
| `-j` | Channel to auto-join |
| `-p` | Server password |
| `-P` | Port (default: 6697) |
| `-H` | Bind to vhost |
| `--no-tls` | Disable TLS |
| `-N` | NickServ password (auto-identify) |
| `--sasl` | SASL credentials (`nick:password`, `EXTERNAL`, `ecdsa:/path/to/key.pem`) |
| `--proxy-type` | Proxy type (`socks5`) |
| `--proxy-server` | Proxy hostname |
| `--proxy-port` | Proxy port |
| `--ipv6` | Force IPv6 |
| `--db-pass` | Database encryption passphrase |

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
| `banlist` | `/banlist` | Ban list management |
| `exclist` | `/exclist` | Exception list management |
| `invlist` | `/invexlist` | Invite exception list |
| `joinlist` | `/joinlist` | Join tracking / clone detection |
| `serverignore` | `/silence` | Server-level ignore |
| `play` | `/play` | Log replay |
| `chanlog` | `/chanlog` | Per-channel logging |
| `news` | `/news` | News system |
| `update` | `/update` | Update checker |
| `oops` | `/oops` | Quick fix last message |
| `splitlist` | `/splitlist` | Netsplit tracking |
| `show_list` | `/showlist` | Unified list display |
| `remove_list` | `/rmlist` | Unified list removal |
| `refriend` | `/refriend` | Quick friend management |
| `rel` | `/rel` | Relationship tracking |
| `noig` | `/noig` | No-ignore whitelist |
| `pager` | `/pager` | In-client file pager |
| `wget` | `/wget` | URL fetch |
| `trans` | `/trans` | Translation helper |
| `define` | `/define` | Dictionary lookup |
| `sc` | `/sc` | Screen/tmux integration |
| `mk` | `/mk` | File creation helper |
| `mme` | `/mme` | Mass message to targets |
| `msay` | `/msay` | Multi-target say |
| `mtog` | `/mtog` | Message toggle |
| `ctog` | `/ctog` | Channel feature toggle |
| `dtog` | `/dtog` | Display feature toggle |
| `wtog` | `/wtog` | Window feature toggle |
| `tog` | `/tog` | Generic toggle |
| `dom` | `/dom` | Domain operations |
| `dump` | `/dump` | Debug dump |
| `ul_save` | `/ulsave` | Userlist save/load |
| `ulw` | `/ulw_*` | Userlist window commands |
| `tab_comp` | `/tabcomp` | Tab completion |
| `bword` | `/bword` | Word manipulation |
| `binds` | `/binds` | Key binding management |
| `defaults` | `/defaults` | Default settings |
| `imail` | `/imail` | Internal mail system |
| `floodlist` | `/floodlist` | Flood protection exceptions |
| `looplist` | `/looplist` | Loop through lists |
| `pic` | `/pic` | ASCII art pictures |
| `ppl` | `/ppl` | People tracking |
| `chanst` | `/chanst` | Channel status |
| `cwho` | `/cwho` | Channel WHO |
| `et` | `/et` | Enhanced topic |
| `db` | `/db` | Key-value database |
| `fkeys` | `/fkey` | Function key bindings |
| `boot` | `/boot` | Boot sequence |
| `stubs` | `/adcc`, `/dcclist`, `/rdcc`, `/redcc` | DCC stubs |

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
```

### Available Functions

| Category | Functions |
|----------|----------|
| Registration | `void.register_command()`, `void.on()` |
| Display | `void.echo()`, `void.version()` |
| Messaging | `void.msg()`, `void.notice()`, `void.me()`, `void.ctcp()` |
| Channel ops | `void.join()`, `void.part()`, `void.op()`, `void.deop()`, `void.voice()`, `void.devoice()`, `void.ban()`, `void.unban()`, `void.kick()`, `void.mode()`, `void.topic()`, `void.invite()` |
| User info | `void.nick()`, `void.nick_change()`, `void.channel()`, `void.server()`, `void.connected()`, `void.whois()`, `void.away()`, `void.quit()` |
| String utils | `void.match()`, `void.strip()`, `void.length()`, `void.sub()`, `void.upper()`, `void.lower()`, `void.token()`, `void.coalesce()` |
| Crypto | `void.sha256()`, `void.sha512()`, `void.hmac_sha256()`, `void.pbkdf2()` |
| Encoding | `void.base64_encode()`, `void.base64_decode()`, `void.hex_encode()`, `void.hex_decode()`, `void.xform()` |
| File I/O | `void.file_read()`, `void.file_write()`, `void.file_append()` |
| Formatting | `void.color()`, `void.bold()`, `void.italic()`, `void.underline()`, `void.reverse()`, `void.reset()` |
| Misc | `void.random()`, `void.json_encode()`, `void.json_decode()`, `void.timer()`, `void.send()`, `void.set()`, `void.get()`, `void.ison()`, `void.userhost()`, `void.log()`, `void.load()`, `void.exec()`, `void.apply_theme()` |

## Configuration

## UI Customization

### Toggle Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `SHOW_NICKLIST` | ON | Show/hide nick list panel |
| `SHOW_STATUSBAR` | ON | Show/hide status bar |
| `SHOW_USER_COUNT` | ON | Show/hide user counts in nick list headers |
| `SHOW_TIMESTAMPS` | ON | Show/hide message timestamps |
| `MOUSE` | OFF | Enable/disable mouse capture (OFF = text selection works) |

Usage: `/set SHOW_NICKLIST OFF` to hide nick list, `/set SHOW_STATUSBAR OFF` to hide status bar.

### Dynamic Prompt

The input prompt shows context:
- Status window: `[nick]> `
- Channel: `[#channel@nick]> `
- Query: `[user@nick]> `

### Themes

16 built-in themes: `/theme list`, `/theme dracula`, `/theme random`

## Configuration

- **config.lua** — Lua configuration (loaded on startup)
- **~/.void/void.db** — SQLCipher encrypted SQLite database
- **~/.void/void.conf** — Text backup of settings

## Building

```bash
cargo build --release
cargo test --test lua_integration -- --nocapture
```

## Credits

- **epic5** — IRC client that inspired Void's architecture and command set
- **LiCe5** — Script pack for epic5 that provided the module system
- **epic6** — Next generation IRC client with modern features
- **irssi** — IRC client that inspired lag meter, raw log, netsplit, massjoin, and other features
- **irc crate** — Rust IRC protocol library
- **ratatui** — Rust TUI framework
- **mlua** — Rust Lua bindings
- **ring** — Rust cryptography library
- **rusqlite** — Rust SQLite bindings (with SQLCipher)
- **encoding_rs** — Character encoding library

## License

MIT
