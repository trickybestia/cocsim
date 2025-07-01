type Map = {
  baseSize: number;
  borderSize: number;
  buildings: Building[];
};

/**
 * Building instance type.
 */
type Building = {
  name: string;
  x: number;
  y: number;
  level: number;

  [option: string]: string | number;
};

/**
 * Building type type. Returned by getBuildingTypes() api to get all possible buildings.
 */
type BuildingType = {
  name: string;
  width: number;
  height: number;
  levels: number;
  options: { name: string; values: (string | number)[] }[];
};

export type { Map, BuildingType, Building };
