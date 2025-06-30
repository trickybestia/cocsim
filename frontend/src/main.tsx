import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import Modal from "react-modal";
import { Route, Switch } from "wouter";

import "./index.css";
import ComposeBaseImagesPage from "./pages/compose-base-images";
import IndexPage from "./pages/index";
import MapEditorPage from "./pages/map-editor";
import NotFoundPage from "./pages/not-found";

Modal.setAppElement("#root");

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Switch>
      <Route path="/" component={IndexPage} />
      <Route path="compose-base-images" component={ComposeBaseImagesPage} />
      <Route path="map-editor" component={MapEditorPage} />
      <Route component={NotFoundPage} />
    </Switch>
  </StrictMode>
);
