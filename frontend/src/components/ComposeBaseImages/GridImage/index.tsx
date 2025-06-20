import React from "react";

import styles from "./index.module.scss";

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
    <div className={styles["grid-image"]}>
      <img src={props.src} />
    </div>
  );
};

export default GridImage;
export type { Props };
