return {
    setup = function()
        local cmp = require("cmp")
        local capabilities = require('cmp_nvim_lsp').default_capabilities()

        -- Set up Rust Analyzer with capabilities
        require('lspconfig').rust_analyzer.setup({
            capabilities = capabilities,
        })
        
        -- Set up Lua LSP with capabilities
        require('lspconfig').lua_ls.setup({
            capabilities = capabilities,
            settings = {
                Lua = {
                    diagnostics = {
                        -- Get the language server to recognize the `vim` global
                        globals = {'vim'},
                    },
                    workspace = {
                        -- Make the server aware of Neovim runtime files
                        library = vim.api.nvim_get_runtime_file("", true),
                        checkThirdParty = false, -- Disable third party checking
                    },
                    -- Do not send telemetry data
                    telemetry = {
                        enable = false,
                    },
                },
            },
        })

        cmp.setup({
            window = {
                completion = cmp.config.window.bordered(),
                documentation = cmp.config.window.bordered(),
            },
            mapping = cmp.mapping.preset.insert({
                ["<C-b>"] = cmp.mapping.scroll_docs(-4),
                ["<C-f>"] = cmp.mapping.scroll_docs(4),
                ["<C-Space>"] = cmp.mapping.complete(),
                ["<C-e>"] = cmp.mapping.abort(),
                ["<CR>"] = cmp.mapping.confirm({ select = true }), -- Accept currently selected item. Set `select` to `false` to only confirm explicitly selected items.
            }),
            sources = cmp.config.sources({
                { name = "nvim_lsp" },
                { name = "path" },   -- Add path source
            }, {
                { name = "buffer" },
            }),
        })
    end,
}