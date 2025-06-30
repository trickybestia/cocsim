import { memo } from "react";
import { Layer, Text } from "react-konva";

type Props = { totalSize: number; canvasSize: number };

// Manually tuned numbers to make text look better
const TEXT_OFFSET_X = -0.1;
const TEXT_OFFSET_Y = -0.4;
const TEXT_SIZE = 2;

const DrawCoordsLayer: React.FC<Props> = memo(
  ({ totalSize, canvasSize }: Props) => {
    const pixelsPerTile = canvasSize / totalSize;

    const coords = [];

    for (let tileX = 0; tileX != totalSize; tileX++) {
      for (let tileY = 0; tileY != totalSize; tileY++) {
        const text = `${tileX},${tileY}`;

        coords.push(
          <Text
            key={text}
            x={(tileX + TEXT_OFFSET_X) * pixelsPerTile}
            y={(tileY + TEXT_OFFSET_Y) * pixelsPerTile}
            width={TEXT_SIZE * pixelsPerTile}
            height={TEXT_SIZE * pixelsPerTile}
            align="center"
            verticalAlign="center"
            rotation={45}
            fontSize={10}
            text={text}
          />
        );
      }
    }

    return <Layer>{coords}</Layer>;
  }
);

export default DrawCoordsLayer;
export type { Props };
