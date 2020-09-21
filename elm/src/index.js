import "normalize.css";
import "./main.css";
import "../../style/main.css";
import { Elm } from "./Main.elm";

function getRootWrapperSize() {
  const { height, width } = document
    .getElementById("elm-root-wrapper")
    .getBoundingClientRect();

  return { height, width };
}

const app = Elm.Main.init({
  node: document.getElementById("elm-root"),
});

const { height, width } = getRootWrapperSize();
app.ports.windowSettings.send({ height, width });

window.addEventListener("resize", function () {
  const { height, width } = getRootWrapperSize();
  app.ports.windowSettings.send({ height, width });
});
