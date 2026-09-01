-- LiCe5: Ul Save — Userlist save/load
-- Save and load userlist to/from file

lice5.ul_save = {
    path = "modules/userlist.dat",
}

function lice5.ul_save.save()
    if not lice5.userlist then
        void.echo("-!- Userlist module not loaded.")
        return
    end
    local lines = {}
    for _, user in ipairs(lice5.userlist.users) do
        table.insert(lines, user.nick .. "|" .. user.host .. "|" .. user.level .. "|" .. user.channels)
    end
    if void.file_write(lice5.ul_save.path, table.concat(lines, "\n")) then
        void.echo("-!- Userlist saved to " .. lice5.ul_save.path)
    else
        void.echo("-!- Failed to save userlist")
    end
end

function lice5.ul_save.load()
    if not lice5.userlist then
        void.echo("-!- Userlist module not loaded.")
        return
    end
    local content = void.file_read(lice5.ul_save.path)
    if content:sub(1, 5) == "Error" then
        void.echo("-!- " .. content)
        return
    end
    local count = 0
    for line in content:gmatch("[^\n]+") do
        local parts = {}
        for part in line:gmatch("[^|]+") do
            table.insert(parts, part)
        end
        if #parts >= 3 then
            lice5.userlist.add(parts[1], parts[2], parts[3], parts[4])
            count = count + 1
        end
    end
    void.echo("-!- Loaded " .. count .. " userlist entries")
end

-- Command: /ulsave [save|load]
void.register_command("ULSAVE", "lice5_cmd_ulsave")
function lice5_cmd_ulsave(args)
    if #args == 0 then
        void.echo("-!- Usage: /ulsave [save|load]")
        return
    end
    if args[1] == "save" then
        lice5.ul_save.save()
    elseif args[1] == "load" then
        lice5.ul_save.load()
    end
end
