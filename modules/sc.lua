-- LiCe5: Sc — Screen management
-- Screen/tmux integration

lice5.sc = {}

-- Command: /sc [status|detach|attach]
void.register_command("SC", "lice5_cmd_sc")
function lice5_cmd_sc(args)
    if #args == 0 then
        void.echo("-!- Screen status: (check your terminal)")
        return
    end
    local action = args[1]:lower()
    if action == "status" then
        void.echo("-!- Screen status: (check your terminal)")
    elseif action == "detach" then
        void.echo("-!- To detach: Ctrl+A, D (in screen) or Ctrl+B, D (in tmux)")
    elseif action == "attach" then
        void.echo("-!- To attach: screen -r or tmux attach")
    end
end
