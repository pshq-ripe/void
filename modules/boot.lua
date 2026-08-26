-- LiCe5: Boot — Boot/initialize
-- Boot sequence and initialization

lice5.boot = {
    booted = false,
}

function lice5.boot.run()
    if lice5.boot.booted then
        void.echo("-!- Already booted.")
        return
    end
    lice5.boot.booted = true
    void.echo("-!- Boot sequence complete.")
end

-- Command: /boot
void.register_command("BOOT", "lice5_cmd_boot")
function lice5_cmd_boot(args)
    lice5.boot.run()
end
