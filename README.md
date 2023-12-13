# Faster win explorer
[![build](https://github.com/Khirath-bit/win-expl/actions/workflows/build.yml/badge.svg)](https://github.com/Khirath-bit/win-expl/actions/workflows/build.yml)


## TODOS
- Features:
    - search help dialog with subcommands
    - icons for common file types and folders
    - finish status bar
    - change back to last folder and add upp button to move to parent folder
- Design
    - hook onto window events to color stuff
- tests
- Optimization
    - comments
    - index of folder that are created by e.g. system because checkup takes too much time
    - currently a depth first search is executed on search, maybe breadth first search and yield results after every depth iteration?
    - keep every component multiple times in app. as component, as clickable, as menuable etc so event handler is more readable and dynamic
- error handling
- split app state & cache
- doc
- add a logger

# License
MIT. See license file.