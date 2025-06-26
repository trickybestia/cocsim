import { useState } from "react";
import { twMerge } from "tailwind-merge";

type Props = React.ImgHTMLAttributes<HTMLImageElement>;

const MaximizableImage: React.FC<Props> = ({
  className,

  ...props
}: Props) => {
  const [isMaximized, setIsMaximized] = useState(false);

  const onClick = () => setIsMaximized(!isMaximized);

  return (
    <>
      <img
        className={twMerge(className, "cursor-zoom-in")}
        onClick={onClick}
        {...props}
      />
      {isMaximized && (
        <div
          className="fixed top-0 left-0 z-40 h-full w-full cursor-zoom-out p-[5%] backdrop-brightness-25"
          onClick={onClick}
        >
          <img
            src={props.src}
            className="relative z-50 h-full w-full object-contain"
          />
        </div>
      )}
    </>
  );
};

export default MaximizableImage;
export type { Props };
