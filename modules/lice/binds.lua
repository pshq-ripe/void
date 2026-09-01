-- LiCe5: Binds — Key binding management
-- Enhanced key binding system

lice5.binds = {
    bindings = {},
}

function lice5.binds.set(key, action)
    lice5.binds.bindings[key] = action
    void.echo("-!- Bound: " .. key .. " = " .. action)
end

function lice5.binds.remove(key)
    if lice5.binds.bindings[key] then
        lice5.binds.bindings[key] = nil
        void.echo("-!- Unbound: " .. key)
        return true
    end
    return false
end

function lice5.binds.list()
    if next(lice5.binds.bindings) == nil then
        void.echo("-!- No custom key bindings.")
    else
        void.echo("-!- Key bindings:")
        for key, action in pairs(lice5.binds.bindings) do
            void.echo("  " .. key .. " = " .. action)
        end
    end
end

-- Command: /binds [key] [action]
void.register_command("BINDS", "lice5_cmd_binds")
function lice5_cmd_binds(args)
    if #args == 0 then
        lice5.binds.list()
        return
    end
    if #args == 1 then
        if not lice5.binds.remove(args[1]) then
            void.echo("-!- No binding for: " .. args[1])
        end
        return
    end
    lice5.binds.set(args[1], table.concat(args, " ", 2))
end
