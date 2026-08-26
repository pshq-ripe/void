-- LiCe5: Fkeys — Function key bindings
-- F1-F12 key binding support

lice5.fkeys = {
    bindings = {},
}

function lice5.fkeys.set(key, action)
    lice5.fkeys.bindings[key] = action
    void.echo("-!- F-key bound: " .. key .. " = " .. action)
end

function lice5.fkeys.list()
    if next(lice5.fkeys.bindings) == nil then
        void.echo("-!- No F-key bindings.")
    else
        void.echo("-!- F-key bindings:")
        for key, action in pairs(lice5.fkeys.bindings) do
            void.echo("  " .. key .. " = " .. action)
        end
    end
end

-- Command: /fkey [F1-F12] [action]
void.register_command("FKEY", "lice5_cmd_fkey")
function lice5_cmd_fkey(args)
    if #args == 0 then
        lice5.fkeys.list()
        return
    end
    if #args == 1 then
        local action = lice5.fkeys.bindings[args[1]]
        if action then
            void.echo("-!- " .. args[1] .. " = " .. action)
        else
            void.echo("-!- No binding for: " .. args[1])
        end
        return
    end
    lice5.fkeys.set(args[1], table.concat(args, " ", 2))
end
