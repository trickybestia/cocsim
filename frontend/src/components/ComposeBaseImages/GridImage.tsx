import React from "react";

type Props = {
  src: string;
  sideText: string;

  onClose: () => void;
  onUp: () => void;
  onDown: () => void;
  onSide: () => void;
};

const GridImage: React.FC<Props> = (props: Props) => {
  return (
    <div>
      <img src={props.src} />
    </div>
  );
};

export default GridImage;
export type { Props };
