# xdiffer
## Overview
**xdiffer** is an efficient XML diff merge tool.

## Features
- Semantic comparision: Detect differences in XML content regardless of node order.
- Fast: Using [X-Diff algorithm](https://pages.cs.wisc.edu/~yuanwang/papers/xdiff.pdf) for efficient XML tree comparision. This tool was created with the intention of comparing ECU Extract, which is very big XML (or ARXML to be exact) file used in automotive domain. So it should be able to handle your regular XML files with ease.
- Efficient Merge Functionality: Combine changes across XML files while maintaining semantic integrity.

## Usage
The tool is deployed as a web app at: https://ndtoan96.github.io/xdiffer/

## Demo

https://github.com/user-attachments/assets/bc2ad40a-17c5-4b4e-ac37-f398f097d998

## How to merge?
It may be confusing at first working with the diff tree. But it's pretty simple. The different nodes are highlighted in the diff tree. There are 3 cases of difference:
- the node only exists in XML 1
- the node only exists in XML 2
- the node exists in both XML 1 and XML 2 but has different value

The button next to the node show its state after merging:
- XML1: include value from XML 1 in the merged XML
- XML2: include value from XML 2 in the merged XML
- None: remove it from the merged XML

![image](https://github.com/user-attachments/assets/1761a80d-cb44-43fb-b349-31cf14dc5f5a)


## Why is it a web app?
Because I cannot find a suitable GUI framework for my need (especially treeview and file drop), mostly due to skill issue though. Plus, I want to try working with web assembly. I plan to make a desktop version eventually but for now I settle with web app.
