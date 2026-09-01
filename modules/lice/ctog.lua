-- LiCe5: Ctog — Channel toggle
-- Toggle channel features on/off

lice5.ctog = {
    joins = true,
    parts = true,
    quits = true,
    nicks = true,
    modes = true,
    topics = true,
}

-- Command: /ctog [joins|parts|quits|nicks|modes|topics] [on|off]
void.register_command("CTOG", "lice5_cmd_ctog")
function lice5_cmd_ctog(args)
    if #args == 0 then
        void.echo("-!- Channel toggles:")
        for key, val in pairs(lice5.ctog) do
            void.echo("  " .. key .. ": " .. (val and "ON" or "OFF"))
        end
        return
    end
    local feature = args[1]:lower()
    if lice5.ctog[feature] == nil then
        void.echo("-!- Unknown feature: " .. feature)
        return
    end
    if #args > 1 then
        lice5.ctog[feature] = args[2] == "on"
    else
        lice5.ctog[feature] = not lice5.ctog[feature]
    end
    void.echo("-!- " .. feature .. ": " .. (lice5.ctog[feature] and "ON" or "OFF"))
end
