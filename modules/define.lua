-- LiCe5: Define — Dictionary lookup
-- Look up word definitions

lice5.define = {}

-- Command: /define <word>
void.register_command("DEFINE", "lice5_cmd_define")
function lice5_cmd_define(args)
    if #args == 0 then
        void.echo("-!- Usage: /define <word>")
        return
    end
    local word = args[1]
    void.echo("-!- Definition: " .. word)
    void.echo("-!- (Dictionary service not yet implemented)")
end
