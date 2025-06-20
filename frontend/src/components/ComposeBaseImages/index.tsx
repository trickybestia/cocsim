import { useState } from "react";

import composeBaseImages from "../../utils/compose-base-images";
import readFiles from "../../utils/read-files";
import GridImage from "./GridImage";
import styles from "./index.module.scss";

type URLBlob = { blob: Blob; url: string };

type Props = { onComposed: (image: Blob) => void };

const Foo: React.FC<Props> = (props: Props) => {
  const [leftImages, setLeftImages] = useState<URLBlob[]>([]);
  const [rightImages, setRightImages] = useState<URLBlob[]>([]);

  const addImages = (
    images: URLBlob[],
    setImages: React.Dispatch<React.SetStateAction<URLBlob[]>>,
    index: number
  ) => {
    readFiles((files) => {
      const blobs: URLBlob[] = files.map((file) => {
        return { blob: file, url: URL.createObjectURL(file) };
      });

      const newImages = images.slice();

      newImages.splice(index, 0, ...blobs);

      setImages(newImages);
    }, "image/*");
  };

  const leftColumn = [
    <button key="0" onClick={() => addImages(leftImages, setLeftImages, 0)}>
      +
    </button>
  ];
  const rightColumn = [
    <button key="0" onClick={() => addImages(rightImages, setRightImages, 0)}>
      +
    </button>
  ];

  for (let i = 0; i != leftImages.length; i++) {
    const url = leftImages[i].url;

    leftColumn.push(
      <GridImage
        key={url + "_img"}
        src={url}
        sideText="to right"
        onClose={() => {}}
        onUp={() => {}}
        onDown={() => {}}
        onSide={() => {}}
      />
    );
    leftColumn.push(
      <button
        key={url + "_btn"}
        onClick={() => addImages(leftImages, setLeftImages, i + 1)}
      >
        +
      </button>
    );
  }

  for (let i = 0; i != rightImages.length; i++) {
    const url = rightImages[i].url;

    rightColumn.push(
      <GridImage
        key={url + "_img"}
        src={url}
        sideText="to left"
        onClose={() => {}}
        onUp={() => {}}
        onDown={() => {}}
        onSide={() => {}}
      />
    );
    rightColumn.push(
      <button
        key={url + "_btn"}
        onClick={() => addImages(rightImages, setRightImages, i + 1)}
      >
        +
      </button>
    );
  }

  return (
    <div className={styles["compose-base-images"]}>
      <button
        className={styles["compose-button"]}
        onClick={() => {
          props.onComposed(
            composeBaseImages(
              leftImages.map((image) => image.blob),
              rightImages.map((image) => image.blob)
            )
          );
        }}
      >
        Compose
      </button>
      <div className={styles["images"]}>
        <div className={styles["column"]}>{leftColumn}</div>
        <div className={styles["column"]}>{rightColumn}</div>
      </div>
    </div>
  );
};

export default Foo;
export type { Props };
