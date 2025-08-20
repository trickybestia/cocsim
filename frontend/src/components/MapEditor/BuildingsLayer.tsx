import { type ReactNode, memo, useContext } from "react";
import React from "react";
import { Layer, Line, Rect } from "react-konva";

import { GameTypesContext } from "../../hooks/use-game-types";
import type { Building } from "../../types";
import getGameType from "../../utils/get-game-type";

type Props = {
  buildings: Building[];
  selectedBuilding: Building | undefined;
  pixelsPerTile: number;
};

const BuildingsLayer: React.FC<Props> = memo(
  ({ buildings, selectedBuilding, pixelsPerTile }: Props) => {
    const gameTypes = useContext(GameTypesContext);

    const shapes: ReactNode[] = [];

    buildings.forEach((building) => {
      if (building === selectedBuilding)
        // handle later
        return;

      const buildingType = getGameType(gameTypes.buildings, building.name);

      shapes.push(
        <React.Fragment key={`${building.x}:${building.y}`}>
          <Rect
            x={building.x * pixelsPerTile}
            y={building.y * pixelsPerTile}
            width={buildingType.width * pixelsPerTile}
            height={buildingType.height * pixelsPerTile}
            stroke="yellow"
            strokeWidth={2}
          />
          <Line
            points={[
              building.x * pixelsPerTile,
              building.y * pixelsPerTile,
              (building.x + buildingType.width) * pixelsPerTile,
              (building.y + buildingType.height) * pixelsPerTile
            ]}
            stroke="yellow"
            strokeWidth={2}
          />
        </React.Fragment>
      );
    });

    return (
      <Layer>
        {shapes}
        {selectedBuilding !== undefined &&
          (() => {
            const selectedBuildingType = getGameType(
              gameTypes.buildings,
              selectedBuilding.name
            );

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
