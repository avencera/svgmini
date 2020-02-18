# SVGMini [![Build Status Travis](https://travis-ci.org/avencera/svgmini.svg?branch=master)](https://travis-ci.org/avencera/svgmini) [![Build status](https://ci.appveyor.com/api/projects/status/cgkccm1nn9x8r4bg/branch/master?svg=true)](https://ci.appveyor.com/project/praveenperera/svgmini/branch/master) [![npm version](https://badge.fury.io/js/svgmini.svg)](https://badge.fury.io/js/svgmini) [![vscode marketplace extension](https://vsmarketplacebadge.apphb.com/version/avencera.svgmini.svg)](https://marketplace.visualstudio.com/items?itemName=avencera.svgmini)

# Usage

Run `SVGMini: Minify SVGs in Current File` from the command palette.

---

<img src="https://github.com/avencera/svgmini/blob/master/explainer.gif?raw=true" alt="Explainer" width="750px">

## Config

### `svgmini.replaceFill`

When set SVGMini will replace all `fill` attributes in SVGs with `currentColor`. Defaults to false.

`"svgmini.replaceFill": true`

## What?

A tool to minify SVGs that are contained in other documents (for example HTML or JSX files).

## Why?

Lots of tools and plugins exist to minify SVGs for example:

- [SVGO](https://github.com/svg/svgo)
- [svgcleaner](https://github.com/RazrFalcon/svgcleaner)
- [SVG VSCode Plugin](https://marketplace.visualstudio.com/items?itemName=jock.svg)

However none of them did what I wanted, which was to minify SVGs within my HTML files.

## Issues or Suggestions?

Post in the GitHub Repo: [http://github.com/avencera/svgmini](http://github.com/avencera/svgmini)
