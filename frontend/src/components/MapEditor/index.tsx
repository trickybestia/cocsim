import { useEffect, useRef, useState } from "react";
import { Image, Layer, Stage } from "react-konva";
import { twMerge } from "tailwind-merge";

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

  return (
    <div
      className={twMerge(className, "flex h-full w-full justify-between gap-2")}
      {...props}
    >
      <div>
        <div className="grid-col grid grid-cols-[auto_min-content] gap-2 text-end">
          <p>Draw grid:</p>
          <input type="checkbox" />
          <p>Base size:</p>
          <input type="number" min={1} max={44} defaultValue={44} />
          <p>Border size:</p>
          <input type="number" min={0} max={4} defaultValue={4} />
        </div>
      </div>

      <div ref={canvasWrapperRef} className="aspect-square bg-red-500">
        <Stage width={canvasSize} height={canvasSize}>
          <Layer>
            <Image
              scaleX={canvasSize / image.width}
              scaleY={canvasSize / image.width}
              image={image}
            />
          </Layer>
        </Stage>
      </div>
    </div>
  );
};

export default MapEditor;
export type { Props };
