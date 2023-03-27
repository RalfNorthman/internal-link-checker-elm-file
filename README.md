# Internal Link Checker

A _sketch_ of a program that checks for link inconsistencies in the documentation in an .elm-file:
- Different strings in link title and anchor link. 
- Anchor link that doesn't have a matching exposed type or function.

There are many cases it doesn't cover: e.g. correct links to other files will show up as errors.