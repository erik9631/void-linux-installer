local pickers = require('telescope.pickers')
local finders = require('telescope.finders')
local conf = require('telescope.config').values
local actions = require('telescope.actions')
local action_state = require('telescope.actions.state')
local Terminal = require('toggleterm.terminal').Terminal

local project_commands = {}  -- This needs to be outside to persist between calls
set_project_commands = function(commands)
    project_commands = commands
end


-- Create the command picker function
local function run_command_picker()
    pickers.new({}, {
        prompt_title = 'Run Command',
        finder = finders.new_table({
            results = project_commands,
            entry_maker = function(entry)
                return {
                    value = entry[2],
                    display = entry[1],
                    ordinal = entry[1],
                }
            end,
        }),
        sorter = conf.generic_sorter({}),
        attach_mappings = function(prompt_bufnr, map)
            actions.select_default:replace(function()
                actions.close(prompt_bufnr)
                local selection = action_state.get_selected_entry()
                local custom_term = Terminal:new({
                    cmd = selection.value,
                    hidden = true,
                    name = selection.display
                })
                
                -- Toggle the terminal
                custom_term:toggle()
            end
            )
            return true
        end,
    }):find()
end

return {
  setup = function()
    vim.api.nvim_create_user_command('RunCommand', run_command_picker, {})
    vim.keymap.set('n', '<leader>rc', ':RunCommand<CR>', { silent = true })
  end,
  set_project_commands = set_project_commands
}