-- LiCe5: News System
-- Display news/announcements

lice5.news = {
    items = {},
    last_shown = 0,
}

function lice5.news.add(text)
    table.insert(lice5.news.items, {
        text = text,
        time = os.time(),
        author = void.nick(),
    })
    void.echo("-!- News added.")
end

function lice5.news.show()
    if #lice5.news.items == 0 then
        void.echo("-!- No news.")
    else
        void.echo("-!- News:")
        for i, item in ipairs(lice5.news.items) do
            void.echo("  " .. i .. ": " .. item.text .. " (" .. item.author .. ")")
        end
    end
    lice5.news.last_shown = #lice5.news.items
end

function lice5.news.check_new()
    if #lice5.news.items > lice5.news.last_shown then
        void.echo("-!- " .. (#lice5.news.items - lice5.news.last_shown) .. " new news items. Type /news to read.")
    end
end

-- Command: /news [add <text>]
void.register_command("NEWS", "lice5_cmd_news")
function lice5_cmd_news(args)
    if #args == 0 then
        lice5.news.show()
        return
    end
    if args[1] == "add" then
        lice5.news.add(table.concat(args, " ", 2))
    else
        lice5.news.add(table.concat(args, " "))
    end
end
