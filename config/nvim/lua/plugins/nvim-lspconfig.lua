return {
  setup = function()
    local lspconfig = require('lspconfig')
   
    
    
    lspconfig.rust_analyzer.setup {
      -- Server-specific settings. See `:help lspconfig-setup`
      settings = {
        ['rust-analyzer'] = {},
      },
    }
    lspconfig.lua_ls.setup {
      -- Server-specific settings. See `:help lspconfig-setup`
      settings = {},
    }
    

  end
}