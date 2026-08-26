-- LiCe5: DNS Lookup
-- DNS resolution commands

lice5.dns = {}

function lice5.dns.lookup(target)
    void.echo("-!- DNS lookup: " .. target)
    -- Use void.whois as a proxy for DNS info
    void.whois(target)
end

-- Command: /dns <nick|host>
void.register_command("DNS", "lice5_cmd_dns")
function lice5_cmd_dns(args)
    if #args == 0 then
        void.echo("-!- Usage: /dns <nick|host|ip>")
        return
    end
    lice5.dns.lookup(args[1])
end
