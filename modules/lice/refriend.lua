-- LiCe5: Refriend — Quick friend management
-- Add/remove friends from userlist quickly

lice5.refriend = {}

-- Command: /refriend <nick> [level]
void.register_command("REFRIEND", "lice5_cmd_refriend")
function lice5_cmd_refriend(args)
    if #args == 0 then
        void.echo("-!- Usage: /refriend <nick> [level]")
        void.echo("-!- Levels: OWNER, ADMIN, OP, HALFOP, VOICE, FRIEND")
        return
    end
    local nick = args[1]
    local level = args[2] or "FRIEND"
    -- Use the userlist module
    if lice5.userlist then
        lice5.userlist.add(nick, "*!*@*", level)
    else
        void.echo("-!- Userlist module not loaded.")
    end
end
