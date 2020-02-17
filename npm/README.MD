# SVGMini

## Usage

### Extension

**[Get it from the VS Code Marketplace →](https://marketplace.visualstudio.com/items?itemName=avencera.svgmini)**

Run `SVGMini: Minify SVGs in Current File` from the command palette.

---

<img src="https://github.com/avencera/svgmini/blob/master/explainer.gif?raw=true" alt="Explainer" width="750px">

### Config

#### `svgmini.replaceFill`

When set SVGMini will replace all `fill` attributes in SVGs with `currentColor`. Defaults to false.

`"svgmini.replaceFill": true`

## CLI

### Install

Install via npm

`yarn global add svgmini`

or

`npm install -g svgmini`

or

Install from a github release:

`curl -LSfs https://avencera.github.io/svgmini/install.sh | sh -s -- --git avencera/svgmini`

or

Download a release directly from github: [github.com/avencera/svgmini/releases](https://github.com/avencera/svgmini/releases)

### Run

Run on a file `svgmini path/to_repo/index.html`

### Config

`--replace-fill`

When set SVGMini will replace all `fill` attributes in SVGs with `currentColor`. Defaults to false.

## What?

A tool to minify SVGs that are contained in other documents (for example HTML or JSX files).

## Why?

Lots of tools and plugins exist to minify SVGs for example:

- [SVGO](https://github.com/svg/svgo)
- [svgcleaner](https://github.com/RazrFalcon/svgcleaner)
- [SVG VSCode Plugin](https://marketplace.visualstudio.com/items?itemName=jock.svg)

However none of them did what I wanted, which was to minify SVGs within my HTML files.
