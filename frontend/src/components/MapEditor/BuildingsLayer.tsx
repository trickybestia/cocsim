import { type ReactNode, memo } from "react";
import { Layer, Rect } from "react-konva";

import useBuildingTypes from "../../hooks/use-building-types";
import type { Building } from "../../types";

type Props = {
  buildings: Building[];
  selectedBuilding: Building | undefined;
  pixelsPerTile: number;
};

const BuildingsLayer: React.FC<Props> = memo(
  ({ buildings, selectedBuilding, pixelsPerTile }: Props) => {
    const { getBuildingType } = useBuildingTypes();

    const shapes: ReactNode[] = [];

    buildings.forEach((building) => {
      if (building === selectedBuilding)
        // handle later
        return;

      const buildingType = getBuildingType(building);

      shapes.push(
        <Rect
          key={`${building.x}:${building.y}`}
          x={building.x * pixelsPerTile}
          y={building.y * pixelsPerTile}
          width={buildingType.width * pixelsPerTile}
          height={buildingType.height * pixelsPerTile}
          stroke="yellow"
          strokeWidth={3}
        />
      );
    });

    return (
      <Layer>
        {shapes}
        {selectedBuilding !== undefined &&
          (() => {
            const selectedBuildingType = getBuildingType(selectedBuilding);

            return (
              <Rect
                x={selectedBuilding.x * pixelsPerTile}
                y={selectedBuilding.y * pixelsPerTile}
                width={selectedBuildingType.width * pixelsPerTile}
                height={selectedBuildingType.height * pixelsPerTile}
                stroke="red"
                strokeWidth={3}
              />
            );
          })()}
      </Layer>
    );
  }
);

export default BuildingsLayer;
export type { Props };
