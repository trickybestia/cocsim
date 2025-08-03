import { memo } from "react";
import { Arc, Circle, Line, Rect } from "react-konva";

import type { Shape } from "../../types";

type Props = { shape: Shape; pixelsPerTile: number; opacity: number };

const ShapeRenderer: React.FC<Props> = memo(
  ({ shape, pixelsPerTile, opacity }: Props) => {
    switch (shape[0]) {
      case "arc":
        return (
          <Arc
            x={shape[1] * pixelsPerTile}
            y={shape[2] * pixelsPerTile}
            innerRadius={(shape[3] - shape[6] / 2.0) * pixelsPerTile}
            outerRadius={(shape[3] + shape[6] / 2.0) * pixelsPerTile}
            rotation={shape[4]}
            angle={shape[5]}
            fill={shape[7]}
            opacity={opacity}
          />
        );
      case "rect":
        return (
          <Rect
            x={shape[1] * pixelsPerTile}
            y={shape[2] * pixelsPerTile}
            width={shape[3] * pixelsPerTile}
            height={shape[4] * pixelsPerTile}
            fill={shape[5]}
            opacity={opacity}
          />
        );
      case "circle":
        return (
          <Circle
            x={shape[1] * pixelsPerTile}
            y={shape[2] * pixelsPerTile}
            radius={shape[3] * pixelsPerTile}
            fill={shape[4]}
            opacity={opacity}
          />
        );
      case "line":
        return (
          <Line
            points={[
              shape[1] * pixelsPerTile,
              shape[2] * pixelsPerTile,
              shape[3] * pixelsPerTile,
              shape[4] * pixelsPerTile
            ]}
            strokeWidth={shape[5] * pixelsPerTile}
            stroke={shape[6]}
            opacity={opacity}
          />
        );

      default:
        throw new Error(`Invalid shape type: ${shape[0]}`);
    }
  }
);

export default ShapeRenderer;
export type { Props };
