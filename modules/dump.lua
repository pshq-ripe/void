-- LiCe5: Dump — Debug dump
-- Dump internal state for debugging

lice5.dump = {}

-- Command: /dump [settings|aliases|hooks|buffers|notify|ignore]
void.register_command("DUMP", "lice5_cmd_dump")
function lice5_cmd_dump(args)
    if #args == 0 then
        void.echo("-!- Usage: /dump [settings|aliases|hooks|buffers|notify|ignore]")
        return
    end
    local what = args[1]:lower()
    if what == "settings" then
        void.echo("-!- (Use /set to view settings)")
    elseif what == "aliases" then
        void.echo("-!- (Use /alias to view aliases)")
    elseif what == "hooks" then
        void.echo("-!- Lua hooks registered:")
        -- Would need access to hooks from Lua side
        void.echo("-!- (Check /help for available hooks)")
    elseif what == "buffers" then
        void.echo("-!- (Use /window to view buffers)")
    elseif what == "notify" then
        void.echo("-!- (Use /notify to view notify list)")
    elseif what == "ignore" then
        void.echo("-!- (Use /ignore to view ignore list)")
    else
        void.echo("-!- Unknown dump target: " .. what)
    end
end
