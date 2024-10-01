import React from "react";
import ReactDOM from "react-dom/client";
import { HotkeysProvider } from "@blueprintjs/core";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <HotkeysProvider>
      <App />
    </HotkeysProvider>
  </React.StrictMode>,
);
