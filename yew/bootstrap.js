import "normalize.css";
import "../style/main.css";

import("./pkg").then((module) => {
  module.run_app();
});
