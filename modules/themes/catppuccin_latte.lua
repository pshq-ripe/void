-- Catppuccin Latte Theme — soothing warm pastel light palette
void_themes.register("CatppuccinLatte", {
    name = "CatppuccinLatte",
    desc = "Soothing pastel light palette (Latte variant)",
    is_dark = false,
    ui = {
        status_bar_bg = "#e6e9ef",          -- Mantle
        status_bar_fg = "#4c4f69",          -- Text
        status_bar_active_bg = "#1e66f5",   -- Blue
        status_bar_active_fg = "#ffffff",   -- White on blue
        status_bar_activity_bg = "#ccd0da", -- Surface0
        status_bar_activity_fg = "#df8e1d", -- Yellow
        status_bar_info_fg = "#5c5f77",     -- Subtext1
        topic_bar_bg = "#dce0e8",           -- Crust
        topic_bar_fg = "#4c4f69",           -- Text
        input_bg = "default",
        input_fg = "#4c4f69",               -- Text
        input_prompt_fg = "#1e66f5",        -- Blue
        border = "#bcc0cc",                 -- Surface1
        timestamp = "#8c8fa1",              -- Overlay1
        scroll_indicator_fg = "#ffffff",
        scroll_indicator_bg = "#df8e1d",
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#4c4f69",                 -- Text
        action = "#df8e1d",                 -- Yellow
        system = "#179299",                 -- Teal
        notice = "#8839ef",                 -- Mauve
        highlight = "#d20f39",              -- Red
        error = "#d20f39",                  -- Red
        server = "#7c7f93",                 -- Overlay2
        ctcp = "#e64553",                   -- Maroon
        url = "#1e66f5",                    -- Blue
    },
    nicks = {
        op = "#d20f39",                     -- Red (@)
        op_nick = "#d20f39",
        voice = "#40a02b",                  -- Green (+)
        voice_nick = "#40a02b",
        halfop = "#179299",                 -- Teal (%)
        halfop_nick = "#179299",
        founder = "#8839ef",                -- Mauve (~)
        founder_nick = "#8839ef",
        admin = "#fe640b",                  -- Peach (&)
        admin_nick = "#fe640b",
        normal = "#4c4f69",                 -- Text
        normal_prefix = "#9ca0b0",
        header = "#7c7f93",
    },
    nick_colors = {
        "#d20f39", -- Red
        "#40a02b", -- Green
        "#df8e1d", -- Yellow
        "#1e66f5", -- Blue
        "#8839ef", -- Mauve
        "#179299", -- Teal
        "#fe640b", -- Peach
        "#04a5e5", -- Sky
        "#ea76cb", -- Pink
        "#7287fd", -- Lavender
        "#e64553", -- Maroon
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
