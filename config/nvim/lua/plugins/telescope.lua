return {
    setup = function()
        -- Set up telescope
        require('telescope').setup({
            defaults = {
                mappings = {
                    i = {
                        ["<C-h>"] = "which_key"
                    }
                }
            },
            pickers = {},
            extensions = {}
        })
    end
}