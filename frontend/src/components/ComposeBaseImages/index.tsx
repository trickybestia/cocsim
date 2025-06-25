import { useState } from "react";
import { twMerge } from "tailwind-merge";

import composeBaseImages from "../../utils/compose-base-images";
import readFiles from "../../utils/read-files";
import GridImage from "./GridImage";

type URLBlob = { blob: Blob; url: string };

type Props = React.HTMLAttributes<HTMLDivElement> & {
  onComposed?: (image: Blob) => void;
};

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
      className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
      key="0"
      onClick={() => addImages(leftImages, setLeftImages, 0)}
    >
      +
    </button>
  ];
  const rightColumn = [
    <button
      className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
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
        className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
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
        className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
        key={url + "_btn"}
        onClick={onAddImagesButtonClick}
      >
        +
      </button>
    );
  }

  const onComposeButtonClick = () => {
    props.onComposed?.(
      composeBaseImages(
        leftImages.map((image) => image.blob),
        rightImages.map((image) => image.blob)
      )
    );
  };

  return (
    <div
      {...props}
      className={twMerge(
        "flex flex-col items-end gap-2 overflow-y-scroll",
        props.className
      )}
    >
      <button
        className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
        onClick={onComposeButtonClick}
      >
        Compose
      </button>
      <div className="flex w-full gap-2">
        <div className="flex flex-1 flex-col gap-2">{leftColumn}</div>
        <div className="flex flex-1 flex-col gap-2">{rightColumn}</div>
      </div>
    </div>
  );
};

export default Foo;
export type { Props };
