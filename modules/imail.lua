-- LiCe5: Imail — Internal mail system
-- Send mail to offline users

lice5.imail = {
    inbox = {},
    sent = {},
}

function lice5.imail.send(to, message)
    table.insert(lice5.imail.sent, {
        to = to,
        message = message,
        time = os.time(),
        read = false,
    })
    void.echo("-!- Mail sent to " .. to .. ": " .. message)
end

function lice5.imail.check(nick)
    local found = false
    for _, mail in ipairs(lice5.imail.inbox) do
        if mail.from:lower() == nick:lower() and not mail.read then
            void.echo("-!- Mail from " .. mail.from .. ": " .. mail.message)
            mail.read = true
            found = true
        end
    end
    if not found then
        void.echo("-!- No new mail from " .. nick)
    end
end

function lice5.imail.list()
    if #lice5.imail.inbox == 0 then
        void.echo("-!- No mail.")
    else
        void.echo("-!- Mail:")
        for i, mail in ipairs(lice5.imail.inbox) do
            local status = mail.read and "[read]" or "[NEW]"
            void.echo("  " .. i .. ": from " .. mail.from .. " " .. status .. " " .. mail.message)
        end
    end
end

-- Command: /imail [send|check|list] [nick] [message]
void.register_command("IMAIL", "lice5_cmd_imail")
function lice5_cmd_imail(args)
    if #args == 0 then
        lice5.imail.list()
        return
    end
    local action = args[1]:lower()
    if action == "send" then
        if #args < 3 then
            void.echo("-!- Usage: /imail send <nick> <message>")
            return
        end
        lice5.imail.send(args[2], table.concat(args, " ", 3))
    elseif action == "check" then
        lice5.imail.check(args[2] or "")
    elseif action == "list" then
        lice5.imail.list()
    else
        -- /imail <nick> <message>
        if #args < 2 then
            void.echo("-!- Usage: /imail <nick> <message>")
            return
        end
        lice5.imail.send(args[1], table.concat(args, " ", 2))
    end
end
