local opt = vim.opt
local builtin = require('telescope.builtin')
opt.exrc = true
opt.number = true
opt.mouse = 'a'
opt.termguicolors = true
-- Set tab width to 4 spaces
opt.tabstop = 4       -- Width of tab character
opt.softtabstop = 4   -- Fine tunes amount of whitespace to be inserted
opt.shiftwidth = 4    -- Size of indent
opt.expandtab = true  -- Converts tabs to spaces

-- Additional useful settings
opt.autoindent = true    -- Copy indent from current line when starting a new line
opt.smartindent = true   -- Smart autoindenting when starting a new line
vim.cmd.colorscheme "astrodark"
opt.termguicolors = true