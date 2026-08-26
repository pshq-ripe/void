-- Catppuccin Mocha Theme
-- Soothing pastel color scheme

void_themes.register("Catppuccin", {
    name = "Catppuccin",
    desc = "Soothing pastel color scheme (Mocha variant)",
    colors = {
        bg = "#1E1E2E",
        bg_panel = "#313244",
        bg_element = "#45475A",
        text = "#CDD6F4",
        text_muted = "#585B70",
        primary = "#89B4FA",
        secondary = "#CBA6F7",
        accent = "#A6E3A1",
        error = "#F38BA8",
        warning = "#FAB387",
        success = "#A6E3A1",
        info = "#89DCEB",
    },
    ui = {
        status_bar_bg = "dark_magenta",
        status_bar_fg = "black",
        topic_bar_bg = "dark_blue",
        topic_bar_fg = "white",
        input_fg = "light_blue",
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
