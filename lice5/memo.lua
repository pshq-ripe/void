-- LiCe5: Memo System
-- Send offline messages to users

lice5.memo = {
    memos = {},  -- {from, to, message, time, read}
}

function lice5.memo.send(to, message)
    table.insert(lice5.memo.memos, {
        from = void.nick(),
        to = to,
        message = message,
        time = os.time(),
        read = false,
    })
    void.echo("-!- Memo sent to " .. to .. ": " .. message)
end

function lice5.memo.check(nick)
    local found = false
    for _, memo in ipairs(lice5.memo.memos) do
        if memo.to:lower() == nick:lower() and not memo.read then
            void.echo("-!- Memo from " .. memo.from .. ": " .. memo.message)
            memo.read = true
            found = true
        end
    end
    if not found then
        void.echo("-!- No new memos for " .. nick)
    end
end

function lice5.memo.list()
    if #lice5.memo.memos == 0 then
        void.echo("-!- No memos.")
    else
        void.echo("-!- Memos:")
        for i, memo in ipairs(lice5.memo.memos) do
            local status = memo.read and "[read]" or "[NEW]"
            void.echo("  " .. i .. ": " .. memo.from .. " -> " .. memo.to .. " " .. status .. " " .. memo.message)
        end
    end
end

-- Command: /memo [send|check|list] [nick] [message]
void.register_command("MEMO", "lice5_cmd_memo")
function lice5_cmd_memo(args)
    if #args == 0 then
        lice5.memo.list()
        return
    end
    local action = args[1]:lower()
    if action == "send" then
        if #args < 3 then
            void.echo("-!- Usage: /memo send <nick> <message>")
            return
        end
        lice5.memo.send(args[2], table.concat(args, " ", 3))
    elseif action == "check" then
        lice5.memo.check(args[2] or void.nick())
    elseif action == "list" then
        lice5.memo.list()
    else
        -- /memo <nick> <message>
        if #args < 2 then
            void.echo("-!- Usage: /memo <nick> <message>")
            return
        end
        lice5.memo.send(args[1], table.concat(args, " ", 2))
    end
end
