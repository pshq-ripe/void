-- Solarized Light Theme — precision cream palette by Ethan Schoonover
void_themes.register("SolarizedLight", {
    name = "SolarizedLight",
    desc = "Precision low-contrast cream light palette",
    is_dark = false,
    ui = {
        status_bar_bg = "#eee8d5",          -- Base2
        status_bar_fg = "#586e75",          -- Base01
        status_bar_active_bg = "#268bd2",   -- Blue
        status_bar_active_fg = "#fdf6e3",   -- Base3 (light on blue)
        status_bar_activity_bg = "#e0d9c4", -- Darker cream
        status_bar_activity_fg = "#cb4b16", -- Orange
        status_bar_info_fg = "#2aa198",     -- Cyan
        topic_bar_bg = "#fdf6e3",           -- Base3
        topic_bar_fg = "#073642",           -- Base02
        input_bg = "default",
        input_fg = "#657b83",               -- Base00
        input_prompt_fg = "#268bd2",        -- Blue
        border = "#93a1a1",                 -- Base1
        timestamp = "#93a1a1",              -- Base1
        scroll_indicator_fg = "#fdf6e3",
        scroll_indicator_bg = "#b58900",    -- Yellow
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#657b83",                 -- Base00
        action = "#b58900",                 -- Yellow
        system = "#2aa198",                 -- Cyan
        notice = "#6c71c4",                 -- Violet
        highlight = "#cb4b16",              -- Orange
        error = "#dc322f",                  -- Red
        server = "#93a1a1",                 -- Base1
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
        normal = "#657b83",                 -- Base00
        normal_prefix = "#93a1a1",
        header = "#93a1a1",
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
})
