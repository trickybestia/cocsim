import { memo } from "react";
import { Image, Layer } from "react-konva";

import type { Shape } from "../../types";
import ShapeRenderer from "./ShapeRenderer";

type Props = {
  totalBaseSize: number;
  canvasSize: number;
  grid: Shape[];
  baseImage?: HTMLImageElement;
};

const GRID_OPACITY = 0.3;

const BackgroundLayer: React.FC<Props> = memo(
  ({ totalBaseSize, canvasSize, grid, baseImage }: Props) => {
    const pixelsPerTile = canvasSize / totalBaseSize;

    const opacity = baseImage === undefined ? 1 : GRID_OPACITY;

    const shapes = grid.map((shape, index) => (
      <ShapeRenderer
        key={index}
        shape={shape}
        pixelsPerTile={pixelsPerTile}
        opacity={opacity}
      />
    ));

    return (
      <Layer>
        {baseImage !== undefined && (
          <Image
            scaleX={canvasSize / baseImage.width}
            scaleY={canvasSize / baseImage.height}
            image={baseImage}
          />
        )}
        {shapes}
      </Layer>
    );
  }
);

export default BackgroundLayer;
export type { Props };
