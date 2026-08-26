-- LiCe5: Finger
-- User info lookup

lice5.finger = {}

-- Command: /finger <nick>
void.register_command("FINGER", "lice5_cmd_finger")
function lice5_cmd_finger(args)
    if #args == 0 then
        void.echo("-!- Usage: /finger <nick>")
        return
    end
    local nick = args[1]
    void.echo("-!- Finger: " .. nick)
    -- Use WHOIS as a proxy for finger info
    void.whois(nick)
end
