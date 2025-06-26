import { twMerge } from "tailwind-merge";

import MaximizableImage from "../MaximizableImage";

type Props = React.HTMLAttributes<HTMLDivElement> & {
  src: string;
  sideDirection: "right" | "left";

  onClose?: () => void;
  onUp?: () => void;
  onDown?: () => void;
  onSide?: () => void;
};

const GridImage: React.FC<Props> = ({
  className,

  src,
  sideDirection,

  onClose,
  onUp,
  onDown,
  onSide,

  ...props
}: Props) => {
  return (
    <div className={twMerge(className, "relative")} {...props}>
      <MaximizableImage src={src} />
      <div className="absolute top-0 right-0 flex flex-col gap-1 p-1">
        <button
          className="block size-6 cursor-pointer bg-red-500 font-mono text-sm font-bold text-white hover:bg-red-700"
          onClick={onClose}
        >
          X
        </button>
        <button
          className="block size-6 cursor-pointer bg-blue-400 font-mono text-sm font-bold text-white hover:bg-blue-600"
          onClick={onUp}
        >
          ⋀
        </button>
        <button
          className="block size-6 cursor-pointer bg-blue-400 font-mono text-sm font-bold text-white hover:bg-blue-600"
          onClick={onDown}
        >
          ⋁
        </button>
        <button
          className={twMerge(
            sideDirection === "left" ? "rotate-90" : "-rotate-90",
            "block size-6 cursor-pointer bg-blue-400 font-mono text-sm font-bold text-white hover:bg-blue-600"
          )}
          onClick={onSide}
        >
          ⋁
        </button>
      </div>
    </div>
  );
};

export default GridImage;
export type { Props };
