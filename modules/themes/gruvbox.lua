-- Gruvbox Theme
-- Retro groove warm color scheme

void_themes.register("Gruvbox", {
    name = "Gruvbox",
    desc = "Retro groove warm color scheme",
    colors = {
        bg = "#282828",
        bg_panel = "#3C3836",
        bg_element = "#504945",
        text = "#EBDBB2",
        text_muted = "#928374",
        primary = "#83A598",
        secondary = "#D3869B",
        accent = "#B8BB26",
        error = "#FB4934",
        warning = "#FABD2F",
        success = "#B8BB26",
        info = "#83A598",
    },
    ui = {
        status_bar_bg = "dark_yellow",
        status_bar_fg = "black",
        topic_bar_bg = "dark_green",
        topic_bar_fg = "black",
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
