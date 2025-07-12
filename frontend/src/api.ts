import axios from "axios";

import type { BuildingType, Frame, UnitType } from "./types";

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

const getBuildingTypes = async (): Promise<BuildingType[]> => {
  return (await axiosInstance.get("/get-building-types")).data;
};

const getUnitTypes = async (): Promise<UnitType[]> => {
  return (await axiosInstance.get("/get-unit-types")).data;
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
  getBuildingTypes,
  getUnitTypes,
  getShowcaseAttackBaseImage,
  getShowcaseAttack,
  getOptimizeAttackWebSocketUrl
};
