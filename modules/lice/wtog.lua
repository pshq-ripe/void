-- LiCe5: Wtog — Window toggle
-- Toggle window features

lice5.wtog = {
    status = true,
    topic = true,
    nicks = true,
}

-- Command: /wtog [status|topic|nicks] [on|off]
void.register_command("WTOG", "lice5_cmd_wtog")
function lice5_cmd_wtog(args)
    if #args == 0 then
        void.echo("-!- Window toggles:")
        for key, val in pairs(lice5.wtog) do
            void.echo("  " .. key .. ": " .. (val and "ON" or "OFF"))
        end
        return
    end
    local feature = args[1]:lower()
    if lice5.wtog[feature] == nil then
        void.echo("-!- Unknown feature: " .. feature)
        return
    end
    if #args > 1 then
        lice5.wtog[feature] = args[2] == "on"
    else
        lice5.wtog[feature] = not lice5.wtog[feature]
    end
    void.echo("-!- " .. feature .. ": " .. (lice5.wtog[feature] and "ON" or "OFF"))
end
