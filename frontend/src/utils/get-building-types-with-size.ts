import type { BuildingType } from "../types";

const getBuildingTypesWithSize = (
  buildingTypes: BuildingType[],
  width: number,
  height: number
): BuildingType[] => {
  return buildingTypes.filter(
    (buildingType) =>
      buildingType.name === "Wall" ||
      (buildingType.width === width && buildingType.height === height)
  );
};

export default getBuildingTypesWithSize;
