-- Tokyo Night Theme — dark theme inspired by Tokyo's night lights
void_themes.register("TokyoNight", {
    name = "TokyoNight",
    desc = "Dark theme inspired by Tokyo's neon night lights",
    is_dark = true,
    ui = {
        status_bar_bg = "#16161e",          -- Night Black
        status_bar_fg = "#a9b1d6",          -- Subtext
        status_bar_active_bg = "#7aa2f7",   -- Tokyo Blue
        status_bar_active_fg = "#15161e",   -- Dark high contrast on blue
        status_bar_activity_bg = "#24283b", -- Storm surface
        status_bar_activity_fg = "#e0af68", -- Warm Yellow
        status_bar_info_fg = "#7dcfff",     -- Cyan
        topic_bar_bg = "#15161e",           -- Dark Night
        topic_bar_fg = "#c0caf5",           -- Bright text
        input_bg = "default",
        input_fg = "#c0caf5",               -- Bright text
        input_prompt_fg = "#7aa2f7",        -- Tokyo Blue
        border = "#414868",                 -- Dark border
        timestamp = "#565f89",              -- Muted comment
        scroll_indicator_fg = "#15161e",
        scroll_indicator_bg = "#e0af68",    -- Yellow
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#c0caf5",                 -- Bright text
        action = "#e0af68",                 -- Warm Yellow
        system = "#7dcfff",                 -- Cyan
        notice = "#bb9af7",                 -- Purple
        highlight = "#f7768e",              -- Red
        error = "#f7768e",                  -- Red
        server = "#565f89",                 -- Comment
        ctcp = "#ff9e64",                   -- Orange
        url = "#7aa2f7",                    -- Tokyo Blue
    },
    nicks = {
        op = "#f7768e",                     -- Red (@)
        op_nick = "#f7768e",
        voice = "#9ece6a",                  -- Green (+)
        voice_nick = "#9ece6a",
        halfop = "#7dcfff",                 -- Cyan (%)
        halfop_nick = "#7dcfff",
        founder = "#bb9af7",                -- Purple (~)
        founder_nick = "#bb9af7",
        admin = "#ff9e64",                  -- Orange (&)
        admin_nick = "#ff9e64",
        normal = "#c0caf5",                 -- Bright text
        normal_prefix = "#565f89",
        header = "#565f89",
    },
    nick_colors = {
        "#7aa2f7", -- Blue
        "#9ece6a", -- Green
        "#e0af68", -- Yellow
        "#bb9af7", -- Purple
        "#7dcfff", -- Cyan
        "#ff9e64", -- Orange
        "#f7768e", -- Red
        "#b4f9f8", -- Light Teal
        "#2ac3de", -- Deep Cyan
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
