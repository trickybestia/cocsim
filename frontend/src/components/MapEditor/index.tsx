import { useEffect, useRef, useState } from "react";
import { Image, Layer, Stage } from "react-konva";
import { twMerge } from "tailwind-merge";

import DrawCoordsLayer from "./DrawCoordsLayer";
import DrawGridLayer from "./DrawGridLayer";
import NumberInput from "./NumberInput";

type Props = React.HTMLAttributes<HTMLDivElement> & {
  image: HTMLImageElement;
};

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
  const [startX, setStartX] = useState(0);
  const [startY, setStartY] = useState(0);
  const [endX, setEndX] = useState(image.width - 1);
  const [endY, setEndY] = useState(image.height - 1);

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

      <div ref={canvasWrapperRef} className="aspect-square bg-red-500">
        <Stage width={canvasSize} height={canvasSize}>
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
