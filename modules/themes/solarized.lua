-- Solarized Dark Theme
-- Precision colors for machines and people

void_themes.register("Solarized", {
    name = "Solarized",
    desc = "Precision colors for machines and people",
    colors = {
        bg = "#002B36",
        bg_panel = "#073642",
        bg_element = "#586E75",
        text = "#839496",
        text_muted = "#586E75",
        primary = "#268BD2",
        secondary = "#2AA198",
        accent = "#B58900",
        error = "#DC322F",
        warning = "#CB4B16",
        success = "#859900",
        info = "#268BD2",
    },
    ui = {
        status_bar_bg = "dark_cyan",
        status_bar_fg = "white",
        topic_bar_bg = "dark_blue",
        topic_bar_fg = "white",
        input_fg = "cyan",
        border = "dark_gray",
        scroll_indicator = "yellow",
        timestamp = "dark_gray",
    },
    nicks = {
        op = "red",
        voice = "yellow",
        halfop = "cyan",
        founder = "magenta",
        admin = "red",
        normal = "green",
    },
    messages = {
        normal = "green",
        action = "yellow",
        system = "cyan",
        notice = "magenta",
        highlight = "white",
        error = "light_red",
        server = "dark_gray",
        ctcp = "red",
    },
})
