import { compose_base_images } from "api_wasm";

import type { API } from ".";

const wasmAPI: API = {
  composeBaseImages: compose_base_images
} as API;

export default wasmAPI;
