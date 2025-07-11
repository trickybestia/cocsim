import { useEffect, useState } from "react";
import { Stage } from "react-konva";

import type { Frame } from "../../types";
import FloatNumberInput from "../FloatNumberInput";
import BackgroundLayer from "./BackgroundLayer";
import CollisionLayer from "./CollisionLayer";
import EntitiesLayer from "./EntitiesLayer";

type Props = {
  frames: Frame[];
  baseImage?: HTMLImageElement;
};

const GameRenderer: React.FC<Props> = ({ frames, baseImage }: Props) => {
  const [frameIndex, setFrameIndex] = useState(0);
  const [speed, setSpeed] = useState(1.0);
  const [isPaused, setIsPaused] = useState(true);

  const canvasSize = 800;

  let collision = null;

  for (let i = frameIndex; collision === null; i--) {
    collision = frames[i].collision;
  }

  useEffect(() => {
    if (isPaused) return;

    const frameTime = frames[1].timeElapsed;

    const intervalId = setInterval(
      () =>
        setFrameIndex((frameIndex) => {
          if (frameIndex === frames.length - 1) {
            setIsPaused(true);

            return frameIndex;
          } else {
            return frameIndex + 1;
          }
        }),
      (frameTime * 1000) / speed
    );

    return () => {
      clearInterval(intervalId);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [speed, isPaused]);

  const onPauseButtonClick = () => {
    setIsPaused(!isPaused);
  };

  return (
    <div className="flex w-[800px] flex-col gap-2">
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
      <div className="flex justify-between">
        <p>{frames[frameIndex].timeElapsed}</p>
        <p>{frames[frameIndex].progressInfo}</p>
      </div>
      <input
        type="range"
        min={0}
        max={frames.length - 1}
        value={frameIndex}
        onChange={(e) => setFrameIndex(parseInt(e.target.value))}
        className="h-2 w-full cursor-pointer appearance-none bg-gray-200 dark:bg-gray-700"
      ></input>
      <div className="flex items-center gap-2">
        <button
          className="col-span-2 cursor-pointer bg-blue-400 px-1 py-1 text-base font-bold text-white hover:bg-blue-600"
          onClick={onPauseButtonClick}
        >
          {isPaused ? (
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              fill="currentColor"
              viewBox="0 0 16 16"
            >
              <path d="m11.596 8.697-6.363 3.692c-.54.313-1.233-.066-1.233-.697V4.308c0-.63.692-1.01 1.233-.696l6.363 3.692a.802.802 0 0 1 0 1.393" />
            </svg>
          ) : (
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              fill="currentColor"
              viewBox="0 0 16 16"
            >
              <path d="M5.5 3.5A1.5 1.5 0 0 1 7 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5m5 0A1.5 1.5 0 0 1 12 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5" />
            </svg>
          )}
        </button>
        <div className="flex h-min gap-2">
          <FloatNumberInput
            text="Speed:"
            min={0.1}
            max={5.0}
            step={0.1}
            defaultValue={1.0}
            onChange={setSpeed}
          />
        </div>
      </div>
    </div>
  );
};

export default GameRenderer;
export type { Props };
