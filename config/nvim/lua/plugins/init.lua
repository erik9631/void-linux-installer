return {
    -- Icons
    {
        "nvim-tree/nvim-web-devicons",
        config = function()
            require("nvim-web-devicons").setup()
        end
    },
    
    -- File Explorer
    {
        "nvim-tree/nvim-tree.lua",
        dependencies = { "nvim-tree/nvim-web-devicons" },
        config = function()
            require("plugins.nvim-tree").setup()
        end
    },

    -- Surround
    {
        "kylechui/nvim-surround",
        config = function()
            require("nvim-surround").setup()
        end
    },

    -- Required by many plugins
    "nvim-lua/plenary.nvim",

    -- Telescope
    {
        "nvim-telescope/telescope.nvim",
        dependencies = {
            "nvim-lua/plenary.nvim",
            "nvim-telescope/telescope-ui-select.nvim"
        },
        config = function()
            require("plugins.telescope").setup()
            require("plugins.telescope-ui-select").setup()
        end
    },

    -- Treesitter
    {
        "nvim-treesitter/nvim-treesitter",
        build = ":TSUpdate",
        config = function()
            require("plugins.nvim-treesitter").setup()
        end
    },

    -- Theme
    {
        "AstroNvim/astrotheme",
        config = function()
            require("plugins.astro").setup()
        end
    },

    -- Status Line
    {
        "nvim-lualine/lualine.nvim",
        dependencies = { "nvim-tree/nvim-web-devicons" },
        config = function()
            require("plugins.lua-line").setup()
        end
    },

    -- Terminal
    {
        "akinsho/toggleterm.nvim",
        config = function()
            require("plugins.toggleterm").setup()
        end
    },

    -- LSP
    {
        "williamboman/mason.nvim",
        config = function()
            require("plugins.mason").setup()
        end
    },
    {
        "williamboman/mason-lspconfig.nvim",
        config = function()
            require("plugins.mason-lspconfig").setup()
        end
    },
    {
        "neovim/nvim-lspconfig",
        config = function()
            require("plugins.nvim-lspconfig").setup()
        end
    },

    -- Formatting
    {
        "nvimtools/none-ls.nvim",
        config = function()
            require("plugins.null-ls").setup()
        end
    },

    -- Dashboard
    {
        "goolord/alpha-nvim",
        config = function()
            require("plugins.alpha-nvim").setup()
        end
    },

    -- Completion
    {
        "hrsh7th/nvim-cmp",
        dependencies = {
            "hrsh7th/cmp-nvim-lsp",
            "hrsh7th/cmp-path",
        },
        config = function()
            require("plugins.nvim-cmp").setup()
        end
    },

    -- Debugging
    {
        "mfussenegger/nvim-dap",
        config = function()
            require("plugins.nvim-dap").setup()
        end
    },
    {
        "rcarriga/nvim-dap-ui",
        dependencies = {
            "mfussenegger/nvim-dap",
            "nvim-neotest/nvim-nio"
        },
        config = function()
            require("plugins.dapui").setup()
        end
    },

    {
        "NeogitOrg/neogit",
        dependencies = {
            "nvim-lua/plenary.nvim",
            "nvim-telescope/telescope.nvim", -- optional but recommended
            "sindrets/diffview.nvim",        -- optional but recommended
        },
        config = function()
            require("plugins.neogit").setup()
        end
    },

    --Run menu like in jetbrains
    --[[
    {
        dir = vim.fn.stdpath("config") .. "/lua/plugins/local/project-menu.lua", --plugins.local.project-menu,
        config = function()
            require("plugins.local.project-menu").setup()
        end
    }, ]]
}
