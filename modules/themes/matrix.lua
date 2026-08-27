-- Matrix Theme — green phosphor hacker terminal aesthetic
void_themes.register("Matrix", {
    name = "Matrix",
    desc = "Green phosphor CRT hacker aesthetic",
    is_dark = true,
    ui = {
        status_bar_bg = "#002008",          -- Deep Forest CRT
        status_bar_fg = "#00bb2d",          -- Matrix Medium Green
        status_bar_active_bg = "#00ff41",   -- Phosphor Neon Green
        status_bar_active_fg = "#001504",   -- Deep Black (high contrast on neon green)
        status_bar_activity_bg = "#003b10", -- Matrix Dark
        status_bar_activity_fg = "#5cff77", -- Bright Green
        status_bar_info_fg = "#5cff77",     -- Bright Green
        topic_bar_bg = "#001004",           -- Pitch Black CRT
        topic_bar_fg = "#5cff77",           -- Phosphor Green
        input_bg = "default",
        input_fg = "#5cff77",               -- Phosphor Green
        input_prompt_fg = "#00ff41",        -- Neon Green
        border = "#004d14",                 -- CRT Border
        timestamp = "#006b1d",              -- Dim Matrix Green
        scroll_indicator_fg = "#001504",
        scroll_indicator_bg = "#00ff41",
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#38ef7d",                 -- Clean legible phosphor green
        action = "#a8ff78",                 -- Light Lime
        system = "#00e5ff",                 -- Digital Cyan
        notice = "#78ffd6",                 -- Mint Neon
        highlight = "#ffffff",              -- Crisp White Flash
        error = "#ff3344",                  -- Digital Red Alert
        server = "#008f11",                 -- Deep Matrix Green
        ctcp = "#ff5252",                   -- Red CTCP
        url = "#00e5ff",                    -- Digital Cyan
    },
    nicks = {
        op = "#00ff41",                     -- Neon Green (@)
        op_nick = "#5cff77",
        voice = "#a8ff78",                  -- Lime Green (+)
        voice_nick = "#a8ff78",
        halfop = "#00e5ff",                 -- Cyan (%)
        halfop_nick = "#00e5ff",
        founder = "#78ffd6",                -- Mint (~)
        founder_nick = "#78ffd6",
        admin = "#ffe600",                  -- Digital Gold (&)
        admin_nick = "#ffe600",
        normal = "#38ef7d",                 -- Phosphor Green
        normal_prefix = "#006b1d",
        header = "#008f11",
    },
    nick_colors = {
        "#00ff41", -- Neon Green
        "#38ef7d", -- Phosphor Green
        "#a8ff78", -- Lime
        "#78ffd6", -- Mint
        "#00e5ff", -- Digital Cyan
        "#5cff77", -- Bright Green
        "#bbfcc8", -- White Green
        "#00d26a", -- Emerald
    },
})
