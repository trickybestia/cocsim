import type { KonvaEventObject } from "konva/lib/Node";
import { useEffect, useRef, useState } from "react";
import { Image, Layer, Stage } from "react-konva";
import { twMerge } from "tailwind-merge";

import clamp from "../../utils/clamp";
import DrawCoordsLayer from "./DrawCoordsLayer";
import DrawGridLayer from "./DrawGridLayer";
import NumberInput from "./NumberInput";

type Props = React.HTMLAttributes<HTMLDivElement> & {
  image: HTMLImageElement;
};

const STAGE_SCALE_FACTOR = 1.1;

const MapEditor: React.FC<Props> = ({
  className,

  image,

  ...props
}: Props) => {
  const canvasWrapperRef = useRef<HTMLDivElement | null>(null);
  const [canvasSize, setCanvasSize] = useState(0);

  useEffect(() => {
    const onResize = () => {
      if (canvasWrapperRef.current === null) return;

      setCanvasSize(canvasWrapperRef.current.offsetHeight);
    };

    onResize();

    window.addEventListener("resize", onResize);

    return () => {
      window.removeEventListener("resize", onResize);
    };
  }, []);

  const [drawGrid, setDrawGrid] = useState(false);
  const [drawCoords, setDrawCoords] = useState(false);
  const [baseSize, setBaseSize] = useState(44);
  const [borderSize, setBorderSize] = useState(4);

  const canvasOnWheel = (e: KonvaEventObject<WheelEvent>) => {
    const stage = e.target.getStage();

    if (stage === null) return;

    e.evt.preventDefault();

    const oldScale = stage.scaleX();
    const pointer = stage.getPointerPosition()!;

    const mousePointTo = {
      x: (pointer.x - stage.x()) / oldScale,
      y: (pointer.y - stage.y()) / oldScale
    };

    // how to scale? Zoom in? Or zoom out?
    const direction = e.evt.deltaY > 0 ? -1 : 1;

    const newScale = clamp(
      0.5,
      direction > 0
        ? oldScale * STAGE_SCALE_FACTOR
        : oldScale / STAGE_SCALE_FACTOR,
      10
    );
    stage.scale({ x: newScale, y: newScale });

    const newPos = {
      x: pointer.x - mousePointTo.x * newScale,
      y: pointer.y - mousePointTo.y * newScale
    };

    stage.position(newPos);
  };

  const canvasOnPointerMove = (e: KonvaEventObject<PointerEvent>) => {
    const stage = e.target.getStage();

    if (stage === null || (e.evt.buttons & 0x1) === 0) return;

    e.evt.preventDefault();

    const position = stage.getPosition();

    stage.position({
      x: position.x + e.evt.movementX,
      y: position.y + e.evt.movementY
    });
  };

  return (
    <div
      className={twMerge(className, "flex h-full w-full justify-between gap-2")}
      {...props}
    >
      <div>
        <div className="grid-col grid grid-cols-[auto_min-content] gap-1 text-end text-nowrap">
          <p>Draw grid:</p>
          <input
            type="checkbox"
            checked={drawGrid}
            onChange={(e) => setDrawGrid(e.target.checked)}
          />
          <p>Draw coords:</p>
          <input
            type="checkbox"
            checked={drawCoords}
            onChange={(e) => setDrawCoords(e.target.checked)}
          />
          <NumberInput
            text="Base size:"
            min={1}
            max={44}
            defaultValue={baseSize}
            onChange={setBaseSize}
          />
          <NumberInput
            text="Border size:"
            min={0}
            max={4}
            defaultValue={borderSize}
            onChange={setBorderSize}
          />
          <NumberInput
            text="Start X:"
            min={0}
            max={image.width - 1}
            defaultValue={startX}
            onChange={setStartX}
          />
          <NumberInput
            text="Start Y:"
            min={0}
            max={image.width - 1}
            defaultValue={startY}
            onChange={setStartY}
          />
          <NumberInput
            text="End X:"
            min={0}
            max={image.width - 1}
            defaultValue={endX}
            onChange={setEndX}
          />
          <NumberInput
            text="End Y:"
            min={0}
            max={image.width - 1}
            defaultValue={endY}
            onChange={setEndY}
          />
        </div>
      </div>

      <div ref={canvasWrapperRef} className="aspect-square bg-green-900">
        <Stage
          width={canvasSize}
          height={canvasSize}
          onWheel={canvasOnWheel}
          onPointerMove={canvasOnPointerMove}
        >
          <Layer>
            <Image
              scaleX={canvasSize / image.width}
              scaleY={canvasSize / image.width}
              crop={{
                x: startX,
                y: startY,
                width: endX - startX + 1,
                height: endY - startY + 1
              }}
              image={image}
            />
          </Layer>
          {drawGrid && (
            <DrawGridLayer
              totalSize={baseSize + borderSize}
              canvasSize={canvasSize}
            />
          )}
          {drawCoords && (
            <DrawCoordsLayer
              totalSize={baseSize + borderSize}
              canvasSize={canvasSize}
            />
          )}
        </Stage>
      </div>
    </div>
  );
};

export default MapEditor;
export type { Props };
