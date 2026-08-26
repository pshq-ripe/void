-- LiCe5: Dtog — Display toggle
-- Toggle various display features

lice5.dtog = {
    timestamps = true,
    colors = true,
    nicks = true,
    modes = true,
}

-- Command: /dtog [timestamps|colors|nicks|modes] [on|off]
void.register_command("DTOG", "lice5_cmd_dtog")
function lice5_cmd_dtog(args)
    if #args == 0 then
        void.echo("-!- Display toggles:")
        for key, val in pairs(lice5.dtog) do
            void.echo("  " .. key .. ": " .. (val and "ON" or "OFF"))
        end
        return
    end
    local feature = args[1]:lower()
    if lice5.dtog[feature] == nil then
        void.echo("-!- Unknown feature: " .. feature)
        return
    end
    if #args > 1 then
        lice5.dtog[feature] = args[2] == "on"
    else
        lice5.dtog[feature] = not lice5.dtog[feature]
    end
    void.echo("-!- " .. feature .. ": " .. (lice5.dtog[feature] and "ON" or "OFF"))
end
