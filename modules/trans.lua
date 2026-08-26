-- LiCe5: Trans — Translation helper
-- Translation commands

lice5.trans = {}

-- Command: /trans <text> — translation placeholder
void.register_command("TRANS", "lice5_cmd_trans")
function lice5_cmd_trans(args)
    if #args == 0 then
        void.echo("-!- Usage: /trans <text>")
        return
    end
    local text = table.concat(args, " ")
    void.echo("-!- Translation: " .. text)
    void.echo("-!- (Translation service not yet implemented)")
end
