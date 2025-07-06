import axios from "axios";

import type { BuildingType, Frame } from "./types";

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

const getShowcaseAttackBaseImage = async (): Promise<Blob> => {
  return (
    await axiosInstance.get("/get-showcase-attack-base-image", {
      responseType: "blob"
    })
  ).data;
};

const getShowcaseAttack = async (): Promise<Frame[]> => {
  return (await axiosInstance.get("/get-showcase-attack")).data;
};

getShowcaseAttack().then(console.log);

export {
  composeBaseImages,
  reverseProjection,
  getBuildingTypes,
  getShowcaseAttackBaseImage,
  getShowcaseAttack
};
