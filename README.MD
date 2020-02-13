# SVGMini

## Usage (comin soon...)

### Plugin

1. Download the VSCode Plugin
2. Run with `SVGMini - Minify SVGs`

### CLI

#### Install
Install via npm

`yarn global add svgmini`

or

`npm install -g svgmini`

or

Install from a github release:

`curl -LSfs https://avencera.github.io/svgmini/install.sh | sh -s -- --git avencera/svgmini`

or

Download a release directly from github: github.com/avencera/svgmini/releases

#### Run

Run on a file `svgmini path/to_repo/index.html`

## What?

A tool to minify SVGs that are contained in other documents (for example HTML or JSX files).

## Why?

Lots of tools and plugins exist to minify SVGs for example: 

* [SVGO](https://github.com/svg/svgo)
* [svgcleaner](https://github.com/RazrFalcon/svgcleaner)
* [SVG VSCode Plugin](https://marketplace.visualstudio.com/items?itemName=jock.svg)

However none of them did what I wanted, which was to minify SVGs within my HTML files.
