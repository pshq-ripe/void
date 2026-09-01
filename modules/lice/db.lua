-- LiCe5: Db — Database operations
-- Simple key-value database

lice5.db = {
    store = {},
}

function lice5.db.set(key, value)
    lice5.db.store[key] = value
    void.echo("-!- DB set: " .. key .. " = " .. value)
end

function lice5.db.get(key)
    local value = lice5.db.store[key]
    if value then
        void.echo("-!- DB get: " .. key .. " = " .. value)
    else
        void.echo("-!- DB get: " .. key .. " = (not set)")
    end
    return value
end

function lice5.db.delete(key)
    if lice5.db.store[key] then
        lice5.db.store[key] = nil
        void.echo("-!- DB deleted: " .. key)
        return true
    end
    return false
end

function lice5.db.list()
    if next(lice5.db.store) == nil then
        void.echo("-!- Database is empty.")
    else
        void.echo("-!- Database:")
        for key, value in pairs(lice5.db.store) do
            void.echo("  " .. key .. " = " .. value)
        end
    end
end

-- Command: /db [set|get|del|list] [key] [value]
void.register_command("DB", "lice5_cmd_db")
function lice5_cmd_db(args)
    if #args == 0 then
        lice5.db.list()
        return
    end
    local action = args[1]:lower()
    if action == "set" and #args >= 3 then
        lice5.db.set(args[2], table.concat(args, " ", 3))
    elseif action == "get" and #args >= 2 then
        lice5.db.get(args[2])
    elseif action == "del" and #args >= 2 then
        lice5.db.delete(args[2])
    elseif action == "list" then
        lice5.db.list()
    else
        void.echo("-!- Usage: /db [set|get|del|list] [key] [value]")
    end
end
