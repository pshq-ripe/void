-- LiCe5: Party Mode
-- Fun party commands

lice5.party = {
    active = false,
}

function lice5.party.start()
    lice5.party.active = true
    void.echo("-!- PARTY MODE ACTIVATED! \\o/")
end

function lice5.party.stop()
    lice5.party.active = false
    void.echo("-!- Party mode deactivated.")
end

-- Command: /party [on|off]
void.register_command("PARTY", "lice5_cmd_party")
function lice5_cmd_party(args)
    if #args == 0 then
        if lice5.party.active then
            lice5.party.stop()
        else
            lice5.party.start()
        end
    elseif args[1] == "on" then
        lice5.party.start()
    elseif args[1] == "off" then
        lice5.party.stop()
    end
end
