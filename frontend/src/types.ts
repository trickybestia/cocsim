type Map = {
  baseSize: number;
  borderSize: number;
  buildings: Building[];
};

type WithName = {
  name: string;
};

type WithCount = {
  count: number;
};

type WithLevel = {
  level: number;
};

type GenericBuilding = {
  name: string;
  x: number;
  y: number;
  level: number;

  [option: string]: string | number;
};

type ClanCastleBuilding = GenericBuilding & {
  name: "ClanCastle";
  units: UnitWithCount[];
};

/**
 * Building instance type.
 */
type Building = GenericBuilding | ClanCastleBuilding;

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

type Unit = {
  name: string;
  level: number;
};

type UnitWithCount = {
  value: Unit;
  count: number;
};

type UnitType = {
  name: string;
  levels: number;
  housingSpace: number;
};

type Spell = {
  name: string;
  level: number;
};

type SpellWithCount = {
  value: Spell;
  count: number;
};

type SpellType = {
  name: string;
  levels: number;
  housingSpace: number;
};

type GameTypes = {
  buildings: BuildingType[];
  units: UnitType[];
  spells: SpellType[];
};

/**
 * Shape type. Represents shape possible to get from backend and draw on canvas.
 */
type Shape =
  | [
      name: "arc",
      x: number,
      y: number,
      radius: number,
      rotation: number,
      angle: number,
      width: number,
      color: string
    ]
  | [
      name: "rect",
      x: number,
      y: number,
      width: number,
      height: number,
      color: string
    ]
  | [name: "circle", x: number, y: number, radius: number, color: string]
  | [
      name: "line",
      x1: number,
      y1: number,
      x2: number,
      y2: number,
      width: number,
      color: string
    ];

/**
 * Frame type. Represents single game drawed frame.
 */
type Frame = {
  timeElapsed: number;
  progressInfo: string;
  totalBaseSize: number;
  grid: Shape[] | null;
  collision: Shape[] | null;
  entities: Shape[];
};

type OptimizeAttackMessage =
  | {
      type: "progress";
      progress: string;
    }
  | {
      type: "result";
      result: Frame[];
    };

export type {
  Map,
  BuildingType,
  GenericBuilding,
  ClanCastleBuilding,
  Building,
  Shape,
  Frame,
  Unit,
  UnitWithCount,
  UnitType,
  Spell,
  SpellWithCount,
  SpellType,
  OptimizeAttackMessage,
  WithCount,
  WithName,
  WithLevel,
  GameTypes
};
