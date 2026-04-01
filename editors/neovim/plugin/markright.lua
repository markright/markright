local parser_config = require("nvim-treesitter.parsers").get_parser_configs()
parser_config.markright = {
  install_info = {
    url = "https://github.com/markright/markright",
    files = { "tree-sitter-markright/src/parser.c" },
    branch = "master",
  },
  filetype = "markright",
}

vim.filetype.add({
  extension = {
    right = "markright",
  },
})

vim.api.nvim_create_autocmd("FileType", {
  pattern = "markright",
  callback = function()
    vim.lsp.start({
      name = "markright-lsp",
      cmd = { "markright-lsp" },
    })
  end,
})
