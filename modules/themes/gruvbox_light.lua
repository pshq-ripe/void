-- Gruvbox Light Theme — warm parchment retro light palette
void_themes.register("GruvboxLight", {
    name = "GruvboxLight",
    desc = "Retro warm parchment light color scheme",
    is_dark = false,
    ui = {
        status_bar_bg = "#ebdbb2",          -- Light bg1
        status_bar_fg = "#3c3836",          -- Dark fg1
        status_bar_active_bg = "#b57614",   -- Darker Warm Yellow
        status_bar_active_fg = "#fbf1c7",   -- Light text on yellow
        status_bar_activity_bg = "#d5c4a1", -- Light bg2
        status_bar_activity_fg = "#af3a03", -- Orange
        status_bar_info_fg = "#427b58",     -- Aqua
        topic_bar_bg = "#f2e5bc",           -- Soft parchment
        topic_bar_fg = "#282828",           -- Dark bg0
        input_bg = "default",
        input_fg = "#3c3836",               -- Fg1
        input_prompt_fg = "#b57614",        -- Yellow
        border = "#bdae93",                 -- Bg3
        timestamp = "#7c6f64",              -- Gray
        scroll_indicator_fg = "#fbf1c7",
        scroll_indicator_bg = "#b57614",
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#3c3836",                 -- Dark text
        action = "#b57614",                 -- Yellow
        system = "#427b58",                 -- Aqua
        notice = "#8f3f71",                 -- Purple
        highlight = "#af3a03",              -- Orange
        error = "#9d0006",                  -- Red
        server = "#7c6f64",                 -- Gray
        ctcp = "#af3a03",                   -- Orange
        url = "#076678",                    -- Blue
    },
    nicks = {
        op = "#9d0006",                     -- Red (@)
        op_nick = "#9d0006",
        voice = "#79740e",                  -- Green (+)
        voice_nick = "#79740e",
        halfop = "#427b58",                 -- Aqua (%)
        halfop_nick = "#427b58",
        founder = "#8f3f71",                -- Purple (~)
        founder_nick = "#8f3f71",
        admin = "#af3a03",                  -- Orange (&)
        admin_nick = "#af3a03",
        normal = "#3c3836",                 -- Dark text
        normal_prefix = "#928374",
        header = "#7c6f64",
    },
    nick_colors = {
        "#9d0006", -- Red
        "#79740e", -- Green
        "#b57614", -- Yellow
        "#076678", -- Blue
        "#8f3f71", -- Purple
        "#427b58", -- Aqua
        "#af3a03", -- Orange
        "#504945", -- Muted Dark
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
