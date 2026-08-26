-- LiCe5: Key Bindings
-- Custom key bindings for common operations

lice5.keybinds = {
    bindings = {},
}

-- Register a key binding
function lice5.keybinds.bind(key, action)
    lice5.keybinds.bindings[key] = action
    void.echo("-!- Bound " .. key .. " = " .. action)
end

-- Common LiCe5 key bindings
function lice5.keybinds.setup_defaults()
    -- These are handled by the Rust side
    -- This module provides the Lua-side binding definitions
    void.echo("-!- LiCe5 key bindings loaded (see /bind for list)")
end

-- Command: /bind [key] [action]
void.register_command("LICE_BIND", "lice5_cmd_bind")
function lice5_cmd_bind(args)
    if not args[1] then
        void.echo("-!- LiCe5 key bindings:")
        for key, action in pairs(lice5.keybinds.bindings) do
            void.echo("  " .. key .. " = " .. action)
        end
        return
    end
    
    if args[2] then
        lice5.keybinds.bind(args[1], table.concat(args, " ", 2))
    else
        lice5.keybinds.bindings[args[1]] = nil
        void.echo("-!- Unbound " .. args[1])
    end
end

-- Setup defaults
lice5.keybinds.setup_defaults()
