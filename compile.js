const fs = require("fs");
const path = require("path");
const mustache = require("mustache");
const rimraf = require("rimraf");
const { getScripts } = require("./view");

rimraf.sync("dist/*");

const input = fs.readFileSync(path.join(__dirname, "home.mustache"), "utf-8");
const html = mustache.render(input, getScripts());
fs.writeFileSync(path.join(__dirname, "dist/index.html"), html);

module.exports = { getScripts };
