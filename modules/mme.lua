-- LiCe5: Mme — Mass message
-- Send message to multiple targets

lice5.mme = {}

-- Command: /mme <target1,target2,...> <message>
void.register_command("MME", "lice5_cmd_mme")
function lice5_cmd_mme(args)
    if #args < 2 then
        void.echo("-!- Usage: /mme <target1,target2,...> <message>")
        return
    end
    local targets = args[1]
    local message = table.concat(args, " ", 2)
    for target in targets:gmatch("[^,]+") do
        target = target:match("^%s*(.-)%s*$")
        if target and target ~= "" then
            void.msg(target, message)
            void.echo("-!- Sent to " .. target .. ": " .. message)
        end
    end
end
