-- LiCe5: NickServ Auto-Identify
-- Automatically identifies with NickServ on connect

lice5.nickserv = {
    password = "",
    nick = "",
    enabled = true,
}

function lice5.nickserv.identify()
    if lice5.nickserv.password ~= "" and lice5.nickserv.enabled then
        void.msg("NickServ", "IDENTIFY " .. lice5.nickserv.password)
        void.echo("-!- Sent IDENTIFY to NickServ")
    end
end

-- Hook: on connect, identify
void.on("CONNECT", "lice5_on_connect_nickserv")
function lice5_on_connect_nickserv(args)
    if lice5.nickserv.password ~= "" then
        -- Small delay then identify
        void.timer(2, "lice5_nickserv_identify")
    end
end

function lice5_nickserv_identify()
    lice5.nickserv.identify()
end

-- Hook: on nick collision (433), try ghost
void.on("NICKINUSE", "lice5_on_nick_in_use")
function lice5_on_nick_in_use(args)
    if lice5.nickserv.nick ~= "" and lice5.nickserv.password ~= "" then
        void.msg("NickServ", "GHOST " .. lice5.nickserv.nick)
        void.echo("-!- Sent GHOST for " .. lice5.nickserv.nick)
        void.timer(1, "lice5_nickserv_recover")
    end
end

function lice5_nickserv_recover()
    if lice5.nickserv.nick ~= "" then
        void.nick_change(lice5.nickserv.nick)
        void.timer(2, "lice5_nickserv_identify")
    end
end

-- Command: /nickserv <password> [nick]
void.register_command("NS", "lice5_cmd_nickserv")
void.register_command("NICKSERV", "lice5_cmd_nickserv")
function lice5_cmd_nickserv(args)
    if not args[1] then
        void.echo("-!- NickServ: " .. (lice5.nickserv.enabled and "ON" or "OFF"))
        return
    end
    
    if args[1] == "off" then
        lice5.nickserv.enabled = false
        void.echo("-!- NickServ auto-identify disabled")
    elseif args[1] == "on" then
        lice5.nickserv.enabled = true
        void.echo("-!- NickServ auto-identify enabled")
    else
        lice5.nickserv.password = args[1]
        lice5.nickserv.nick = args[2] or void.nick()
        lice5.nickserv.enabled = true
        void.echo("-!- NickServ password set for " .. lice5.nickserv.nick)
    end
end
