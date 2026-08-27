-- Dracula Theme — famous vibrant vampire dark theme with neon accents
void_themes.register("Dracula", {
    name = "Dracula",
    desc = "Famous dark theme with neon vampire accents",
    is_dark = true,
    ui = {
        status_bar_bg = "#21222c",          -- Darker background
        status_bar_fg = "#f8f8f2",          -- Foreground
        status_bar_active_bg = "#bd93f9",   -- Purple
        status_bar_active_fg = "#282a36",   -- Dark background (high contrast on purple)
        status_bar_activity_bg = "#44475a", -- Selection / Current Line
        status_bar_activity_fg = "#50fa7b", -- Green
        status_bar_info_fg = "#8be9fd",     -- Cyan
        topic_bar_bg = "#1e1f29",           -- Deep background
        topic_bar_fg = "#f8f8f2",           -- Foreground
        input_bg = "default",
        input_fg = "#f8f8f2",               -- Foreground
        input_prompt_fg = "#ff79c6",        -- Pink
        border = "#6272a4",                 -- Comment
        timestamp = "#6272a4",              -- Comment
        scroll_indicator_fg = "#282a36",
        scroll_indicator_bg = "#f1fa8c",    -- Yellow
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#f8f8f2",                 -- Foreground
        action = "#f1fa8c",                 -- Yellow
        system = "#8be9fd",                 -- Cyan
        notice = "#bd93f9",                 -- Purple
        highlight = "#ff79c6",              -- Pink
        error = "#ff5555",                  -- Red
        server = "#6272a4",                 -- Comment
        ctcp = "#ffb86c",                   -- Orange
        url = "#8be9fd",                    -- Cyan
    },
    nicks = {
        op = "#ff5555",                     -- Red (@)
        op_nick = "#ff5555",
        voice = "#50fa7b",                  -- Green (+)
        voice_nick = "#50fa7b",
        halfop = "#8be9fd",                 -- Cyan (%)
        halfop_nick = "#8be9fd",
        founder = "#ff79c6",                -- Pink (~)
        founder_nick = "#ff79c6",
        admin = "#ffb86c",                  -- Orange (&)
        admin_nick = "#ffb86c",
        normal = "#f8f8f2",                 -- Foreground
        normal_prefix = "#6272a4",
        header = "#6272a4",
    },
    nick_colors = {
        "#ff5555", -- Red
        "#50fa7b", -- Green
        "#f1fa8c", -- Yellow
        "#bd93f9", -- Purple
        "#ff79c6", -- Pink
        "#8be9fd", -- Cyan
        "#ffb86c", -- Orange
        "#e9f284", -- Lime
        "#d6acff", -- Light Purple
        "#a4ffff", -- Light Cyan
    },
})
