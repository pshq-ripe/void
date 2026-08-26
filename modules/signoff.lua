-- LiCe5: Signoff Messages
-- Random quit/signoff messages

lice5.signoff = {
    reasons = {},
}

function lice5.signoff.load_reasons()
    local f = io.open("modules/quit.reasons", "r")
    if f then
        for line in f:lines() do
            line = line:match("^%s*(.-)%s*$")
            if line and line ~= "" and not line:match("^#") then
                table.insert(lice5.signoff.reasons, line)
            end
        end
        f:close()
    end
    if #lice5.signoff.reasons == 0 then
        lice5.signoff.reasons = {
            "Leaving",
            "Goodbye",
            "See you later",
            "BRB",
            "Gone fishing",
            "EOF",
            "Connection reset by peer",
            "Segmentation fault",
        }
    end
end

function lice5.signoff.random()
    if #lice5.signoff.reasons == 0 then return "Leaving" end
    return lice5.signoff.reasons[math.random(1, #lice5.signoff.reasons)]
end

lice5.signoff.load_reasons()

-- Command: /signoff [reason]
void.register_command("SIGNOFF", "lice5_cmd_signoff")
function lice5_cmd_signoff(args)
    local reason = #args > 0 and table.concat(args, " ") or lice5.signoff.random()
    void.echo("-!- Signoff: " .. reason)
    void.quit(reason)
end
