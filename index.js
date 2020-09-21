const fs = require("fs");
const path = require("path");
const express = require("express");
const mustacheExpress = require("mustache-express");

const PORT = 10000;

const app = express();
app.engine("mustache", mustacheExpress());
app.set("view engine", "mustache");
app.set("views", __dirname);
app.use(express.static("dist"));

app.get("/", (req, res) => {
  const elmPath = "./elm/build/static/js";
  const elmFiles = readFiles(elmPath, "elm");
  copyFiles(elmFiles);

  const yewPath = "./yew/dist";
  const yewFiles = readFiles(yewPath, "yew");
  copyFiles(yewFiles);

  res.render("home", {
    scripts: elmFiles
      .concat(yewFiles)
      .map((f) => path.join(f.publicPath, f.name)),
  });
});

app.listen(PORT, () => {
  console.log(`Example app listening at http://localhost:${PORT}`);
});

function readFiles(filesPath, publicPath) {
  return fs
    .readdirSync(filesPath)
    .filter((s) => s.endsWith(".js") || s.endsWith(".wasm"))
    .map((s) => ({
      name: s,
      publicPath,
      distPath: path.join("dist", publicPath),
      path: path.join(filesPath, s),
    }));
}

function copyFiles(files) {
  fs.mkdirSync(files[0].distPath, { recursive: true });

  files.forEach((f) => fs.copyFileSync(f.path, path.join(f.distPath, f.name)));
}
