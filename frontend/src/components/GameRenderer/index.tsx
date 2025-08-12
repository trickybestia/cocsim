import type Konva from "konva";
import type { KonvaEventObject } from "konva/lib/Node";
import { useEffect, useRef, useState } from "react";
import { Stage } from "react-konva";
import { twMerge } from "tailwind-merge";

import type { Frame } from "../../types";
import clamp from "../../utils/clamp";
import FloatNumberInput from "../FloatNumberInput";
import BackgroundLayer from "./BackgroundLayer";
import CollisionLayer from "./CollisionLayer";
import EntitiesLayer from "./EntitiesLayer";

type Props = {
  className?: string;
  frames: Frame[];
  baseImage?: HTMLImageElement;
};

const GameRenderer: React.FC<Props> = ({
  className,
  frames,
  baseImage
}: Props) => {
  const canvasWrapperRef = useRef<HTMLDivElement | null>(null);
  const canvasRef = useRef<Konva.Stage>(null);
  const [canvasSize, setCanvasSize] = useState(1);

  const [frameIndex, setFrameIndex] = useState(0);
  const [speed, setSpeed] = useState(1.0);
  const [isPaused, setIsPaused] = useState(true);
  const [isSliderDragged, setIsSliderDragged] = useState(false); // user is interacting with timeline slider
  const [gameLikeRotation, setGameLikeRotation] = useState(false);

  let collision = null;

  for (let i = frameIndex; collision === null; i--) {
    collision = frames[i].collision;
  }

  const layerProps = {
    rotation: gameLikeRotation ? -45 : 0,
    x: canvasSize / 2,
    y: canvasSize / 2,
    offset: { x: canvasSize / 2, y: canvasSize / 2 },
    scale: {
      x: gameLikeRotation ? Math.SQRT1_2 : 1,
      y: gameLikeRotation ? Math.SQRT1_2 : 1
    }
  };

  useEffect(() => {
    const onResize = () => {
      if (canvasWrapperRef.current === null) return;

      setCanvasSize(
        Math.min(
          canvasWrapperRef.current.offsetHeight,
          canvasWrapperRef.current.offsetWidth
        )
      );
    };

    onResize();

    window.addEventListener("resize", onResize);

    return () => {
      window.removeEventListener("resize", onResize);
    };
  }, []);

  useEffect(() => {
    if (isPaused || isSliderDragged) return;

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
  }, [speed, isPaused, isSliderDragged]);

  const resetCamera = (gameLikeRotation: boolean) => {
    if (canvasRef.current === null) return;

    canvasRef.current.scale({ x: 1, y: gameLikeRotation ? 0.75 : 1 });
    canvasRef.current.setPosition({ x: 0, y: 0 });
  };

  const onPauseButtonClick = () => {
    setIsPaused(!isPaused);
  };

  const canvasOnWheel = (e: KonvaEventObject<WheelEvent>) => {
    const stage = e.target.getStage();

    if (stage === null) return;

    e.evt.preventDefault();

    const oldXScale = stage.scaleX();
    const oldYScale = stage.scaleY();
    const pointer = stage.getPointerPosition()!;

    const mousePointTo = {
      x: (pointer.x - stage.x()) / oldXScale,
      y: (pointer.y - stage.y()) / oldYScale
    };

    // how to scale? Zoom in? Or zoom out?
    const direction = e.evt.deltaY > 0 ? -1 : 1;

    const STAGE_SCALE_FACTOR = 1.1;

    const newXScale = clamp(
      0.5,
      direction > 0
        ? oldXScale * STAGE_SCALE_FACTOR
        : oldXScale / STAGE_SCALE_FACTOR,
      10
    );
    const newYScale = (newXScale * oldYScale) / oldXScale;
    stage.scale({ x: newXScale, y: newYScale });

    const newPos = {
      x: pointer.x - mousePointTo.x * newXScale,
      y: pointer.y - mousePointTo.y * newYScale
    };

    stage.position(newPos);
  };

  const canvasOnMouseDown = (e: KonvaEventObject<MouseEvent>) => {
    // to prevent auto scrolling on middle mouse button press
    e.evt.preventDefault();
  };

  const canvasOnPointerMove = (e: KonvaEventObject<PointerEvent>) => {
    const stage = e.target.getStage();

    if (stage === null) return;

    if ((e.evt.buttons & 0x4) !== 0) {
      // mouse wheel pressed
      e.evt.preventDefault();

      const position = stage.getPosition();

      stage.position({
        x: position.x + e.evt.movementX,
        y: position.y + e.evt.movementY
      });
    }
  };

  return (
    <div className={twMerge("flex h-full grow flex-col gap-2", className)}>
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
        onMouseDown={() => setIsSliderDragged(true)}
        onMouseUp={() => setIsSliderDragged(false)}
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
          <p>Rotate as in game:</p>
          <input
            type="checkbox"
            checked={gameLikeRotation}
            onChange={(e) => {
              setGameLikeRotation(e.target.checked);
              resetCamera(e.target.checked);
            }}
          ></input>
        </div>
      </div>

      <div className="relative flex grow justify-around" ref={canvasWrapperRef}>
        <Stage
          ref={canvasRef}
          className="absolute bg-green-900"
          width={canvasSize}
          height={canvasSize}
          listening={false}
          scaleY={gameLikeRotation ? 0.75 : 1}
          onPointerMove={canvasOnPointerMove}
          onWheel={canvasOnWheel}
          onMouseDown={canvasOnMouseDown}
        >
          <BackgroundLayer
            layerProps={layerProps}
            totalBaseSize={frames[0].totalBaseSize}
            grid={frames[0].grid!}
            baseImage={baseImage}
            canvasSize={canvasSize}
          />
          <CollisionLayer
            layerProps={layerProps}
            totalBaseSize={frames[frameIndex].totalBaseSize}
            collision={collision}
            canvasSize={canvasSize}
          />
          <EntitiesLayer
            layerProps={layerProps}
            totalBaseSize={frames[frameIndex].totalBaseSize}
            entities={frames[frameIndex].entities}
            canvasSize={canvasSize}
          />
        </Stage>
      </div>
    </div>
  );
};

export default GameRenderer;
export type { Props };
