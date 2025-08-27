import type { Frame, GameTypes } from "../types";
import api from "./api_impl";

type API = {
  composeBaseImages: (left: Blob[], right: Blob[]) => Promise<Blob>;
  reverseProjection: (image: Blob) => Promise<Blob>;
  getGameTypes: () => Promise<GameTypes>;
  getShowcaseAttackBaseImage: () => Promise<HTMLImageElement>;
  getShowcaseAttack: () => Promise<Frame[]>;
  getOptimizeAttackWebSocketUrl: () => string;
};

export default api;
export type { API };
