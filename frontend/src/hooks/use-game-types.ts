import { createContext } from "react";
import useSWR from "swr";

import { getGameTypes } from "../api";
import type { GameTypes } from "../types";

const GameTypesContext = createContext<GameTypes>({
  buildings: [],
  units: [],
  spells: []
});

const useGameTypesSWR = (): GameTypes | undefined => {
  const { data } = useSWR("get-game-types", getGameTypes, {
    revalidateIfStale: false,
    revalidateOnFocus: false,
    revalidateOnReconnect: false
  });

  return data;
};

export { GameTypesContext, useGameTypesSWR };
