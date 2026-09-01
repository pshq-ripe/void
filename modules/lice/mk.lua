-- LiCe5: Mk — Make/create helper
-- Quick file creation

lice5.mk = {}

-- Command: /mk <filename> [content]
void.register_command("MK", "lice5_cmd_mk")
function lice5_cmd_mk(args)
    if #args == 0 then
        void.echo("-!- Usage: /mk <filename> [content]")
        return
    end
    local filename = args[1]
    local content = #args > 1 and table.concat(args, " ", 2) or ""
    if void.file_write(filename, content) then
        void.echo("-!- Created: " .. filename)
    else
        void.echo("-!- Failed to create: " .. filename)
    end
end
