-- Cyberpunk / Synthwave Theme — 80s outrun neon magenta, cyan, and electric yellow
void_themes.register("Cyberpunk", {
    name = "Cyberpunk",
    desc = "80s retro-futuristic synthwave & cyberpunk neon",
    is_dark = true,
    ui = {
        status_bar_bg = "#120826",          -- Deep Void Purple
        status_bar_fg = "#00f0ff",          -- Electric Cyan
        status_bar_active_bg = "#ff007f",   -- Hot Neon Pink/Magenta
        status_bar_active_fg = "#000000",   -- Black (high contrast on neon pink)
        status_bar_activity_bg = "#2a164d", -- Mid Purple
        status_bar_activity_fg = "#ffe600", -- Electric Yellow
        status_bar_info_fg = "#ffe600",     -- Yellow
        topic_bar_bg = "#0a0314",           -- Darkest Abyss
        topic_bar_fg = "#f0e6ff",           -- Soft Lavender White
        input_bg = "default",
        input_fg = "#00f0ff",               -- Cyan
        input_prompt_fg = "#ff007f",        -- Hot Pink
        border = "#ff007f",                 -- Neon Pink Border
        timestamp = "#795290",              -- Muted Purple
        scroll_indicator_fg = "#000000",
        scroll_indicator_bg = "#ffe600",
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#e0d6ff",                 -- Crisp Lavender White
        action = "#ffe600",                 -- Electric Yellow
        system = "#00f0ff",                 -- Electric Cyan
        notice = "#ff007f",                 -- Hot Pink
        highlight = "#ffffff",              -- Pure Flash White
        error = "#ff003c",                  -- Laser Red
        server = "#795290",                 -- Synth Purple
        ctcp = "#ff7700",                   -- Neon Orange
        url = "#00f0ff",                    -- Electric Cyan
    },
    nicks = {
        op = "#ff007f",                     -- Hot Pink (@)
        op_nick = "#ff007f",
        voice = "#00f0ff",                  -- Cyan (+)
        voice_nick = "#00f0ff",
        halfop = "#ffe600",                 -- Electric Yellow (%)
        halfop_nick = "#ffe600",
        founder = "#bf00ff",                -- Electric Violet (~)
        founder_nick = "#bf00ff",
        admin = "#ff7700",                  -- Neon Orange (&)
        admin_nick = "#ff7700",
        normal = "#e0d6ff",                 -- Soft Lavender
        normal_prefix = "#795290",
        header = "#ff007f",
    },
    nick_colors = {
        "#ff007f", -- Hot Pink
        "#00f0ff", -- Cyan
        "#ffe600", -- Electric Yellow
        "#00ff66", -- Laser Green
        "#bf00ff", -- Violet
        "#ff7700", -- Orange
        "#00e1d9", -- Aqua
        "#ff003c", -- Laser Red
    },
})
