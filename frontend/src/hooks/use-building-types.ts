import { createContext, useContext } from "react";

import type { Building, BuildingType } from "../types";

const BuildingTypesContext = createContext<BuildingType[]>([]);

const useBuildingTypes = (): {
  buildingTypes: BuildingType[];
  getBuildingType: (building: Building | string) => BuildingType;
} => {
  const buildingTypes = useContext(BuildingTypesContext);

  const getBuildingType = (building: Building | string): BuildingType => {
    const buildingName =
      typeof building === "string" ? building : building.name;

    const result = buildingTypes.find(
      (buildingType) => buildingType.name === buildingName
    );

    if (result === undefined)
      throw new Error(`BuildingType with name === "${buildingName}" not found`);

    return result;
  };

  return { buildingTypes, getBuildingType };
};

export default useBuildingTypes;
export { BuildingTypesContext };
