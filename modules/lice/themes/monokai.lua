-- Monokai Pro Theme — iconic developer high-contrast vibrant palette
void_themes.register("Monokai", {
    name = "Monokai",
    desc = "Iconic developer high-contrast palette (Monokai Pro)",
    is_dark = true,
    ui = {
        status_bar_bg = "#221f22",          -- Dark Charcoal
        status_bar_fg = "#fcfcfa",          -- Light Fg
        status_bar_active_bg = "#ffd866",   -- Warm Yellow
        status_bar_active_fg = "#2d2a2e",   -- Dark bg (high contrast on yellow)
        status_bar_activity_bg = "#403e41", -- Surface
        status_bar_activity_fg = "#ff6188", -- Red/Pink
        status_bar_info_fg = "#78dce8",     -- Cyan
        topic_bar_bg = "#19181a",           -- Deep Background
        topic_bar_fg = "#fcfcfa",           -- Light text
        input_bg = "default",
        input_fg = "#fcfcfa",               -- Light text
        input_prompt_fg = "#ffd866",        -- Yellow
        border = "#403e41",                 -- Surface border
        timestamp = "#727072",              -- Gray
        scroll_indicator_fg = "#2d2a2e",
        scroll_indicator_bg = "#ffd866",
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#fcfcfa",                 -- White/Cream
        action = "#ffd866",                 -- Yellow
        system = "#78dce8",                 -- Cyan
        notice = "#ab9df2",                 -- Violet
        highlight = "#ff6188",              -- Red/Pink
        error = "#ff6188",                  -- Red
        server = "#727072",                 -- Gray
        ctcp = "#fc9867",                   -- Orange
        url = "#78dce8",                    -- Cyan
    },
    nicks = {
        op = "#ff6188",                     -- Red/Pink (@)
        op_nick = "#ff6188",
        voice = "#a9dc76",                  -- Green (+)
        voice_nick = "#a9dc76",
        halfop = "#78dce8",                 -- Cyan (%)
        halfop_nick = "#78dce8",
        founder = "#ab9df2",                -- Purple (~)
        founder_nick = "#ab9df2",
        admin = "#fc9867",                  -- Orange (&)
        admin_nick = "#fc9867",
        normal = "#fcfcfa",                 -- White
        normal_prefix = "#727072",
        header = "#727072",
    },
    nick_colors = {
        "#ff6188", -- Red/Pink
        "#a9dc76", -- Green
        "#ffd866", -- Yellow
        "#fc9867", -- Orange
        "#78dce8", -- Cyan
        "#ab9df2", -- Violet
        "#e5c07b", -- Gold
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
