return {
  setup = function()
    require("astrotheme").setup({})
    vim.api.nvim_create_autocmd("ColorScheme", {
      pattern = "*",
      callback = function()
        vim.api.nvim_set_hl(0, "NvimTreeRootFolder", { fg = "#cdd6f4" })
        vim.api.nvim_set_hl(0, "NvimTreeHighlights", { fg = "#cdd6f4" })
      end,
    })
  end
}