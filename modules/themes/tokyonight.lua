-- Tokyo Night Theme
-- Dark theme inspired by Tokyo's night lights

void_themes.register("TokyoNight", {
    name = "TokyoNight",
    desc = "Dark theme inspired by Tokyo's night lights",
    colors = {
        bg = "#1A1B26",
        bg_panel = "#24283B",
        bg_element = "#414868",
        text = "#C0CAF5",
        text_muted = "#565F89",
        primary = "#7AA2F7",
        secondary = "#BB9AF7",
        accent = "#9ECE6A",
        error = "#F7768E",
        warning = "#E0AF68",
        success = "#9ECE6A",
        info = "#7DCFFF",
    },
    ui = {
        status_bar_bg = "dark_blue",
        status_bar_fg = "white",
        topic_bar_bg = "dark_magenta",
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
