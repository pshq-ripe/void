-- LiCe5: Sensors
-- Channel activity monitoring

lice5.sensors = {
    channels = {},
}

function lice5.sensors.enable(channel)
    lice5.sensors.channels[channel:upper()] = {
        joins = 0,
        parts = 0,
        kicks = 0,
        bans = 0,
        msgs = 0,
        started = os.time(),
    }
    void.echo("-!- Sensors enabled for " .. channel)
end

function lice5.sensors.disable(channel)
    lice5.sensors.channels[channel:upper()] = nil
    void.echo("-!- Sensors disabled for " .. channel)
end

function lice5.sensors.report(channel)
    local data = lice5.sensors.channels[channel:upper()]
    if not data then
        void.echo("-!- No sensor data for " .. channel)
        return
    end
    local duration = os.time() - data.started
    void.echo("-!- Sensor report for " .. channel .. " (" .. duration .. "s):")
    void.echo("  Joins: " .. data.joins .. " | Parts: " .. data.parts .. " | Kicks: " .. data.kicks)
    void.echo("  Bans: " .. data.bans .. " | Messages: " .. data.msgs)
end

-- Hooks
void.on("JOIN", "lice5_sensors_join")
function lice5_sensors_join(args)
    local channel = args[2] or ""
    local data = lice5.sensors.channels[channel:upper()]
    if data then data.joins = data.joins + 1 end
end

void.on("PART", "lice5_sensors_part")
function lice5_sensors_part(args)
    local channel = args[2] or ""
    local data = lice5.sensors.channels[channel:upper()]
    if data then data.parts = data.parts + 1 end
end

-- Command: /sensors [enable|disable|report] [#channel]
void.register_command("SENSORS", "lice5_cmd_sensors")
function lice5_cmd_sensors(args)
    local channel = void.channel()
    if #args == 0 then
        lice5.sensors.report(channel)
        return
    end
    local action = args[1]:lower()
    local ch = args[2] or channel
    if action == "enable" or action == "on" then
        lice5.sensors.enable(ch)
    elseif action == "disable" or action == "off" then
        lice5.sensors.disable(ch)
    elseif action == "report" then
        lice5.sensors.report(ch)
    end
end
