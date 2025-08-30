import init, {
  compose_base_images,
  get_game_types,
  get_showcase_attack,
  get_showcase_attack_base_image,
  initThreadPool,
  optimize_attack_connect,
  reverse_projection
} from "api_wasm";
import api_wasm_bg from "api_wasm/api_wasm_bg.wasm?url";

import type { Api, ApiStream, ApiStreamConnector } from ".";

await init({ module_or_path: api_wasm_bg });
await initThreadPool(navigator.hardwareConcurrency);

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

const optimizeAttack: ApiStreamConnector = {
  connect: (
    onOpen: (stream: ApiStream) => void,
    onMessage: (data: string) => void
  ): ApiStream => {
    const stream = optimize_attack_connect();

    stream.start(onMessage);

    onOpen(stream);

    return stream;
  }
};

const wasmAPI: Api = {
  composeBaseImages: compose_base_images,
  reverseProjection: reverse_projection,
  getGameTypes: get_game_types,
  getShowcaseAttackBaseImage,
  getShowcaseAttack: get_showcase_attack,
  optimizeAttack
};

export default wasmAPI;
