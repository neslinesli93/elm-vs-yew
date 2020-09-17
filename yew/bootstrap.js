import "normalize.css";
import "./static/style.scss";

import("./pkg").then((module) => {
  module.run_app();
});
