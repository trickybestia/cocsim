import axios from "axios";

import type { BuildingType } from "./types";

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

export { composeBaseImages, reverseProjection, getBuildingTypes };
