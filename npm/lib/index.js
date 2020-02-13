"use strict";

const path = require("path");

module.exports.svgminiPath = path.join(
  __dirname,
  `../bin/svgmini${process.platform === "win32" ? ".exe" : ""}`
);
