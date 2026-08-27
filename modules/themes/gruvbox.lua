-- Gruvbox Dark Theme — retro groove warm earth colors
void_themes.register("Gruvbox", {
    name = "Gruvbox",
    desc = "Retro groove warm dark color scheme",
    is_dark = true,
    ui = {
        status_bar_bg = "#3c3836",          -- Bg1
        status_bar_fg = "#ebdbb2",          -- Fg1
        status_bar_active_bg = "#fabd2f",   -- Bright Yellow
        status_bar_active_fg = "#282828",   -- Dark bg0 (high contrast on yellow)
        status_bar_activity_bg = "#504945", -- Bg2
        status_bar_activity_fg = "#fe8019", -- Orange
        status_bar_info_fg = "#8ec07c",     -- Aqua
        topic_bar_bg = "#1d2021",           -- Hard Dark Bg
        topic_bar_fg = "#ebdbb2",           -- Fg1
        input_bg = "default",
        input_fg = "#ebdbb2",               -- Fg1
        input_prompt_fg = "#fabd2f",        -- Yellow
        border = "#665c54",                 -- Bg3
        timestamp = "#928374",              -- Gray
        scroll_indicator_fg = "#282828",
        scroll_indicator_bg = "#fabd2f",    -- Yellow
        chat_bg = "default",
        nick_list_bg = "default",
    },
    messages = {
        normal = "#ebdbb2",                 -- Fg1
        action = "#fabd2f",                 -- Yellow
        system = "#8ec07c",                 -- Aqua
        notice = "#d3869b",                 -- Purple
        highlight = "#fe8019",              -- Orange
        error = "#fb4934",                  -- Red
        server = "#928374",                 -- Gray
        ctcp = "#fe8019",                   -- Orange
        url = "#83a598",                    -- Blue
    },
    nicks = {
        op = "#fb4934",                     -- Red (@)
        op_nick = "#fb4934",
        voice = "#b8bb26",                  -- Green (+)
        voice_nick = "#b8bb26",
        halfop = "#8ec07c",                 -- Aqua (%)
        halfop_nick = "#8ec07c",
        founder = "#d3869b",                -- Purple (~)
        founder_nick = "#d3869b",
        admin = "#fe8019",                  -- Orange (&)
        admin_nick = "#fe8019",
        normal = "#ebdbb2",                 -- Fg1
        normal_prefix = "#928374",
        header = "#928374",
    },
    nick_colors = {
        "#fb4934", -- Red
        "#b8bb26", -- Green
        "#fabd2f", -- Yellow
        "#83a598", -- Blue
        "#d3869b", -- Purple
        "#8ec07c", -- Aqua
        "#fe8019", -- Orange
        "#d5c4a1", -- Warm Light Fg
        "#bdae93", -- Muted Sand
    },
})
