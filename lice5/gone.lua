-- LiCe5: Gone/Back System
-- Random away/back reasons from files

lice5.gone = {
    away_since = nil,
    away_msg = "",
    gone_reasons = {},
    back_reasons = {},
}

-- Load reasons from files
function lice5.gone.load_reasons()
    -- Load gone reasons
    local f = io.open("lice5/gone.reasons", "r")
    if f then
        for line in f:lines() do
            line = line:match("^%s*(.-)%s*$")
            if line and line ~= "" and not line:match("^#") then
                table.insert(lice5.gone.gone_reasons, line)
            end
        end
        f:close()
    end
    -- Load back reasons
    f = io.open("lice5/back.reasons", "r")
    if f then
        for line in f:lines() do
            line = line:match("^%s*(.-)%s*$")
            if line and line ~= "" and not line:match("^#") then
                table.insert(lice5.gone.back_reasons, line)
            end
        end
        f:close()
    end
    -- Fallback defaults
    if #lice5.gone.gone_reasons == 0 then
        lice5.gone.gone_reasons = {"Gone", "BRB", "AFK", "Away", "Sleeping", "Eating", "Working"}
    end
    if #lice5.gone.back_reasons == 0 then
        lice5.gone.back_reasons = {"Back", "I'm back", "Returned", "Here", "Present"}
    end
end

function lice5.gone.random_reason(list)
    if #list == 0 then return "Away" end
    return list[math.random(1, #list)]
end

function lice5.gone.set_away(msg)
    if lice5.gone.away_since then
        void.echo("-!- Already away: " .. lice5.gone.away_msg)
        return
    end
    lice5.gone.away_msg = msg or lice5.gone.random_reason(lice5.gone.gone_reasons)
    lice5.gone.away_since = os.time()
    void.away(lice5.gone.away_msg)
    void.echo("-!- You are now away: " .. lice5.gone.away_msg)
end

function lice5.gone.set_back(msg)
    if not lice5.gone.away_since then
        void.echo("-!- You are not away.")
        return
    end
    local duration = os.time() - lice5.gone.away_since
    local mins = math.floor(duration / 60)
    local secs = duration % 60
    local back_msg = msg or lice5.gone.random_reason(lice5.gone.back_reasons)

    lice5.gone.away_since = nil
    lice5.gone.away_msg = ""
    void.away(nil)
    void.echo("-!- You are back (" .. mins .. "m " .. secs .. "s): " .. back_msg)
end

-- Initialize
lice5.gone.load_reasons()

-- Command: /gone [message] — set away with random or custom reason
void.register_command("GONE", "lice5_cmd_gone")
function lice5_cmd_gone(args)
    if #args == 0 then
        lice5.gone.set_away()
    elseif args[1] == "off" or args[1] == "back" then
        lice5.gone.set_back()
    elseif args[1] == "-a" then
        -- Away on all servers (future multi-server)
        local msg = #args > 1 and table.concat(args, " ", 2) or nil
        lice5.gone.set_away(msg)
    else
        lice5.gone.set_away(table.concat(args, " "))
    end
end

-- Command: /back [message] — return from away
void.register_command("BACK", "lice5_cmd_back")
function lice5_cmd_back(args)
    local msg = #args > 0 and table.concat(args, " ") or nil
    lice5.gone.set_back(msg)
end
