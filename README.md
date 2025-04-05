# xdiffer
## Overview
**xdiffer** is an efficient XML diff merge tool.

## Features
- Semantic comparision: Detect differences in XML content regardless of node order.
- Fast: Using [X-Diff algorithm](https://pages.cs.wisc.edu/~yuanwang/papers/xdiff.pdf) for efficient XML tree comparision. This tool was created with the intention of comparing ECU Extract, which is very big XML (or ARXML to be exact) file used in automotive domain. So it should be able to handle your regular XML files with ease.
- Efficient Merge Functionality: Combine changes across XML files while maintaining semantic integrity.

## Usage
The tool is deployed as a web app at: https://ndtoan96.github.io/xdiffer/

## Why is it a web app?
Because I cannot find a suitable GUI framework for my need (especially treeview and file drop), mostly due to skill issue though. I plan to make a desktop version eventually but for now I settle with web app.
