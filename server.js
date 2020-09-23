const fs = require("fs");
const path = require("path");
const rimraf = require("rimraf");
const express = require("express");
const mustacheExpress = require("mustache-express");
const { getScripts } = require("./view");

const PORT = 10000;

const app = express();
app.engine("mustache", mustacheExpress());
app.set("view engine", "mustache");
app.set("views", __dirname);
app.use(express.static("dist"));

rimraf.sync("dist/*");

app.get("/", (req, res) => {
  res.render("home", {
    scripts: getScripts(),
  });
});

app.listen(PORT, () => {
  console.log(`Server listening at http://localhost:${PORT}`);
});
