-- LiCe5: Log Manager (epic6 style)
-- Automatic per-channel/per-server log file management

lice5.logman = {
    enabled = false,
    log_dir = "~/.void/logs",
    format = "%Y-%m-%d",  -- date format for log filenames
    auto_log = false,     -- auto-start logging on join
}

function lice5.logman.get_log_path(channel)
    local date = os.date(lice5.logman.format)
    local safe_channel = channel:gsub("[^%w%-]", "_")
    return lice5.logman.log_dir .. "/" .. safe_channel .. "-" .. date .. ".log"
end

function lice5.logman.start(channel)
    if not channel or channel == "" then
        channel = void.channel()
    end
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    -- Use the Rust-side logging system
    void.echo("-!- Logging started for " .. channel)
end

function lice5.logman.stop(channel)
    if not channel or channel == "" then
        channel = void.channel()
    end
    void.echo("-!- Logging stopped for " .. channel)
end

-- Hook: auto-log on join
void.on("JOIN", "lice5_logman_autojoin")
function lice5_logman_autojoin(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    if nick == void.nick() and lice5.logman.auto_log then
        lice5.logman.start(channel)
    end
end

-- Command: /logman [start|stop|auto] [channel]
void.register_command("LOGMAN", "lice5_cmd_logman")
function lice5_cmd_logman(args)
    if #args == 0 then
        void.echo("-!- Log manager: " .. (lice5.logman.enabled and "ON" or "OFF"))
        void.echo("-!- Auto-log: " .. (lice5.logman.auto_log and "ON" or "OFF"))
        void.echo("-!- Log dir: " .. lice5.logman.log_dir)
        return
    end

    local action = args[1]:lower()
    local channel = args[2] or ""

    if action == "on" or action == "start" then
        lice5.logman.enabled = true
        lice5.logman.start(channel)
    elseif action == "off" or action == "stop" then
        lice5.logman.enabled = false
        lice5.logman.stop(channel)
    elseif action == "auto" then
        lice5.logman.auto_log = not lice5.logman.auto_log
        void.echo("-!- Auto-logging: " .. (lice5.logman.auto_log and "ON" or "OFF"))
    elseif action == "dir" then
        if channel ~= "" then
            lice5.logman.log_dir = channel
            void.echo("-!- Log directory: " .. channel)
        else
            void.echo("-!- Log directory: " .. lice5.logman.log_dir)
        end
    else
        void.echo("-!- Usage: /logman [on|off|auto|dir] [channel|path]")
    end
end
