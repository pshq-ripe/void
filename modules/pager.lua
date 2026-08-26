-- LiCe5: Pager — In-client file pager
-- Display files page by page

lice5.pager = {
    buffer = {},
    page_size = 20,
    current_page = 1,
}

function lice5.pager.load(filename)
    local content = void.file_read(filename)
    if content:sub(1, 5) == "Error" then
        void.echo("-!- " .. content)
        return
    end
    lice5.pager.buffer = {}
    for line in content:gmatch("[^\n]+") do
        table.insert(lice5.pager.buffer, line)
    end
    lice5.pager.current_page = 1
    lice5.pager.show_page()
end

function lice5.pager.show_page()
    local start = (lice5.pager.current_page - 1) * lice5.pager.page_size + 1
    local finish = math.min(start + lice5.pager.page_size - 1, #lice5.pager.buffer)
    local total_pages = math.ceil(#lice5.pager.buffer / lice5.pager.page_size)

    void.echo("-!- Page " .. lice5.pager.current_page .. "/" .. total_pages .. " (" .. #lice5.pager.buffer .. " lines)")
    for i = start, finish do
        void.echo(lice5.pager.buffer[i])
    end
end

function lice5.pager.next()
    local total_pages = math.ceil(#lice5.pager.buffer / lice5.pager.page_size)
    if lice5.pager.current_page < total_pages then
        lice5.pager.current_page = lice5.pager.current_page + 1
        lice5.pager.show_page()
    else
        void.echo("-!- End of file.")
    end
end

function lice5.pager.prev()
    if lice5.pager.current_page > 1 then
        lice5.pager.current_page = lice5.pager.current_page - 1
        lice5.pager.show_page()
    else
        void.echo("-!- Beginning of file.")
    end
end

-- Command: /pager <filename> or /pager next/prev
void.register_command("PAGER", "lice5_cmd_pager")
function lice5_cmd_pager(args)
    if #args == 0 then
        if #lice5.pager.buffer == 0 then
            void.echo("-!- Usage: /pager <filename>")
        else
            lice5.pager.show_page()
        end
        return
    end
    if args[1] == "next" or args[1] == "n" then
        lice5.pager.next()
    elseif args[1] == "prev" or args[1] == "p" then
        lice5.pager.prev()
    else
        lice5.pager.load(args[1])
    end
end
