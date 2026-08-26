-- LiCe5: Alarm/Timer System
-- Reminders and scheduled actions

lice5.alarm = {
    alarms = {},  -- {name, time, command, repeat_count}
    next_id = 1,
}

function lice5.alarm.set(name, seconds, command, repeat_count)
    local id = lice5.alarm.next_id
    lice5.alarm.next_id = lice5.alarm.next_id + 1
    table.insert(lice5.alarm.alarms, {
        id = id,
        name = name or ("alarm_" .. id),
        fire_at = os.time() + seconds,
        command = command,
        repeat_count = repeat_count or 1,
        interval = seconds,
    })
    void.echo("-!- Alarm '" .. (name or id) .. "' set for " .. seconds .. "s")
    return id
end

function lice5.alarm.cancel(name_or_id)
    for i, alarm in ipairs(lice5.alarm.alarms) do
        if alarm.name == name_or_id or alarm.id == tonumber(name_or_id) then
            table.remove(lice5.alarm.alarms, i)
            void.echo("-!- Alarm '" .. alarm.name .. "' cancelled.")
            return true
        end
    end
    void.echo("-!- No such alarm: " .. name_or_id)
    return false
end

function lice5.alarm.list()
    if #lice5.alarm.alarms == 0 then
        void.echo("-!- No active alarms.")
    else
        void.echo("-!- Active alarms:")
        for _, alarm in ipairs(lice5.alarm.alarms) do
            local remaining = alarm.fire_at - os.time()
            void.echo("  [" .. alarm.id .. "] " .. alarm.name .. " in " .. remaining .. "s: " .. alarm.command)
        end
    end
end

-- Check alarms (called periodically)
function lice5.alarm.check()
    local now = os.time()
    local to_fire = {}
    for i, alarm in ipairs(lice5.alarm.alarms) do
        if now >= alarm.fire_at then
            table.insert(to_fire, i)
        end
    end
    -- Fire in reverse order to avoid index shifting
    for i = #to_fire, 1, -1 do
        local idx = to_fire[i]
        local alarm = lice5.alarm.alarms[idx]
        void.echo("-!- ALARM: " .. alarm.name .. " — " .. alarm.command)
        -- Execute the command
        if alarm.command:sub(1, 1) == "/" then
            -- It's a command — would need to be dispatched
            void.echo("  (command: " .. alarm.command .. ")")
        else
            void.echo("  " .. alarm.command)
        end
        -- Handle repeat
        alarm.repeat_count = alarm.repeat_count - 1
        if alarm.repeat_count <= 0 then
            table.remove(lice5.alarm.alarms, idx)
        else
            alarm.fire_at = now + alarm.interval
        end
    end
end

-- Command: /alarm [name] <seconds> <command>
void.register_command("ALARM", "lice5_cmd_alarm")
function lice5_cmd_alarm(args)
    if #args == 0 then
        lice5.alarm.list()
        return
    end
    if args[1] == "off" or args[1] == "cancel" then
        if args[2] then
            lice5.alarm.cancel(args[2])
        else
            void.echo("-!- Usage: /alarm cancel <name|id>")
        end
        return
    end
    if #args < 2 then
        void.echo("-!- Usage: /alarm [name] <seconds> <command>")
        return
    end
    -- Check if first arg is a number (no name) or a name
    local seconds = tonumber(args[1])
    if seconds then
        -- No name: /alarm <seconds> <command>
        local command = table.concat(args, " ", 2)
        lice5.alarm.set(nil, seconds, command)
    else
        -- Named: /alarm <name> <seconds> <command>
        if #args < 3 then
            void.echo("-!- Usage: /alarm <name> <seconds> <command>")
            return
        end
        local name = args[1]
        seconds = tonumber(args[2])
        if not seconds then
            void.echo("-!- Invalid seconds: " .. args[2])
            return
        end
        local command = table.concat(args, " ", 3)
        lice5.alarm.set(name, seconds, command)
    end
end
