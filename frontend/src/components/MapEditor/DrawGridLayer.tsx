import { memo } from "react";
import { Circle, Layer } from "react-konva";

type Props = { totalSize: number; canvasSize: number };

const DrawGridLayer: React.FC<Props> = memo(
  ({ totalSize, canvasSize }: Props) => {
    const pixelsPerTile = canvasSize / totalSize;

    const dots = [];

    for (let tileX = 0; tileX != totalSize + 1; tileX++) {
      for (let tileY = 0; tileY != totalSize + 1; tileY++) {
        dots.push(
          <Circle
            key={`${tileX}:${tileY}`}
            x={tileX * pixelsPerTile}
            y={tileY * pixelsPerTile}
            radius={2}
            fill="white"
          />
        );
      }
    }

    return <Layer>{dots}</Layer>;
  }
);

export default DrawGridLayer;
export type { Props };
