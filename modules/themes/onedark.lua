-- One Dark Theme — Atom's classic iconic dark palette
void_themes.register("OneDark", {
    name = "OneDark",
    desc = "Atom's classic balanced dark theme",
    is_dark = true,
    ui = {
        status_bar_bg = "#21252b",          -- Darker Surface
        status_bar_fg = "#abb2bf",          -- Foreground
        status_bar_active_bg = "#61afef",   -- Blue
        status_bar_active_fg = "#282c34",   -- Dark bg (high contrast on blue)
        status_bar_activity_bg = "#2c313a", -- Selection
        status_bar_activity_fg = "#e5c07b", -- Yellow
        status_bar_info_fg = "#56b6c2",     -- Cyan
        topic_bar_bg = "#1e2227",           -- Deep Background
        topic_bar_fg = "#abb2bf",           -- Foreground
        input_bg = "default",
        input_fg = "#abb2bf",               -- Foreground
        input_prompt_fg = "#61afef",        -- Blue
        border = "#3e4451",                 -- Border
        timestamp = "#5c6370",              -- Comments
        scroll_indicator_fg = "#282c34",
        scroll_indicator_bg = "#e5c07b",
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#abb2bf",                 -- Foreground
        action = "#e5c07b",                 -- Yellow
        system = "#56b6c2",                 -- Cyan
        notice = "#c678dd",                 -- Purple
        highlight = "#e06c75",              -- Red
        error = "#e06c75",                  -- Red
        server = "#5c6370",                 -- Comments
        ctcp = "#d19a66",                   -- Orange
        url = "#61afef",                    -- Blue
    },
    nicks = {
        op = "#e06c75",                     -- Red (@)
        op_nick = "#e06c75",
        voice = "#98c379",                  -- Green (+)
        voice_nick = "#98c379",
        halfop = "#56b6c2",                 -- Cyan (%)
        halfop_nick = "#56b6c2",
        founder = "#c678dd",                -- Purple (~)
        founder_nick = "#c678dd",
        admin = "#d19a66",                  -- Orange (&)
        admin_nick = "#d19a66",
        normal = "#abb2bf",                 -- Foreground
        normal_prefix = "#5c6370",
        header = "#5c6370",
    },
    nick_colors = {
        "#61afef", -- Blue
        "#98c379", -- Green
        "#e5c07b", -- Yellow
        "#c678dd", -- Purple
        "#56b6c2", -- Cyan
        "#d19a66", -- Orange
        "#e06c75", -- Red
        "#be5046", -- Dark Red
    },
})
