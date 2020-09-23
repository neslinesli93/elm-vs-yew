const fs = require("fs");
const path = require("path");

function getScripts() {
  const elmPath = "./elm/build/static/js";
  const elmFiles = readFiles(elmPath, "elm");
  copyFiles(elmFiles);

  const yewPath = "./yew/dist";
  const yewFiles = readFiles(yewPath, "yew");
  copyFiles(yewFiles);

  const cssFiles = readFiles("./style", "style");
  copyFiles(cssFiles, true);

  return elmFiles
    .concat(yewFiles)
    .filter((f) => filesForTemplate(f.name))
    .map((f) => path.join(f.publicPath, f.name));
}

function readFiles(filesPath, publicPath) {
  return fs.readdirSync(filesPath).map((s) => ({
    name: s,
    publicPath,
    distPath: path.join("dist", publicPath),
    path: path.join(filesPath, s),
  }));
}

function copyFiles(files, skipFilter) {
  fs.mkdirSync(files[0].distPath, { recursive: true });

  files
    .filter((f) => skipFilter || filesForApplication(f.name))
    .forEach((f) => fs.copyFileSync(f.path, path.join(f.distPath, f.name)));
}

function filesForApplication(f) {
  return f.endsWith(".js") || f.endsWith(".wasm");
}

function filesForTemplate(f) {
  return (f.startsWith("elm") || f.startsWith("yew")) && f.endsWith(".js");
}

module.exports = { getScripts };
