-- LiCe5: Looplist — Loop through a list
-- Iterate over a list and execute commands

lice5.looplist = {}

-- Command: /looplist <list> <command>
void.register_command("LOOPLIST", "lice5_cmd_looplist")
function lice5_cmd_looplist(args)
    if #args < 2 then
        void.echo("-!- Usage: /looplist <item1,item2,...> <command>")
        return
    end
    local items_str = args[1]
    local command = table.concat(args, " ", 2)
    local count = 0
    for item in items_str:gmatch("[^,]+") do
        item = item:match("^%s*(.-)%s*$")
        if item and item ~= "" then
            -- Replace $item in command
            local cmd = command:gsub("$item", item)
            void.echo("-!- Loop: " .. cmd)
            count = count + 1
        end
    end
    void.echo("-!- Loop completed: " .. count .. " items")
end
