-- LiCe5: Oops — Fix last mistake
-- Quickly fix common mistakes (wrong channel, wrong nick, etc.)

lice5.oops = {
    last_target = nil,
    last_message = nil,
}

-- Hook: track last message
void.on("PUBLIC", "lice5_oops_track")
function lice5_oops_track(args)
    local nick = args[1] or ""
    if nick == void.nick() then
        lice5.oops.last_target = void.channel()
    end
end

-- Command: /oops <correction> — send correction to last target
void.register_command("OOPS", "lice5_cmd_oops")
function lice5_cmd_oops(args)
    if #args == 0 then
        void.echo("-!- Usage: /oops <correction text>")
        return
    end
    local correction = table.concat(args, " ")
    local channel = lice5.oops.last_target or void.channel()
    if channel == "" then
        void.echo("-!- No target to correct.")
        return
    end
    void.msg(channel, "* " .. void.nick() .. " meant: " .. correction)
end
