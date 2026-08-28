-- Catppuccin Mocha Theme — soothing warm pastel dark palette
void_themes.register("Catppuccin", {
    name = "Catppuccin",
    desc = "Soothing pastel palette (Mocha dark variant)",
    is_dark = true,
    ui = {
        status_bar_bg = "#181825",          -- Mantle
        status_bar_fg = "#a6adc8",          -- Subtext0
        status_bar_active_bg = "#89b4fa",   -- Blue
        status_bar_active_fg = "#11111b",   -- Crust (dark high contrast on blue)
        status_bar_activity_bg = "#313244", -- Surface0
        status_bar_activity_fg = "#f9e2af", -- Yellow
        status_bar_info_fg = "#bac2de",     -- Subtext1
        topic_bar_bg = "#11111b",           -- Crust
        topic_bar_fg = "#cdd6f4",           -- Text
        input_bg = "default",
        input_fg = "#cdd6f4",               -- Text
        input_prompt_fg = "#89b4fa",        -- Blue
        border = "#45475a",                 -- Surface1
        timestamp = "#6c7086",              -- Overlay0
        scroll_indicator_fg = "#11111b",
        scroll_indicator_bg = "#f9e2af",
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#cdd6f4",                 -- Text
        action = "#f9e2af",                 -- Yellow
        system = "#94e2d5",                 -- Teal
        notice = "#cba6f7",                 -- Mauve
        highlight = "#f38ba8",              -- Red
        error = "#f38ba8",                  -- Red
        server = "#9399b2",                 -- Overlay2
        ctcp = "#eba0ac",                   -- Maroon
        url = "#89b4fa",                    -- Blue
    },
    nicks = {
        op = "#f38ba8",                     -- Red (@)
        op_nick = "#f38ba8",
        voice = "#f9e2af",                  -- Yellow (+)
        voice_nick = "#f9e2af",
        halfop = "#94e2d5",                 -- Teal (%)
        halfop_nick = "#94e2d5",
        founder = "#cba6f7",                -- Mauve (~)
        founder_nick = "#cba6f7",
        admin = "#fab387",                  -- Peach (&)
        admin_nick = "#fab387",
        normal = "#cdd6f4",                 -- Text
        normal_prefix = "#6c7086",
        header = "#7f849c",
    },
    nick_colors = {
        "#f38ba8", -- Red
        "#a6e3a1", -- Green
        "#f9e2af", -- Yellow
        "#89b4fa", -- Blue
        "#cba6f7", -- Mauve
        "#94e2d5", -- Teal
        "#fab387", -- Peach
        "#89dceb", -- Sky
        "#f5c2e7", -- Pink
        "#b4befe", -- Lavender
        "#eba0ac", -- Maroon
        "#f5e0dc", -- Rosewater
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
