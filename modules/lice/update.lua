-- LiCe5: Update Checker
-- Check for Void IRC client updates

lice5.update = {
    current_version = "0.1.0",
}

function lice5.update.check()
    void.echo("-!- Void IRC Client v" .. lice5.update.current_version)
    void.echo("-!- Check https://github.com/pshq-ripe/void for updates")
end

-- Command: /update
void.register_command("UPDATE", "lice5_cmd_update")
function lice5_cmd_update(args)
    lice5.update.check()
end
