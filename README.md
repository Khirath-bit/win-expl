# Faster win explorer
[![build](https://github.com/Khirath-bit/win-expl/actions/workflows/build.yml/badge.svg)](https://github.com/Khirath-bit/win-expl/actions/workflows/build.yml)
[![tests - search engine](https://github.com/Khirath-bit/win-expl/actions/workflows/run-tests-search-engine.yml/badge.svg)](https://github.com/Khirath-bit/win-expl/actions/workflows/run-tests-search-engine.yml)


## TODOS
- Features:
    - search help dialog with subcommands
    - icons for common file types and folders
    - finish status bar
    - file preview
- Design
    - hook onto window events to color stuff
- tests
- Optimization
    - comments
    - index of folder that are created by e.g. system because checkup takes too much time
    - currently a depth first search is executed on search, maybe breadth first search and yield results after every depth iteration?
    - ~~keep every component multiple times in app. as component, as clickable, as menuable etc so event handler is more readable and dynamic~~ Note: Maybe with some types but not really good
- error handling
- doc
- add a logger

# License
MIT. See license file.
