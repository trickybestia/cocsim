import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Route, Switch } from "wouter";

import "./index.css";
import Index from "./pages/index";
import NotFound from "./pages/not-found";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Switch>
      <Route path="/" component={Index} />
      <Route component={NotFound} />
    </Switch>
  </StrictMode>
);
