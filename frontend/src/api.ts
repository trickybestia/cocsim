import axios from "axios";

import type { Frame, GameTypes } from "./types";

const axiosInstance = axios.create({
  baseURL: "http://localhost:8000/api"
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

  return new Promise((resolve) => {
    image.addEventListener("load", () => {
      resolve(image);
    });
  });
};

const getShowcaseAttack = async (): Promise<Frame[]> => {
  return (await axiosInstance.get("/get-showcase-attack")).data;
};

const getOptimizeAttackWebSocketUrl = (): string => {
  return axiosInstance.getUri({ url: "/optimize-attack" });
};

export {
  composeBaseImages,
  reverseProjection,
  getGameTypes,
  getShowcaseAttackBaseImage,
  getShowcaseAttack,
  getOptimizeAttackWebSocketUrl
};
