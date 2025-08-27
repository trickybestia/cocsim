import {
  compose_base_images,
  get_game_types,
  get_showcase_attack,
  get_showcase_attack_base_image,
  reverse_projection
} from "api_wasm";

import type { API } from ".";

const getShowcaseAttackBaseImage = async (): Promise<HTMLImageElement> => {
  const imageBlob = await get_showcase_attack_base_image();
  const imageUrl = URL.createObjectURL(imageBlob);
  const image = new Image();

  image.src = imageUrl;

  return await new Promise((resolve) => {
    image.addEventListener("load", () => {
      resolve(image);
    });
  });
};

const wasmAPI: API = {
  composeBaseImages: compose_base_images,
  reverseProjection: reverse_projection,
  getGameTypes: get_game_types,
  getShowcaseAttackBaseImage,
  getShowcaseAttack: get_showcase_attack,
  getOptimizeAttackWebSocketUrl: () => {
    throw new Error("unimplemented");
  }
};

export default wasmAPI;
