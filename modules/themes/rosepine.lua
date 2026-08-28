-- Rosé Pine Theme — Soho vibes, muted rose, pine, and gold
void_themes.register("RosePine", {
    name = "RosePine",
    desc = "Soho vibes with muted rose, pine, and gold",
    is_dark = true,
    ui = {
        status_bar_bg = "#191724",          -- Base
        status_bar_fg = "#e0def4",          -- Text
        status_bar_active_bg = "#eb6f92",   -- Love (Rose)
        status_bar_active_fg = "#191724",   -- Base (high contrast on rose)
        status_bar_activity_bg = "#26233a", -- Surface
        status_bar_activity_fg = "#f6c177", -- Gold
        status_bar_info_fg = "#9ccfd8",     -- Foam
        topic_bar_bg = "#12101b",           -- Crust
        topic_bar_fg = "#e0def4",           -- Text
        input_bg = "default",
        input_fg = "#e0def4",               -- Text
        input_prompt_fg = "#ebbcba",        -- Rose
        border = "#403d52",                 -- Highlight High
        timestamp = "#6e6a86",              -- Muted
        scroll_indicator_fg = "#191724",
        scroll_indicator_bg = "#f6c177",
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#e0def4",                 -- Text
        action = "#f6c177",                 -- Gold
        system = "#9ccfd8",                 -- Foam
        notice = "#c4a7e7",                 -- Iris
        highlight = "#eb6f92",              -- Love
        error = "#eb6f92",                  -- Love
        server = "#6e6a86",                 -- Muted
        ctcp = "#ebbcba",                   -- Rose
        url = "#31748f",                    -- Pine
    },
    nicks = {
        op = "#eb6f92",                     -- Love (@)
        op_nick = "#eb6f92",
        voice = "#31748f",                  -- Pine (+)
        voice_nick = "#31748f",
        halfop = "#9ccfd8",                 -- Foam (%)
        halfop_nick = "#9ccfd8",
        founder = "#c4a7e7",                -- Iris (~)
        founder_nick = "#c4a7e7",
        admin = "#f6c177",                  -- Gold (&)
        admin_nick = "#f6c177",
        normal = "#e0def4",                 -- Text
        normal_prefix = "#6e6a86",
        header = "#6e6a86",
    },
    nick_colors = {
        "#eb6f92", -- Love (Rose)
        "#31748f", -- Pine
        "#f6c177", -- Gold
        "#c4a7e7", -- Iris
        "#9ccfd8", -- Foam
        "#ebbcba", -- Rose
        "#ea9a97", -- Light Rose
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
