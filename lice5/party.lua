-- LiCe5: Party Mode
-- Fun party commands — disco colors, random actions, party vibes

lice5.party = {
    active = false,
    disco_colors = {"\x034", "\x037", "\x038", "\x039", "\x0312", "\x0313", "\x036", "\x033"},
    disco_frame = 1,
    party_lines = {
        "\\o/ \\o/ \\o/",
        "♪ ♫ ♪ ♫ ♪",
        "┏(°.°)┛ ┏(°.°)┛",
        "(ノ◕ヮ◕)ノ*:・゚✧",
        "♪┏(・o・)┛♪┗(・o・)┓♪",
        "ヽ(>∀<☆)ノ",
        "（☆▽☆）",
        "╰(*°▽°*)╯",
        "₍ᐢ.ˬ.ᐢ₎♡",
        "₍ᐢ.ˬ.ᐢ₎♡ ₍ᐢ.ˬ.ᐢ₎♡",
        "( ˘▽˘)っ♨",
        "ヽ(✿ﾟ▽ﾟ)ノ",
        "♪(^∇^*)",
        "(づ｡◕‿‿◕｡)づ",
        "(ノ´ヮ`)ノ*: ・゚✧",
        "✧･ﾟ: *✧･ﾟ:* 　*:･ﾟ✧*:･ﾟ✧",
    },
}

function lice5.party.start()
    lice5.party.active = true
    void.echo("-!- PARTY MODE ACTIVATED! \\o/")
    void.echo("-!- Disco colors enabled! Messages get rainbow treatment!")
    void.echo("-!- Type /party to send a random party line")
end

function lice5.party.stop()
    lice5.party.active = false
    void.echo("-!- Party mode deactivated. Back to boring mode.")
end

function lice5.party.disco(text)
    -- Apply disco colors to text — each word gets a different color
    if not lice5.party.active then return text end
    local words = {}
    local color_idx = 1
    for word in text:gmatch("%S+") do
        local color = lice5.party.disco_colors[color_idx]
        table.insert(words, color .. word .. "\x03")
        color_idx = (color_idx % #lice5.party.disco_colors) + 1
    end
    return table.concat(words, " ")
end

function lice5.party.random_line()
    return lice5.party.party_lines[math.random(1, #lice5.party.party_lines)]
end

-- Hook: apply disco colors to outgoing messages
void.on("PUBLIC", "lice5_party_disco")
function lice5_party_disco(args)
    if not lice5.party.active then return end
    -- The disco effect is visual only — we can't modify outgoing messages
    -- But we can echo party vibes
end

-- Command: /party [on|off|<text>]
void.register_command("PARTY", "lice5_cmd_party")
function lice5_cmd_party(args)
    if #args == 0 then
        if lice5.party.active then
            -- Send a random party line
            local line = lice5.party.random_line()
            local channel = void.channel()
            if channel ~= "" then
                void.msg(channel, line)
            else
                void.echo(line)
            end
        else
            lice5.party.start()
        end
    elseif args[1] == "on" then
        lice5.party.start()
    elseif args[1] == "off" then
        lice5.party.stop()
    elseif args[1] == "disco" then
        -- Send disco-colored text
        local text = table.concat(args, " ", 2)
        local channel = void.channel()
        if channel ~= "" then
            void.msg(channel, lice5.party.disco(text))
        end
    else
        -- Send party text with disco colors
        local text = table.concat(args, " ")
        local channel = void.channel()
        if channel ~= "" then
            void.msg(channel, lice5.party.disco(text))
        end
    end
end

-- Command: /disco <text> — shortcut for disco colors
void.register_command("DISCO", "lice5_cmd_disco")
function lice5_cmd_disco(args)
    if #args == 0 then
        void.echo("-!- Usage: /disco <text> — apply rainbow disco colors")
        return
    end
    local text = table.concat(args, " ")
    local channel = void.channel()
    if channel ~= "" then
        void.msg(channel, lice5.party.disco(text))
    end
end

-- Command: /dance — send random dance moves
void.register_command("DANCE", "lice5_cmd_dance")
function lice5_cmd_dance(args)
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel to dance!")
        return
    end
    local moves = {
        "┏(°.°)┛",
        "┗(°.°)┓",
        "ヽ(>∀<☆)ノ",
        "(ノ´ヮ`)ノ*: ・゚✧",
        "╰(*°▽°*)╯",
        "( ˘▽˘)っ♨",
        "ヽ(✿ﾟ▽ﾟ)ノ",
        "♪(^∇^*)",
        "(づ｡◕‿‿◕｡)づ",
        "\\o/ \\o/ \\o/",
    }
    void.msg(channel, moves[math.random(1, #moves)])
end
