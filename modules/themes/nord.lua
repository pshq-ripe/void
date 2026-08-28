-- Nord Theme — arctic north-bluish clean and elegant color palette
void_themes.register("Nord", {
    name = "Nord",
    desc = "Arctic north-bluish clean aesthetic",
    is_dark = true,
    ui = {
        status_bar_bg = "#2e3440",          -- Nord0 (Polar Night)
        status_bar_fg = "#d8dee9",          -- Nord4 (Snow Storm)
        status_bar_active_bg = "#88c0d0",   -- Nord8 (Frost Ice Blue)
        status_bar_active_fg = "#2e3440",   -- Nord0 (Dark high contrast on ice blue)
        status_bar_activity_bg = "#434c5e", -- Nord2
        status_bar_activity_fg = "#ebcb8b", -- Nord13 (Yellow)
        status_bar_info_fg = "#8fbcbb",     -- Nord7 (Frost Teal)
        topic_bar_bg = "#242933",           -- Deep Polar Night
        topic_bar_fg = "#eceff4",           -- Nord6 (Bright Snow Storm)
        input_bg = "default",
        input_fg = "#eceff4",               -- Nord6
        input_prompt_fg = "#88c0d0",        -- Nord8
        border = "#4c566a",                 -- Nord3
        timestamp = "#4c566a",              -- Nord3
        scroll_indicator_fg = "#2e3440",
        scroll_indicator_bg = "#ebcb8b",    -- Nord13
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#d8dee9",                 -- Nord4
        action = "#ebcb8b",                 -- Nord13 (Yellow)
        system = "#88c0d0",                 -- Nord8 (Ice Blue)
        notice = "#b48ead",                 -- Nord15 (Purple)
        highlight = "#81a1c1",              -- Nord9
        error = "#bf616a",                  -- Nord11 (Red)
        server = "#4c566a",                 -- Nord3
        ctcp = "#d08770",                   -- Nord12 (Orange)
        url = "#88c0d0",                    -- Nord8
    },
    nicks = {
        op = "#bf616a",                     -- Nord11 (Red @)
        op_nick = "#bf616a",
        voice = "#a3be8c",                  -- Nord14 (Green +)
        voice_nick = "#a3be8c",
        halfop = "#8fbcbb",                 -- Nord7 (Teal %)
        halfop_nick = "#8fbcbb",
        founder = "#b48ead",                -- Nord15 (Purple ~)
        founder_nick = "#b48ead",
        admin = "#d08770",                  -- Nord12 (Orange &)
        admin_nick = "#d08770",
        normal = "#d8dee9",                 -- Nord4
        normal_prefix = "#4c566a",
        header = "#4c566a",
    },
    nick_colors = {
        "#88c0d0", -- Frost Ice Blue
        "#a3be8c", -- Aurora Green
        "#ebcb8b", -- Aurora Yellow
        "#81a1c1", -- Frost Glacier Blue
        "#b48ead", -- Aurora Purple
        "#8fbcbb", -- Frost Teal
        "#d08770", -- Aurora Orange
        "#bf616a", -- Aurora Red
        "#e5e9f0", -- Snow Storm
        "#5e81ac", -- Deep Frost
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
