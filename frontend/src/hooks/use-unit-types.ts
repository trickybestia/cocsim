import { createContext, useContext } from "react";

import type { Unit, UnitType } from "../types";

const UnitTypesContext = createContext<UnitType[]>([]);

const useUnitTypes = (): {
  unitTypes: UnitType[];
  getUnitType: (unit: Unit | string) => UnitType;
} => {
  const unitTypes = useContext(UnitTypesContext);

  const getUnitType = (unit: Unit | string): UnitType => {
    const unitName = typeof unit === "string" ? unit : unit.name;

    const result = unitTypes.find((unitType) => unitType.name === unitName);

    if (result === undefined)
      throw new Error(`UnitType with name === "${unitName}" not found`);

    return result;
  };

  return { unitTypes, getUnitType };
};

export default useUnitTypes;
export { UnitTypesContext };
