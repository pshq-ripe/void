-- Dracula Theme
-- Dark theme with vibrant colors

void_themes.register("Dracula", {
    name = "Dracula",
    desc = "Dark theme with vibrant neon colors",
    colors = {
        bg = "#282A36",
        bg_panel = "#44475A",
        bg_element = "#6272A4",
        text = "#F8F8F2",
        text_muted = "#6272A4",
        primary = "#8BE9FD",
        secondary = "#BD93F9",
        accent = "#50FA7B",
        error = "#FF5555",
        warning = "#FFB86C",
        success = "#50FA7B",
        info = "#8BE9FD",
    },
    ui = {
        status_bar_bg = "magenta",
        status_bar_fg = "black",
        topic_bar_bg = "dark_magenta",
        topic_bar_fg = "white",
        input_fg = "light_green",
        border = "dark_gray",
        scroll_indicator = "yellow",
        timestamp = "dark_gray",
    },
    nicks = {
        op = "light_red",
        voice = "yellow",
        halfop = "cyan",
        founder = "light_magenta",
        admin = "red",
        normal = "light_green",
    },
    messages = {
        normal = "light_green",
        action = "yellow",
        system = "cyan",
        notice = "light_magenta",
        highlight = "white",
        error = "light_red",
        server = "dark_gray",
        ctcp = "red",
    },
})
