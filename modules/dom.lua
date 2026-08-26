-- LiCe5: Dom — Domain operations
-- DNS and domain utilities

lice5.dom = {}

-- Command: /dom <domain> — domain info lookup
void.register_command("DOM", "lice5_cmd_dom")
function lice5_cmd_dom(args)
    if #args == 0 then
        void.echo("-!- Usage: /dom <domain>")
        return
    end
    local domain = args[1]
    void.echo("-!- Domain: " .. domain)
    void.echo("-!- (Use /dns for DNS lookup)")
end
