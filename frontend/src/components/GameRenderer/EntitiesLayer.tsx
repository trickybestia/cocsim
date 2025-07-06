import { memo } from "react";
import { Layer } from "react-konva";

import type { Shape } from "../../types";
import ShapeRenderer from "./ShapeRenderer";

type Props = {
  totalBaseSize: number;
  canvasSize: number;
  entities: Shape[];
};

const ENTITIES_OPACITY = 1;

const EntitiesLayer: React.FC<Props> = memo(
  ({ totalBaseSize, canvasSize, entities }: Props) => {
    const pixelsPerTile = canvasSize / totalBaseSize;

    const shapes = entities.map((shape, index) => (
      <ShapeRenderer
        key={index}
        shape={shape}
        pixelsPerTile={pixelsPerTile}
        opacity={ENTITIES_OPACITY}
      />
    ));

    return <Layer>{shapes}</Layer>;
  }
);

export default EntitiesLayer;
export type { Props };
