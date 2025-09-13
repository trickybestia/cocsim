import axios from "axios";

import type { Api, ApiStream, ApiStreamConnector } from ".";
import type { Frame, GameTypes } from "../types";

const axiosInstance = axios.create({
  baseURL: import.meta.env.VITE_AXIOS_API_BASE_URL
});

const composeBaseImages = async (
  left: Blob[],
  right: Blob[]
): Promise<Blob> => {
  const data = new FormData();

  for (let i = 0; i != left.length; i++) {
    data.append("left", left[i]);
  }

  for (let i = 0; i != right.length; i++) {
    data.append("right", right[i]);
  }

  return (
    await axiosInstance.postForm("/compose-base-images", data, {
      responseType: "blob"
    })
  ).data;
};

const reverseProjection = async (image: Blob): Promise<Blob> => {
  return (
    await axiosInstance.postForm(
      "/reverse-projection",
      {
        image: image
      },
      {
        responseType: "blob"
      }
    )
  ).data;
};

const getGameTypes = async (): Promise<GameTypes> => {
  return (await axiosInstance.get("/get-game-types")).data;
};

const getShowcaseAttackBaseImage = (): Promise<HTMLImageElement> => {
  const imageUrl = axiosInstance.getUri({
    url: "/get-showcase-attack-base-image"
  });

  const image = new Image();

  image.src = imageUrl;
  image.crossOrigin = "anonymous";

  return new Promise((resolve) => {
    image.addEventListener("load", () => {
      resolve(image);
    });
  });
};

const getShowcaseAttack = async (): Promise<Frame[]> => {
  return (await axiosInstance.get("/get-showcase-attack")).data;
};

const optimizeAttack: ApiStreamConnector = {
  connect: (
    onOpen: (stream: ApiStream) => void,
    onMessage: (data: string) => void
  ): ApiStream => {
    const socket = new WebSocket(
      axiosInstance.getUri({ url: "/optimize-attack" })
    );

    const apiStream: ApiStream = {
      send: (data: string) => {
        socket.send(data);
      },
      close() {
        socket.close();
      }
    };

    socket.onopen = () => {
      onOpen(socket);
    };
    socket.onmessage = (e) => {
      onMessage(e.data);
    };

    return apiStream;
  }
};

const axiosAPI: Api = {
  composeBaseImages,
  reverseProjection,
  getGameTypes,
  getShowcaseAttackBaseImage,
  getShowcaseAttack,
  optimizeAttack
};

export default axiosAPI;
