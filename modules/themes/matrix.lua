-- Matrix Theme
-- Green-on-black hacker aesthetic

void_themes.register("Matrix", {
    name = "Matrix",
    desc = "Green-on-black hacker aesthetic",
    colors = {
        bg = "#000000",
        bg_panel = "#0A0A0A",
        bg_element = "#1A1A1A",
        text = "#00FF00",
        text_muted = "#006600",
        primary = "#00FF00",
        secondary = "#00CC00",
        accent = "#33FF33",
        error = "#FF0000",
        warning = "#FFFF00",
        success = "#00FF00",
        info = "#00FF00",
    },
    ui = {
        status_bar_bg = "black",
        status_bar_fg = "green",
        topic_bar_bg = "black",
        topic_bar_fg = "light_green",
        input_fg = "light_green",
        border = "green",
        scroll_indicator = "light_green",
        timestamp = "dark_gray",
    },
    nicks = {
        op = "light_green",
        voice = "green",
        halfop = "cyan",
        founder = "light_green",
        admin = "light_green",
        normal = "green",
    },
    messages = {
        normal = "green",
        action = "light_green",
        system = "cyan",
        notice = "light_green",
        highlight = "white",
        error = "red",
        server = "dark_gray",
        ctcp = "red",
    },
})
