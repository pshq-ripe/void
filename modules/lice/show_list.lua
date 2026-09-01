-- LiCe5: Show List — Display various lists
-- Unified list display command

lice5.show_list = {}

-- Command: /showlist [bans|exceptions|invites|users|ops|voices]
void.register_command("SHOWLIST", "lice5_cmd_showlist")
function lice5_cmd_showlist(args)
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    if #args == 0 then
        void.echo("-!- Usage: /showlist [bans|exceptions|invites|users|ops|voices]")
        return
    end
    local list_type = args[1]:lower()
    if list_type == "bans" or list_type == "ban" then
        void.mode(channel, "+b")
    elseif list_type == "exceptions" or list_type == "exc" then
        void.mode(channel, "+e")
    elseif list_type == "invites" or list_type == "inv" then
        void.mode(channel, "+I")
    elseif list_type == "users" or list_type == "nicks" then
        void.echo("-!- Users in " .. channel .. ": (use /names)")
    elseif list_type == "ops" then
        void.echo("-!- Ops in " .. channel .. ": (check nick list for @ prefix)")
    elseif list_type == "voices" then
        void.echo("-!- Voices in " .. channel .. ": (check nick list for + prefix)")
    else
        void.echo("-!- Unknown list type: " .. list_type)
    end
end
