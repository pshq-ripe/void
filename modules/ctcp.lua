-- LiCe5: CTCP Replies
-- Enhanced CTCP reply system

lice5.ctcp = {
    version = "Void IRC Client v0.1.0 (LiCe5/" .. lice5.version .. ")",
    userinfo = "LiCe5 script pack for Void",
    clientinfo = "ACTION VERSION PING TIME CLIENTINFO USERINFO DCC",
}

void.on("CTCP", "lice5_on_ctcp")
function lice5_on_ctcp(args)
    local nick = args[1] or ""
    local ctcp_type = (args[2] or ""):upper()
    local ctcp_args = args[3] or ""
    
    if ctcp_type == "VERSION" then
        void.notice(nick, "\001VERSION " .. lice5.ctcp.version .. "\001")
    elseif ctcp_type == "USERINFO" then
        void.notice(nick, "\001USERINFO " .. lice5.ctcp.userinfo .. "\001")
    elseif ctcp_type == "CLIENTINFO" then
        void.notice(nick, "\001CLIENTINFO " .. lice5.ctcp.clientinfo .. "\001")
    elseif ctcp_type == "PING" then
        void.notice(nick, "\001PING " .. ctcp_args .. "\001")
    elseif ctcp_type == "TIME" then
        void.notice(nick, "\001TIME " .. os.date("%a %b %d %H:%M:%S %Y") .. "\001")
    end
end
