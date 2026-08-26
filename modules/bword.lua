-- LiCe5: Bword — Word manipulation
-- String/word manipulation utilities

lice5.bword = {}

function lice5.bword.get_word(text, n)
    local words = {}
    for word in text:gmatch("%S+") do
        table.insert(words, word)
    end
    return words[n] or ""
end

function lice5.bword.word_count(text)
    local count = 0
    for _ in text:gmatch("%S+") do
        count = count + 1
    end
    return count
end

function lice5.bword.remove_word(text, n)
    local words = {}
    for word in text:gmatch("%S+") do
        table.insert(words, word)
    end
    table.remove(words, n)
    return table.concat(words, " ")
end

function lice5.bword.insert_word(text, n, word)
    local words = {}
    for w in text:gmatch("%S+") do
        table.insert(words, w)
    end
    table.insert(words, n, word)
    return table.concat(words, " ")
end

-- Command: /bword <text> — word manipulation demo
void.register_command("BWORD", "lice5_cmd_bword")
function lice5_cmd_bword(args)
    if #args == 0 then
        void.echo("-!- Usage: /bword <text> — word manipulation utilities")
        void.echo("-!- Functions: get_word, word_count, remove_word, insert_word")
        return
    end
    local text = table.concat(args, " ")
    void.echo("-!- Words: " .. lice5.bword.word_count(text))
    void.echo("-!- First: " .. lice5.bword.get_word(text, 1))
    void.echo("-!- Last: " .. lice5.bword.get_word(text, lice5.bword.word_count(text)))
end
