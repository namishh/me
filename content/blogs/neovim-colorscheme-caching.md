---
draft: false
title: Easy Neovim Colorscheme Caching
description: Easily Learn How To Create Custom Colorschmes With Caching
date: 05 Sep 2023
author: Namish 
category: talks
---

![img](/caching.png)
[photos from my config](https://github.com/chadcat7/kodo)

## Why Do I Need This

I have always implemented my own way of having colorschemes, earlier I used to do something like this

```lua  title="highlights.lua" 
local highlights = require('themes.highlights')

local function setup(opts)
  if opts == nil then opts = { theme = "nirvana" } end
  local colors = require('themes.colorschemes.' .. opts.theme).get_colors()
  highlights.highlight_all(colors, opts)
end

return { setup = setup }
```

The main thing here is the line `local colors = require('themes.colorschemes.' .. opts.theme).get_colors()`. Observe that we have to `require` the theme file everytime. Instead of doing this everytime we can compile the colors into a cache file so that it can be called faster the next time. This is what we will try to implement in this blog. At the end I will also teach you how to make a custom telescope prompt for selecting your custom themes

## Setup

In this setup we will create some files required for this. Make a directory `.config/nvim/lua/themes`. Throughout this blog, all the files would be in this directory


### init.lua

```lua
// init.lua
vim.g.currentTheme = "onedark"

local M = {}

local function hexToRgb(c)
  c = string.lower(c)
  return { tonumber(c:sub(2, 3), 16), tonumber(c:sub(4, 5), 16), tonumber(c:sub(6, 7), 16) }
end

-- some functions to make light and dark colors

M.blend = function(foreground, background, alpha)
  alpha = type(alpha) == "string" and (tonumber(alpha, 16) / 0xff) or alpha
  local bg = hexToRgb(background)
  local fg = hexToRgb(foreground)

  local blendChannel = function(i)
    local ret = (alpha * fg[i] + ((1 - alpha) * bg[i]))
    return math.floor(math.min(math.max(0, ret), 255) + 0.5)
  end

  return string.format("#%02x%02x%02x", blendChannel(1), blendChannel(2), blendChannel(3))
end

M.darken = function(hex, bg, amount)
  return M.blend(hex, bg, amount)
end

M.lighten = function(hex, fg, amount)
  return M.blend(hex, fg, amount)
end

return M
```

Explaination of the blend function:

+ It takes in three parametres:
    1. foreground : `string` \- in the form of hex
    2. background : `string` \- also in the form of hex
    3. alpha : `float` \- between 0 and 1, representing the blending alpha or transparency level.
+ The purpose of this function is to blend the foreground color over the background color with the given transparency level (alpha). 
+ If alpha is a string, it assumes it's a hexadecimal color string and converts it to a numeric value by dividing it by `0xff (255)`. If alpha is a number, it uses it as is.
+ It then converts both the foreground and background colors from hexadecimal format to `RGB` format using a helper function called `hexToRgb` which is responsible for converting a hexadecimal color string to an `RGB` color representation.
+ The function defines another inner function called `blendChannel(i)` which blends the color channels (red, green, and blue) individually based on the alpha value and returns the blended channel value. It ensures that the resulting channel value is clamped between 0 and 255 and rounds it to the nearest integer.
+ Finally, the function constructs a new hexadecimal color string using the blended `RGB` color channels and returns it.

<br/>

Finally we will also have a function to fetch the themes colors for setting the highlights

```lua 
M.getCurrentTheme = function()
  local path = "themes.schemes." .. vim.g.currentTheme
  local theme = require(path).get_colors()
  return theme
end
``` 

### scheme

let us make a file at `lua/themes/schemes/onedark.lua`

```lua 
// themes/schemes/onedark.lua
local M = {}

function M.get_colors()
  return {
    background = "#181b21",
    black = "#181b21",
    darker = '#111418',
    foreground = "#dcdee6",

    cursor = "#dcdee6",
    comment = '#79818f',

    contrast = '#1c1f26',
    cursorline = '#1c1f26',

    color0 = "#272b33",
    color1 = "#c75f68",
    color2 = "#60ae7f",
    color3 = "#cb795f",
    color4 = "#7095db",
    color5 = "#b475c6",
    color6 = "#63b0b9",
    color7 = "#abb2bf",
    color8 = "#2d3139",
    color9 = "#e0626c",
    color10 = "#6bbd8c",
    color11 = "#d9846a",
    color12 = "#7ca0e3",
    color13 = "#bf75d4",
    color14 = "#6ec0cb",
    color15 = "#abb2bf",
    comment_light = "#9096a1",
  }
end

return M
``` 

to get more colorschemes, check out my [repo](https://github.com/chadcat7/kodo/tree/main/lua/themes/schemes)

### highlights

for highlights, we will create folder `lua/themes/hls`. In later sections, we will create a function that will loop through all the files in the repo, so you can name files in it however you want. But each file should return a table with highlights. For example for the dashboard \- 

```lua 
// themes/hls/alpha.lua
local themes = require("themes")
local colors = themes.getCurrentTheme()

return {
  AlphaHeader = { fg = colors.color4, bg = colors.background },
  AlphaLabel = { fg = colors.color7, bg = colors.background },
  AlphaIcon = { fg = colors.color5, bold = true, },
  AlphaKeyPrefix = { fg = colors.color1, bg = themes.darken(colors.color1, colors.background, 0.04) },
  AlphaMessage = { fg = colors.color2, bg = colors.background },
  AlphaFooter = { fg = colors.comment, bg = colors.background },
}
``` 

Instead of manually typing out all the highlights, get them all from [here](https://github.com/chadcat7/kodo/tree/main/lua/themes/hls)

## implementation

now let us actually start implementing what we were here for, `colorscheme caching`. All of this code is in `themes/init.lua`

```lua 
// themes/init.lua 

-- simple helper functions
M.mergeTb         = function(...)
  return vim.tbl_deep_extend("force", ...)
end

M.loadTb          = function(g)
  g = require("themes.hls." .. g)
  return g
end
``` 

Explaination: 

+ `M.mergeTb`: This function merges multiple tables into one using `vim.tbl_deep_extend("force", ...)`. It takes any number of tables as arguments and returns a single merged table.

+ `M.loadTb`: This function loads a highlights table from a file.

### making cache

this function creates the cache and saves the file at `~/.local/share/nvim/colors_data/`

```lua 
// themes/init.lua 

vim.g.theme_cache = vim.fn.stdpath "data" .. "/colors_data/"

M.tableToStr      = function(tb)
  local result = ""

  for hlgroupName, hlgroup_vals in pairs(tb) do
    local hlname = "'" .. hlgroupName .. "',"
    local opts = ""

    for optName, optVal in pairs(hlgroup_vals) do
      local valueInStr = ((type(optVal)) == "boolean" or type(optVal) == "number") and tostring(optVal)
          or '"' .. optVal .. '"'
      opts = opts .. optName .. "=" .. valueInStr .. ","
    end

    result = result .. "vim.api.nvim_set_hl(0," .. hlname .. "{" .. opts .. "})"
  end

  return result
end

M.toCache         = function(filename, tb)
  local lines = "return string.dump(function()" .. M.tableToStr(tb) .. "end, true)"
  local file = io.open(vim.g.theme_cache .. filename, "wb")

  if file then
    ---@diagnostic disable-next-line: deprecated
    file:write(loadstring(lines)())
    file:close()
  end
end
``` 

Explaination: 

+ `M.tableToStr`: This function converts a table of color scheme definitions into a string that can be written to a Lua file. It iterates over the table and its sub-tables, converting the data into a format suitable for use with `vim.api.nvim_set_hl`.

+ `M.toCache`: This function takes a filename and a table of color scheme definitions, converts the table into a Lua code string using `M.tableToStr`, and writes this code to a cache file. The resulting file is written in binary mode to the path specified by `vim.g.theme_cache`.

### applying

```lua
// themes/init.lua 
local hl_files = vim.fn.stdpath "config" .. "/lua/themes/hls"

M.compile         = function()
  if not vim.loop.fs_stat(vim.g.theme_cache) then
    vim.fn.mkdir(vim.g.theme_cache, "p")
  end

  for _, file in ipairs(vim.fn.readdir(hl_files)) do
    local filename = vim.fn.fnamemodify(file, ":r")
    M.toCache(filename, M.loadTb(filename))
  end
end

M.load            = function()
  M.compile()
  for _, file in ipairs(vim.fn.readdir(vim.g.theme_cache)) do
    dofile(vim.g.theme_cache .. file)
  end
end
``` 

Explaination:

+ `M.compile`: This function compiles color scheme definitions into cache files. It checks if the cache directory specified in `vim.g.theme_cache` exists and creates it if it doesn't. It then iterates through a list of color scheme files, converts them to Lua code using `M.toCache`, and writes them to cache files.

+ `M.load`: This function loads cached color scheme definitions into Neovim. It first compiles the color schemes using `M.compile` and then iterates through the cache files in the cache directory, using `dofile` to load each one into Neovim

This is it basically, now the 

## last step

create a file `~/.config/nvim/colors/onedark.lua`

```lua
// colors/onedark.lua
vim.g.currentTheme = "onedark"
require("plenary.reload").reload_module "themes"
require("themes").load()
``` 

and now just run this command: 

```
:colorscheme onedark
``` 

## bonus - telescope picker

now if you have multiple schemes in `themes/schemes`, we can create a telescope picker for just them. For this you will first need to install [Telescope](https://github.com/nvim-telescope/telescope.nvim)

```lua
// themes/picker.lua 
local pickers      = require "telescope.pickers"
local finders      = require "telescope.finders"
local actions      = require "telescope.actions"
local action_state = require "telescope.actions.state"
local conf         = require("telescope.config").values

local M            = {}

-- this code generates the list of all schemes in .config/nvim/colors
local schemes      = {}
local files        = vim.fn.stdpath "config" .. "/colors/"
for _, file in ipairs(vim.fn.readdir(files)) do
  local f = vim.fn.fnamemodify(file, ":r")
  table.insert(schemes, f)
end

M.setup = function(opts)
  opts = opts or {}
  -- create a new picker
  pickers.new(opts, {
    prompt_title = "Kolorschemes",
    finder = finders.new_table {
      results = schemes -- add all the schemes to the table
    },
    sorter = conf.generic_sorter(opts),
    attach_mappings = function(buffer)
      actions.select_default:replace(function()
        local theme = action_state.get_selected_entry().value -- get the selected value
        -- apply the scheme
        vim.g.currentTheme = theme
        vim.cmd("colorscheme " .. theme)
        actions.close(buffer)
      end)
      return true
    end,
  }):find()
end

return M
``` 

You can run this by 

```
:lua require("themes.pick").setup()
``` 

`Voila`, we are done!

![img](/nvimonedark.png)

## credits

+ [base46](https://github.com/NvChad/base46/)
+ [tokyonight](https://github.com/folke/tokyonight.nvim)
