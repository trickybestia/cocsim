import { Circle, Layer } from "react-konva";

type Props = { totalSize: number; canvasSize: number };

const DrawGridLayer: React.FC<Props> = ({ totalSize, canvasSize }: Props) => {
  const pixelsPerTile = canvasSize / totalSize;

  const dots = [];

  for (let tile_x = 0; tile_x != totalSize + 1; tile_x++) {
    for (let tile_y = 0; tile_y != totalSize + 1; tile_y++) {
      dots.push(
        <Circle
          key={`${tile_x}:${tile_y}`}
          x={tile_x * pixelsPerTile}
          y={tile_y * pixelsPerTile}
          radius={2}
          fill="white"
        />
      );
    }
  }

  return <Layer>{dots}</Layer>;
};

export default DrawGridLayer;
export type { Props };
