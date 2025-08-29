import type { Frame, GameTypes } from "../types";
import api from "./api_impl";

type ApiStream = {
  send: (data: string) => void;
  close: () => void;
};

type ApiStreamConnector = {
  connect: (
    onOpen: (stream: ApiStream) => void,
    onMessage: (data: string) => void
  ) => ApiStream;
};

type Api = {
  composeBaseImages: (left: Blob[], right: Blob[]) => Promise<Blob>;
  reverseProjection: (image: Blob) => Promise<Blob>;
  getGameTypes: () => Promise<GameTypes>;
  getShowcaseAttackBaseImage: () => Promise<HTMLImageElement>;
  getShowcaseAttack: () => Promise<Frame[]>;
  optimizeAttack: ApiStreamConnector;
};

export default api;
export type { ApiStream, ApiStreamConnector, Api };
