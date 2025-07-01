import { BlobReader, BlobWriter, TextReader, ZipWriter } from "@zip.js/zip.js";

import type { Building, BuildingType, Map } from "../types";

const createBuildingsGrid = (
  buildings: Building[],
  buildingTypes: BuildingType[],
  mapTotalSize: number
): (Building | undefined)[][] => {
  const result: (Building | undefined)[][] = [];

  for (let i = 0; i != mapTotalSize; i++) {
    result.push(new Array(mapTotalSize));
  }

  buildings.forEach((building) => {
    const buildingType = buildingTypes.find(
      (buildingType) => buildingType.name === building.name
    )!;

    for (
      let tileX = building.x;
      tileX != building.x + buildingType.width;
      tileX++
    ) {
      for (
        let tileY = building.y;
        tileY != building.y + buildingType.height;
        tileY++
      ) {
        result[tileX][tileY] = building;
      }
    }
  });

  return result;
};

const checkIntersection = (
  buildingsGrid: (Building | undefined)[][],
  area: {
    leftTop: { x: number; y: number };
    rightBottom: { x: number; y: number };
  }
): boolean => {
  for (let x = area.leftTop.x; x <= area.rightBottom.x; x++) {
    for (let y = area.leftTop.y; y <= area.rightBottom.y; y++) {
      if (buildingsGrid[x][y] !== undefined) {
        return true;
      }
    }
  }

  return false;
};

const resizeBuildings = (
  buildings: Building[],
  buildingTypes: BuildingType[],
  mapTotalSize: number
): Building[] => {
  return buildings.filter((building) => {
    const buildingType = buildingTypes.find(
      (buildingType) => buildingType.name === building.name
    )!;

    return (
      building.x + buildingType.width < mapTotalSize &&
      building.y + buildingType.height < mapTotalSize
    );
  });
};

const cropImage = async (
  image: HTMLImageElement,
  x: number,
  y: number,
  width: number,
  height: number
): Promise<Blob> => {
  const canvas = new OffscreenCanvas(width, height);

  canvas
    .getContext("2d")!
    .drawImage(image, x, y, width, height, 0, 0, width, height);

  return await canvas.convertToBlob({ type: "image/jpeg", quality: 0.8 });
};

const exportToZip = async (map: Map, image: Blob): Promise<Blob> => {
  const zipWriter = new ZipWriter(new BlobWriter("application/zip"), {
    level: 0,
    compressionMethod: 0
  });

  console.log(image);

  await zipWriter.add("map.json", new TextReader(JSON.stringify(map)));
  await zipWriter.add("map.jpg", new BlobReader(image));

  return await zipWriter.close();
};

export {
  createBuildingsGrid,
  checkIntersection,
  resizeBuildings,
  cropImage,
  exportToZip
};
