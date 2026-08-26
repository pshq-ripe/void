math.randomseed(os.time())

config = {
    nickname = "void_" .. tostring(math.random(100, 999)),
    server = "irc.spadhausen.com",
    channels = {"#ghost-bots"}
}

-- LiCe5 compatibility layer (modules/)
dofile("modules/init.lua")

-- Custom quit reasons (LiCe style)
local quit_reasons = {
    "Leaving",
    "Connection reset by peer",
    "Ping timeout: 240 seconds",
    "Segmentation fault (core dumped)",
    "Ctrl+C",
    "Gone to lunch",
    "BRB",
}

function get_quit_message()
    return quit_reasons[math.random(1, #quit_reasons)]
end

-- Example: custom command via Lua
void.register_command("HELLO", "cmd_hello")
function cmd_hello(args)
    void.echo("Hello, " .. (args[1] or "world") .. "! I am " .. void.nick())
end

-- Example: on-join hook
void.on("JOIN", "on_join_example")
function on_join_example(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    if nick ~= void.nick() then
        void.echo("-!- Welcome to " .. channel .. ", " .. nick .. "!")
    end
end
