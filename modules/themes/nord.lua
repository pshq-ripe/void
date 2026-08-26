-- Nord Theme (inspired by MiMo-Code nord.json)
-- Arctic, north-bluish color palette

void_themes.register("Nord", {
    name = "Nord",
    desc = "Arctic north-bluish color palette",
    colors = {
        bg = "#2E3440",
        bg_panel = "#3B4252",
        bg_element = "#434C5E",
        text = "#D8DEE9",
        text_muted = "#4C566A",
        primary = "#88C0D0",
        secondary = "#81A1C1",
        accent = "#8FBCBB",
        error = "#BF616A",
        warning = "#D08770",
        success = "#A3BE8C",
        info = "#88C0D0",
    },
    ui = {
        status_bar_bg = "dark_gray",
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
