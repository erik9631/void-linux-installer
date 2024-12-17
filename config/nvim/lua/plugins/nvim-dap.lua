-- More thorough check for working rust-gdb
local function has_working_rust_gdb()
  if vim.fn.has('win32') == 1 then
    return false  -- On Windows, we'll always use our custom setup
  end
  return vim.fn.executable('rust-gdb') == 1
end

return {
  setup = function()
    local dap = require("dap")
    -- Get the directory of the current script
    local current_dir = debug.getinfo(1).source:match("@?(.*/)"):sub(1, -2)
    local script_path = current_dir .. "/pretty_print.py"
    
    if not has_working_rust_gdb() then
      dap.adapters.gdb = {
        type = "executable",
        command = "gdb",
        args = { "--interpreter=dap", "-iex", "source " .. script_path }
      }
    else
      dap.adapters.gdb = {
        type = "executable",
        command = "rust-gdb",
        args = { "--interpreter=dap" }
      }
    end

    dap.configurations.rust = {
      {
        name = "Debug Rust",
        type = "gdb",
        request = "launch",
        program = "./target/debug/${workspaceFolderBasename}",
        cwd = "${workspaceFolder}",
      },
    }
  end
}