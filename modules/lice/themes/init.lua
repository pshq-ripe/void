-- Void IRC Client Theme System
-- Themes define comprehensive color schemes for all UI elements
-- Usage: /theme <name>, /theme list, /theme info <name>, /theme random

void_themes = {}
void_themes.current = "catppuccin"

-- Register a theme definition
function void_themes.register(name, theme)
    theme.name = name
    theme.desc = theme.desc or (name .. " theme")
    theme.is_dark = (theme.is_dark ~= false)
    void_themes[name:lower()] = theme
end

-- Apply a theme by name
function void_themes.apply(name)
    local key = name:lower()
    local theme = void_themes[key]
    if not theme then
        void.echo("-!- Theme not found: " .. name .. ". Type /theme list to see available themes.")
        return false
    end
    -- Apply theme to Rust renderer
    void.apply_theme(key)
    void_themes.current = key
    return true
end

-- List all available themes
function void_themes.list()
    void.echo("-!- ========================================================")
    void.echo("-!- Available Themes in Void IRC Client:")
    void.echo("-!- ========================================================")
    
    local dark_themes = {}
    local light_themes = {}
    
    for key, theme in pairs(void_themes) do
        if type(theme) == "table" and theme.name then
            if theme.is_dark then
                table.insert(dark_themes, theme)
            else
                table.insert(light_themes, theme)
            end
        end
    end
    
    table.sort(dark_themes, function(a, b) return a.name < b.name end)
    table.sort(light_themes, function(a, b) return a.name < b.name end)
    
    void.echo("-!- [Dark Themes]")
    for _, theme in ipairs(dark_themes) do
        local marker = (void_themes.current == theme.name:lower()) and " [*ACTIVE*]" or ""
        void.echo(string.format("  %-18s %s %s", theme.name, marker, theme.desc or ""))
    end
    
    if #light_themes > 0 then
        void.echo("-!- [Light Themes]")
        for _, theme in ipairs(light_themes) do
            local marker = (void_themes.current == theme.name:lower()) and " [*ACTIVE*]" or ""
            void.echo(string.format("  %-18s %s %s", theme.name, marker, theme.desc or ""))
        end
    end
    
    void.echo("-!- ========================================================")
    void.echo("-!- Type /theme <name> to apply, or /theme random for a surprise!")
end

-- Show detailed information about a theme
function void_themes.info(name)
    local theme = void_themes[name:lower()]
    if not theme then
        void.echo("-!- Theme not found: " .. name)
        return
    end
    void.echo("-!- -- Theme Info: " .. theme.name .. " --")
    void.echo("  Description: " .. (theme.desc or "N/A"))
    void.echo("  Type:        " .. (theme.is_dark and "Dark" or "Light"))
    void.echo("  Status:      " .. ((void_themes.current == name:lower()) and "Active" or "Inactive"))
    if theme.ui then
        void.echo("  Status Bar:  " .. (theme.ui.status_bar_bg or "default") .. " (bg) / " .. (theme.ui.status_bar_fg or "default") .. " (fg)")
        void.echo("  Topic Bar:   " .. (theme.ui.topic_bar_bg or "default") .. " (bg) / " .. (theme.ui.topic_bar_fg or "default") .. " (fg)")
        void.echo("  Border:      " .. (theme.ui.border or "default"))
    end
end

-- Apply a random theme
function void_themes.random()
    local names = {}
    for key, theme in pairs(void_themes) do
        if type(theme) == "table" and theme.name then
            table.insert(names, theme.name)
        end
    end
    if #names == 0 then return end
    local choice = names[math.random(1, #names)]
    void_themes.apply(choice)
end

-- Command: /theme [name|list|info <name>|random]
void.register_command("THEME", "void_cmd_theme")
function void_cmd_theme(args)
    if #args == 0 then
        if void_themes.current and void_themes[void_themes.current] then
            local cur = void_themes[void_themes.current]
            void.echo("-!- Current theme: " .. cur.name .. " -- " .. (cur.desc or ""))
        else
            void.echo("-!- No custom theme active (using defaults)")
        end
        void_themes.list()
        return
    end

    local subcmd = args[1]:lower()
    if subcmd == "list" then
        void_themes.list()
        return
    elseif subcmd == "random" then
        void_themes.random()
        return
    elseif subcmd == "info" and args[2] then
        void_themes.info(args[2])
        return
    end

    void_themes.apply(args[1])
end
