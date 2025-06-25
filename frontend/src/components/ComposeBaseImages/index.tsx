import { useState } from "react";

import composeBaseImages from "../../utils/compose-base-images";
import readFiles from "../../utils/read-files";
import GridImage from "./GridImage";

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
    <button
      className="cursor-pointer font-bold text-base bg-blue-400 hover:bg-blue-600 text-white px-2 py-1"
      key="0"
      onClick={() => addImages(leftImages, setLeftImages, 0)}
    >
      +
    </button>
  ];
  const rightColumn = [
    <button
      className="cursor-pointer font-bold text-base bg-blue-400 hover:bg-blue-600 text-white px-2 py-1"
      key="0"
      onClick={() => addImages(rightImages, setRightImages, 0)}
    >
      +
    </button>
  ];

  for (let i = 0; i != leftImages.length; i++) {
    const url = leftImages[i].url;

    const onAddImagesButtonClick = () =>
      addImages(leftImages, setLeftImages, i + 1);

    leftColumn.push(
      <GridImage key={url + "_img"} src={url} sideText="to right" />
    );
    leftColumn.push(
      <button
        className="cursor-pointer font-bold text-base bg-blue-400 hover:bg-blue-600 text-white px-2 py-1"
        key={url + "_btn"}
        onClick={onAddImagesButtonClick}
      >
        +
      </button>
    );
  }

  for (let i = 0; i != rightImages.length; i++) {
    const url = rightImages[i].url;

    const onAddImagesButtonClick = () =>
      addImages(rightImages, setRightImages, i + 1);

    rightColumn.push(
      <GridImage key={url + "_img"} src={url} sideText="to left" />
    );
    rightColumn.push(
      <button
        className="cursor-pointer font-bold text-base bg-blue-400 hover:bg-blue-600 text-white px-2 py-1"
        key={url + "_btn"}
        onClick={onAddImagesButtonClick}
      >
        +
      </button>
    );
  }

  const onComposeButtonClick = () => {
    props.onComposed(
      composeBaseImages(
        leftImages.map((image) => image.blob),
        rightImages.map((image) => image.blob)
      )
    );
  };

  return (
    <div className="flex flex-col gap-2 items-end">
      <button
        className="cursor-pointer font-bold text-base bg-blue-400 hover:bg-blue-600 text-white px-2 py-1"
        onClick={onComposeButtonClick}
      >
        Compose
      </button>
      <div className="flex gap-2 w-full">
        <div className="flex flex-col gap-2 flex-1">{leftColumn}</div>
        <div className="flex flex-col gap-2 flex-1">{rightColumn}</div>
      </div>
    </div>
  );
};

export default Foo;
export type { Props };
