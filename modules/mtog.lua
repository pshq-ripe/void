-- LiCe5: Mtog — Message toggle
-- Toggle message display on/off

lice5.mtog = {
    enabled = true,
}

-- Command: /mtog [on|off]
void.register_command("MTOG", "lice5_cmd_mtog")
function lice5_cmd_mtog(args)
    if #args == 0 then
        lice5.mtog.enabled = not lice5.mtog.enabled
    elseif args[1] == "on" then
        lice5.mtog.enabled = true
    elseif args[1] == "off" then
        lice5.mtog.enabled = false
    end
    void.echo("-!- Message display: " .. (lice5.mtog.enabled and "ON" or "OFF"))
end
