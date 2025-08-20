import { Route, Switch } from "wouter";

import { GameTypesContext, useGameTypesSWR } from "../hooks/use-game-types";
import AttackOptimizerPage from "../pages/attack-optimizer";
import ComposeBaseImagesPage from "../pages/compose-base-images";
import IndexPage from "../pages/index";
import MapEditorPage from "../pages/map-editor";
import NotFoundPage from "../pages/not-found";
import ShowcasePage from "../pages/showcase";

const App: React.FC = () => {
  const gameTypes = useGameTypesSWR();

  return gameTypes === undefined ? (
    "Loading..."
  ) : (
    <GameTypesContext value={gameTypes}>
      <Switch>
        <Route path="/" component={IndexPage} />
        <Route path="compose-base-images" component={ComposeBaseImagesPage} />
        <Route path="map-editor" component={MapEditorPage} />
        <Route path="showcase" component={ShowcasePage} />
        <Route path="attack-optimizer" component={AttackOptimizerPage} />
        <Route component={NotFoundPage} />
      </Switch>
    </GameTypesContext>
  );
};

export default App;
