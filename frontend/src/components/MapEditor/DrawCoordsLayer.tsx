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

    for (let tile_x = 0; tile_x != totalSize; tile_x++) {
      for (let tile_y = 0; tile_y != totalSize; tile_y++) {
        const text = `${tile_x},${tile_y}`;

        coords.push(
          <Text
            key={text}
            x={(tile_x + TEXT_OFFSET_X) * pixelsPerTile}
            y={(tile_y + TEXT_OFFSET_Y) * pixelsPerTile}
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
