import type { LayerConfig } from "konva/lib/Layer";
import { memo } from "react";
import { Layer } from "react-konva";

import type { Shape } from "../../types";
import ShapeRenderer from "./ShapeRenderer";

type Props = {
  layerProps: LayerConfig;
  totalBaseSize: number;
  canvasSize: number;
  collision: Shape[];
};

const COLLISION_OPACITY = 0.4;

const CollisionLayer: React.FC<Props> = memo(
  ({ layerProps, totalBaseSize, canvasSize, collision }: Props) => {
    const pixelsPerTile = canvasSize / totalBaseSize;

    const shapes = collision.map((shape, index) => (
      <ShapeRenderer
        key={index}
        shape={shape}
        pixelsPerTile={pixelsPerTile}
        opacity={COLLISION_OPACITY}
      />
    ));

    return <Layer {...layerProps}>{shapes}</Layer>;
  }
);

export default CollisionLayer;
export type { Props };
