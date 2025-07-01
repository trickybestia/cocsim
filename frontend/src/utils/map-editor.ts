import type { Building, BuildingType } from "../types";

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

export { createBuildingsGrid, checkIntersection, resizeBuildings };
