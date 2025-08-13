import type { BuildingType } from "../types";

const getBuildingTypesWithSize = (
  buildingTypes: BuildingType[],
  width: number,
  height: number
): BuildingType[] => {
  return buildingTypes.filter(
    (buildingType) =>
      width % buildingType.width === 0 && height % buildingType.height === 0
  );
};

export default getBuildingTypesWithSize;
