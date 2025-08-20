import {
  BlobReader,
  BlobWriter,
  TextReader,
  TextWriter,
  ZipReader,
  ZipWriter
} from "@zip.js/zip.js";

import type { Building, BuildingType, Map } from "../types";
import getGameType from "./get-game-type";

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
    const buildingType = getGameType(buildingTypes, building.name);

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
    const buildingType = getGameType(buildingTypes, building.name);

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

  await zipWriter.add("map.json", new TextReader(JSON.stringify(map)));
  await zipWriter.add("map.jpg", new BlobReader(image));

  return await zipWriter.close();
};

const parseMap = (json: string): Map => {
  // Maybe validate map using zod

  return JSON.parse(json);
};

const importFromZip = async (zip: Blob): Promise<{ map: Map; image: Blob }> => {
  const zipReader = new ZipReader(new BlobReader(zip));
  let image: Blob | undefined = undefined;
  let map: Map | undefined = undefined;

  for await (const entry of zipReader.getEntriesGenerator()) {
    if (entry.filename === "map.jpg") {
      const imageBlobWriter = new BlobWriter("image/jpeg");

      await entry.getData!(imageBlobWriter);

      image = await imageBlobWriter.getData();
    } else if (entry.filename === "map.json") {
      const mapTextWriter = new TextWriter();

      await entry.getData!(mapTextWriter);

      map = parseMap(await mapTextWriter.getData());
    }
  }

  if (image === undefined || map === undefined) {
    throw new Error("importFromZip failed");
  }

  return { map, image };
};

export {
  createBuildingsGrid,
  checkIntersection,
  resizeBuildings,
  cropImage,
  exportToZip,
  importFromZip
};
