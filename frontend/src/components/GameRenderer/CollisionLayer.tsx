import { memo } from "react";
import { Layer } from "react-konva";

import type { Shape } from "../../types";
import ShapeRenderer from "./ShapeRenderer";

type Props = {
  totalBaseSize: number;
  canvasSize: number;
  collision: Shape[];
};

const COLLISION_OPACITY = 0.4;

const CollisionLayer: React.FC<Props> = memo(
  ({ totalBaseSize, canvasSize, collision }: Props) => {
    const pixelsPerTile = canvasSize / totalBaseSize;

    const shapes = collision.map((shape, index) => (
      <ShapeRenderer
        key={index}
        shape={shape}
        pixelsPerTile={pixelsPerTile}
        opacity={COLLISION_OPACITY}
      />
    ));

    return <Layer>{shapes}</Layer>;
  }
);

export default CollisionLayer;
export type { Props };
