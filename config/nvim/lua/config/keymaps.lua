local builtin = require('telescope.builtin')

vim.g.mapleader = " "  -- Set space as leader
vim.keymap.set('n', '<C-n>', ':NvimTreeToggle<CR>', { noremap = true, silent = true })
vim.keymap.set('n', '<C-s>', ':w<CR>', { noremap = true, silent = true, desc = 'Save file' })
vim.keymap.set('i', '<C-s>', '<Esc>:w<CR>', { noremap = true, silent = true, desc = 'Save file' })
vim.keymap.set('n', '<C-f>', builtin.find_files, { desc = 'Find Files' })
vim.keymap.set('n', '<C-g>', builtin.live_grep, { desc = 'Find in files' })
vim.keymap.set('n', 'gD', vim.lsp.buf.declaration, { desc = 'Go to declaration' })
vim.keymap.set('n', 'gd', vim.lsp.buf.definition, { desc = 'Go to definition' })
vim.keymap.set('n', 'K', vim.lsp.buf.hover, { desc = 'Hover Documentation' })
vim.keymap.set('n', 'gi', vim.lsp.buf.implementation, { desc = 'Go to implementation' })
vim.keymap.set({'n', 'i'}, '<C-k>', vim.lsp.buf.signature_help, { desc = 'Signature Documentation' })
vim.keymap.set('n', '<leader>wa', vim.lsp.buf.add_workspace_folder, { desc = 'Workspace Add Folder' })
vim.keymap.set('n', '<leader>wr', vim.lsp.buf.remove_workspace_folder, { desc = 'Workspace Remove Folder' })
vim.keymap.set('n', '<leader>f', vim.lsp.buf.format, { desc = 'Format file' })
vim.keymap.set('v', '<leader>f', vim.lsp.buf.format, { desc = 'Format selection' })
vim.keymap.set('n', '<leader>fp', ':Telescope projects<CR>', { desc = 'Find projects' })
vim.keymap.set('n', '<leader>rc', ':RunCommand<CR>', { silent = true })
vim.keymap.set('n', '<leader>q', ':q<CR>', { desc = 'Quit current window' })

vim.keymap.set('n', '<Leader>b', function() require('dap').toggle_breakpoint() end)
vim.keymap.set('n', '<F9>', function() require('dap').continue() end)
vim.keymap.set('n', '<F8>', function() require('dap').step_over() end)
vim.keymap.set('n', '<F7>', function() require('dap').step_into() end)
vim.keymap.set('n', '<S-F7>', function() require('dap').step_out() end)
-- Or if you want to terminate the debugging session as well
vim.keymap.set('n', '<S-F5>', function()
    require('dap').terminate()
    require('dap').repl.close()
    require('dapui').close()  -- if you're using nvim-dap-ui
end)


vim.keymap.set('n', '<leader>gs', ':Neogit<CR>', { desc = "Open Neogit Status" })
vim.keymap.set('n', '<leader>gc', ':Neogit commit<CR>', { desc = "Neogit Commit" })
vim.keymap.set('n', '<leader>gp', ':Neogit pull<CR>', { desc = "Neogit Pull" })
vim.keymap.set('n', '<leader>gP', ':Neogit push<CR>', { desc = "Neogit Push" })