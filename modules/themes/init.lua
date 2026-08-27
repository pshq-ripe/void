-- Void IRC Client Theme System
-- Themes define colors for all UI elements
-- Usage: /theme <name> or /theme list

void_themes = {}

-- Theme format:
-- Each theme is a table with color assignments for UI elements
-- Colors are IRC mIRC color codes (0-15) or "default"
-- Elements: status_bar, topic_bar, input, nick_op, nick_voice, nick_normal,
--           msg_normal, msg_action, msg_system, msg_notice, msg_highlight,
--           msg_error, msg_server, timestamp, border, scroll_indicator

function void_themes.register(name, theme)
    theme.name = name
    void_themes[name:lower()] = theme
end

function void_themes.apply(name)
    local theme = void_themes[name:lower()]
    if not theme then
        void.echo("-!- Theme not found: " .. name)
        return false
    end
    -- Apply theme to Rust renderer
    void.apply_theme(name:lower())
    void_themes.current = name:lower()
    return true
end

function void_themes.list()
    void.echo("-!- Available themes:")
    for name, theme in pairs(void_themes) do
        if type(theme) == "table" and theme.name then
            local marker = (void_themes.current == name) and " *" or ""
            void.echo("  " .. theme.name .. marker)
        end
    end
end

-- Command: /theme [name|list]
void.register_command("THEME", "void_cmd_theme")
function void_cmd_theme(args)
    if #args == 0 then
        if void_themes.current then
            void.echo("-!- Current theme: " .. void_themes.current)
        else
            void.echo("-!- No theme active (using defaults)")
        end
        void_themes.list()
        return
    end
    if args[1] == "list" then
        void_themes.list()
        return
    end
    void_themes.apply(args[1])
end
