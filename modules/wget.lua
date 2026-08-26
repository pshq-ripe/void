-- LiCe5: Wget — URL fetch
-- Fetch content from URLs

lice5.wget = {}

-- Command: /wget <url>
void.register_command("WGET", "lice5_cmd_wget")
function lice5_cmd_wget(args)
    if #args == 0 then
        void.echo("-!- Usage: /wget <url>")
        return
    end
    local url = args[1]
    void.echo("-!- Fetching: " .. url)
    -- Note: actual HTTP fetch would need to be implemented in Rust side
    -- For now, just echo the URL
    void.echo("-!- URL: " .. url)
    void.echo("-!- (HTTP fetch not yet implemented — use /exec curl " .. url .. ")")
end
