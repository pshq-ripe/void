-- Irssi Classic Theme — nostalgic deep blue statusbar with green/white accents
void_themes.register("Irssi", {
    name = "Irssi",
    desc = "Nostalgic Irssi-style classic IRC theme with blue statusbar",
    is_dark = true,
    ui = {
        status_bar_bg = "#0000aa",          -- Classic Irssi Deep Blue
        status_bar_fg = "#ffffff",          -- White
        status_bar_active_bg = "#00aaaa",   -- Cyan Active Tab
        status_bar_active_fg = "#000000",   -- Black on Cyan
        status_bar_activity_bg = "#000055", -- Darker Blue
        status_bar_activity_fg = "#ffff55", -- Bright Yellow
        status_bar_info_fg = "#55ffff",     -- Light Cyan
        topic_bar_bg = "#000080",           -- Navy
        topic_bar_fg = "#ffffff",           -- White
        input_bg = "default",
        input_fg = "#ffffff",               -- White
        input_prompt_fg = "#55ff55",        -- Bright Green
        border = "#555555",                 -- Dark Gray
        timestamp = "#aaaaaa",              -- Gray
        scroll_indicator_fg = "#000000",
        scroll_indicator_bg = "#ffff55",
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#ffffff",                 -- White
        action = "#ffff55",                 -- Yellow
        system = "#55ffff",                 -- Cyan
        notice = "#ff55ff",                 -- Magenta
        highlight = "#ff5555",              -- Bright Red
        error = "#ff5555",                  -- Bright Red
        server = "#aaaaaa",                 -- Gray
        ctcp = "#ff5555",                   -- Red
        url = "#55ffff",                    -- Cyan
    },
    nicks = {
        op = "#ff5555",                     -- Red (@)
        op_nick = "#55ff55",
        voice = "#ffff55",                  -- Yellow (+)
        voice_nick = "#55ff55",
        halfop = "#55ffff",                 -- Cyan (%)
        halfop_nick = "#55ff55",
        founder = "#ff55ff",                -- Magenta (~)
        founder_nick = "#55ff55",
        admin = "#ff5555",                  -- Red (&)
        admin_nick = "#55ff55",
        normal = "#ffffff",                 -- White
        normal_prefix = "#555555",
        header = "#55ffff",
    },
    nick_colors = {
        "#55ff55", -- Light Green
        "#55ffff", -- Light Cyan
        "#ffff55", -- Light Yellow
        "#ff55ff", -- Light Magenta
        "#ff5555", -- Light Red
        "#5555ff", -- Light Blue
        "#ffffff", -- White
    },
})
