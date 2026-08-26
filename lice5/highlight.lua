-- LiCe5: Nick Highlight
-- Highlights messages containing your nick with configurable colors

lice5.highlight = {
    color = "yellow",
    patterns = {},
}

-- Add highlight pattern
function lice5.highlight.add(pattern, color)
    table.insert(lice5.highlight.patterns, { pattern = pattern, color = color or lice5.highlight.color })
    void.echo("-!- Highlight added: " .. pattern)
end

-- Remove highlight pattern
function lice5.highlight.remove(pattern)
    for i, h in ipairs(lice5.highlight.patterns) do
        if h.pattern == pattern then
            table.remove(lice5.highlight.patterns, i)
            void.echo("-!- Highlight removed: " .. pattern)
            return
        end
    end
end

-- Hook: check messages for highlights
void.on("PUBLIC", "lice5_on_highlight")
function lice5_on_highlight(args)
    local nick = args[1] or ""
    local text = args[2] or ""
    local my_nick = void.nick():lower()
    
    -- Check if message contains our nick
    if text:lower():find(my_nick, 1, true) then
        void.echo("-!- Highlight: <" .. nick .. "> " .. text)
    end
    
    -- Check custom patterns
    for _, h in ipairs(lice5.highlight.patterns) do
        if void.match(h.pattern, text) then
            void.echo("-!- Highlight match (" .. h.pattern .. "): <" .. nick .. "> " .. text)
        end
    end
end

-- Command: /highlight [pattern] [color]
void.register_command("LICE_HIGHLIGHT", "lice5_cmd_highlight")
function lice5_cmd_highlight(args)
    if not args[1] then
        void.echo("-!- Highlight patterns:")
        for i, h in ipairs(lice5.highlight.patterns) do
            void.echo("  " .. i .. ": " .. h.pattern .. " (" .. h.color .. ")")
        end
        return
    end
    
    local pattern = args[1]
    local color = args[2] or "yellow"
    
    -- Toggle: remove if exists, add otherwise
    for i, h in ipairs(lice5.highlight.patterns) do
        if h.pattern == pattern then
            table.remove(lice5.highlight.patterns, i)
            void.echo("-!- Highlight removed: " .. pattern)
            return
        end
    end
    
    lice5.highlight.add(pattern, color)
end
