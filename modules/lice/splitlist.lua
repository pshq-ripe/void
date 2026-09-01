-- LiCe5: Split List — Track netsplits
-- Detect and display netsplits

lice5.splitlist = {
    splits = {},
}

function lice5.splitlist.record(server1, server2)
    table.insert(lice5.splitlist.splits, {
        server1 = server1,
        server2 = server2,
        time = os.time(),
    })
    void.echo("-!- Netsplit detected: " .. server1 .. " <-> " .. server2)
end

function lice5.splitlist.show()
    if #lice5.splitlist.splits == 0 then
        void.echo("-!- No netsplits recorded.")
    else
        void.echo("-!- Recent netsplits:")
        for i, split in ipairs(lice5.splitlist.splits) do
            local ago = os.time() - split.time
            void.echo("  " .. i .. ": " .. split.server1 .. " <-> " .. split.server2 .. " (" .. ago .. "s ago)")
        end
    end
end

-- Command: /splitlist
void.register_command("SPLITLIST", "lice5_cmd_splitlist")
function lice5_cmd_splitlist(args)
    lice5.splitlist.show()
end
