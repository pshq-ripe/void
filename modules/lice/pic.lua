-- LiCe5: Pic — ASCII art pictures
-- Display ASCII art pictures

lice5.pic = {
    pictures = {},
}

function lice5.pic.load(filename)
    local content = void.file_read(filename)
    if content:sub(1, 5) == "Error" then
        void.echo("-!- " .. content)
        return
    end
    table.insert(lice5.pic.pictures, {
        name = filename,
        content = content,
    })
    void.echo("-!- Picture loaded: " .. filename)
end

function lice5.pic.show(index)
    if #lice5.pic.pictures == 0 then
        void.echo("-!- No pictures loaded.")
        return
    end
    local pic = lice5.pic.pictures[index or 1]
    if pic then
        for line in pic.content:gmatch("[^\n]+") do
            void.echo(line)
        end
    end
end

-- Command: /pic [load|show] [filename|index]
void.register_command("PIC", "lice5_cmd_pic")
function lice5_cmd_pic(args)
    if #args == 0 then
        lice5.pic.show(1)
        return
    end
    if args[1] == "load" and args[2] then
        lice5.pic.load(args[2])
    elseif args[1] == "show" then
        lice5.pic.show(tonumber(args[2]) or 1)
    else
        lice5.pic.load(args[1])
    end
end
