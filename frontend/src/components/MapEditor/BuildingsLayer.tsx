import { type ReactNode, memo } from "react";
import { Layer, Rect } from "react-konva";

import type { Building, BuildingType } from "../../types";

type Props = {
  totalSize: number;
  canvasSize: number;
  buildings: Building[];
  buildingTypes: BuildingType[];
};

const BuildingsLayer: React.FC<Props> = memo(
  ({ totalSize, canvasSize, buildings, buildingTypes }: Props) => {
    const pixelsPerTile = canvasSize / totalSize;

    const shapes: ReactNode[] = [];

    buildings.forEach((building) => {
      const buildingType = buildingTypes.find(
        (value) => value.name == building.name
      )!;

      shapes.push(
        <Rect
          key={`${building.x}:${building.y}`}
          x={building.x * pixelsPerTile}
          y={building.y * pixelsPerTile}
          width={(building.x + buildingType.width) * pixelsPerTile}
          height={(building.y + buildingType.height) * pixelsPerTile}
          stroke="yellow"
          strokeWidth={2}
        />
      );
    });

    return <Layer>{shapes}</Layer>;
  }
);

export default BuildingsLayer;
export type { Props };
