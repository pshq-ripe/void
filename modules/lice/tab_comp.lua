-- LiCe5: Tab Completion Enhancement
-- Enhanced tab completion with sub-command support

lice5.tab_comp = {
    last_partial = "",
    last_completions = {},
    last_index = 0,
}

-- Command: /tabcomp — show tab completion status
void.register_command("TABCOMP", "lice5_cmd_tabcomp")
function lice5_cmd_tabcomp(args)
    void.echo("-!- Tab completion: active (built-in)")
    void.echo("-!- Press TAB to complete nicks, /commands, and sub-commands")
end
