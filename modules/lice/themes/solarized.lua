-- Solarized Dark Theme — precision colors by Ethan Schoonover
void_themes.register("Solarized", {
    name = "Solarized",
    desc = "Precision color palette (Solarized Dark)",
    is_dark = true,
    ui = {
        status_bar_bg = "#073642",          -- Base02
        status_bar_fg = "#93a1a1",          -- Base1
        status_bar_active_bg = "#2aa198",   -- Cyan
        status_bar_active_fg = "#002b36",   -- Base03 (dark high contrast on cyan)
        status_bar_activity_bg = "#002b36", -- Base03
        status_bar_activity_fg = "#b58900", -- Yellow
        status_bar_info_fg = "#268bd2",     -- Blue
        topic_bar_bg = "#002b36",           -- Base03
        topic_bar_fg = "#93a1a1",           -- Base1
        input_bg = "default",
        input_fg = "#839496",               -- Base0
        input_prompt_fg = "#2aa198",        -- Cyan
        border = "#586e75",                 -- Base01
        timestamp = "#586e75",              -- Base01
        scroll_indicator_fg = "#002b36",
        scroll_indicator_bg = "#b58900",    -- Yellow
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#839496",                 -- Base0
        action = "#b58900",                 -- Yellow
        system = "#2aa198",                 -- Cyan
        notice = "#6c71c4",                 -- Violet
        highlight = "#cb4b16",              -- Orange
        error = "#dc322f",                  -- Red
        server = "#586e75",                 -- Base01
        ctcp = "#d33682",                   -- Magenta
        url = "#268bd2",                    -- Blue
    },
    nicks = {
        op = "#dc322f",                     -- Red (@)
        op_nick = "#dc322f",
        voice = "#859900",                  -- Green (+)
        voice_nick = "#859900",
        halfop = "#2aa198",                 -- Cyan (%)
        halfop_nick = "#2aa198",
        founder = "#d33682",                -- Magenta (~)
        founder_nick = "#d33682",
        admin = "#cb4b16",                  -- Orange (&)
        admin_nick = "#cb4b16",
        normal = "#839496",                 -- Base0
        normal_prefix = "#586e75",
        header = "#586e75",
    },
    nick_colors = {
        "#268bd2", -- Blue
        "#2aa198", -- Cyan
        "#859900", -- Green
        "#b58900", -- Yellow
        "#cb4b16", -- Orange
        "#dc322f", -- Red
        "#d33682", -- Magenta
        "#6c71c4", -- Violet
    },
    formats = {
        status_format = "%T %N%# %@%C%+ %W %A %H%B %F %Q%M",
        input_prompt = "> ",
        join = "* $0 has joined $1",
        part = "* $0 has left $1 ($2)",
        quit = "* $0 has quit IRC ($1)",
        kick = "* $0 was kicked from $1 by $2 ($3)",
        nick = "* $0 is now known as $1",
        mode = "* $0 sets mode: $1",
        topic = "* $0 set topic to: $1",
        msg = "<$0> $1",
        notice = "-$0- $1",
        action = "* $0 $1",
        public = "<$0> $1",
    },
    ctcp = {
        version = "Void IRC Client v0.3.0 (Rust)",
        userinfo = "Void IRC Client",
        source = "https://github.com/pshq-ripe/void",
    },
    reasons = {
        kick = "Requested",
        part = "Leaving",
        quit = "Leaving",
    },
})
