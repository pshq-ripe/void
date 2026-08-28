-- BitchX Retro Theme — 90s legendary bold IRC client aesthetic
void_themes.register("BitchX", {
    name = "BitchX",
    desc = "90s legendary BitchX hacker aesthetic with red & cyan accents",
    is_dark = true,
    ui = {
        status_bar_bg = "#800000",          -- Maroon / Dark Red
        status_bar_fg = "#ffffff",          -- Bold White
        status_bar_active_bg = "#ff0000",   -- Bright Red
        status_bar_active_fg = "#ffffff",   -- White on Red
        status_bar_activity_bg = "#400000", -- Deep Red
        status_bar_activity_fg = "#00ffff", -- Cyan
        status_bar_info_fg = "#00ffff",     -- Cyan
        topic_bar_bg = "#000080",           -- Navy Blue
        topic_bar_fg = "#00ffff",           -- Cyan
        input_bg = "default",
        input_fg = "#ffffff",               -- White
        input_prompt_fg = "#ff0000",        -- Red
        border = "#008080",                 -- Teal
        timestamp = "#808080",              -- Gray
        scroll_indicator_fg = "#ffffff",
        scroll_indicator_bg = "#ff0000",
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#ffffff",                 -- White
        action = "#00ffff",                 -- Cyan
        system = "#00ff00",                 -- Green
        notice = "#ff00ff",                 -- Magenta
        highlight = "#ff0000",              -- Red
        error = "#ff0000",                  -- Red
        server = "#808080",                 -- Gray
        ctcp = "#ff0000",                   -- Red
        url = "#00ffff",                    -- Cyan
    },
    nicks = {
        op = "#ff0000",                     -- Red (@)
        op_nick = "#00ff00",                -- Green
        voice = "#00ffff",                  -- Cyan (+)
        voice_nick = "#00ff00",             -- Green
        halfop = "#ffff00",                 -- Yellow (%)
        halfop_nick = "#00ff00",            -- Green
        founder = "#ff00ff",                -- Magenta (~)
        founder_nick = "#00ff00",
        admin = "#ff8000",                  -- Orange (&)
        admin_nick = "#00ff00",
        normal = "#ffffff",                 -- White
        normal_prefix = "#808080",
        header = "#00ffff",
    },
    nick_colors = {
        "#ff0000", -- Red
        "#00ff00", -- Green
        "#00ffff", -- Cyan
        "#ffff00", -- Yellow
        "#ff00ff", -- Magenta
        "#ffffff", -- White
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
