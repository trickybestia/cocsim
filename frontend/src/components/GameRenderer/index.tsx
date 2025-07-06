import { useEffect, useState } from "react";
import { Stage } from "react-konva";

import type { Frame } from "../../types";
import BackgroundLayer from "./BackgroundLayer";
import CollisionLayer from "./CollisionLayer";
import EntitiesLayer from "./EntitiesLayer";

type Props = {
  frames: Frame[];
  baseImage?: HTMLImageElement;
};

const GameRenderer: React.FC<Props> = ({ frames, baseImage }: Props) => {
  const [frameIndex, setFrameIndex] = useState(0);
  const canvasSize = 800;

  let collision = null;

  for (let i = frameIndex; collision === null; i--) {
    collision = frames[i].collision;
  }

  useEffect(() => {
    const intervalId = setInterval(
      () =>
        setFrameIndex((frameIndex) =>
          frameIndex === frames.length - 1 ? frameIndex : frameIndex + 1
        ),
      (1 / 60) * 1000
    );

    return () => {
      clearInterval(intervalId);
    };
  });

  return (
    <div className="flex flex-col">
      <Stage width={canvasSize} height={canvasSize} listening={false}>
        <BackgroundLayer
          totalBaseSize={frames[0].totalBaseSize}
          grid={frames[0].grid!}
          baseImage={baseImage}
          canvasSize={canvasSize}
        />
        <CollisionLayer
          totalBaseSize={frames[frameIndex].totalBaseSize}
          collision={collision}
          canvasSize={canvasSize}
        />
        <EntitiesLayer
          totalBaseSize={frames[frameIndex].totalBaseSize}
          entities={frames[frameIndex].entities}
          canvasSize={canvasSize}
        />
      </Stage>
    </div>
  );
};

export default GameRenderer;
export type { Props };
