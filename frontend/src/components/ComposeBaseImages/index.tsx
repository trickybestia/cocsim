import { useState } from "react";
import { twJoin, twMerge } from "tailwind-merge";

import { composeBaseImages, reverseProjection } from "../../api";
import readFiles from "../../utils/read-files";
import GridImage from "./GridImage";

type URLBlob = { blob: Blob; url: string };

type Props = React.HTMLAttributes<HTMLDivElement> & {
  onDone?: (image: Blob) => void;
};

const ComposeBaseImages: React.FC<Props> = ({
  className,

  onDone,

  ...props
}: Props) => {
  const [leftImages, setLeftImages] = useState<URLBlob[]>([]);
  const [rightImages, setRightImages] = useState<URLBlob[]>([]);
  const [composedImage, setComposedImage] = useState<URLBlob | undefined>(
    undefined
  );
  const [isComposing, setIsComposing] = useState(false);
  const [reversedImage, setReversedImage] = useState<URLBlob | undefined>(
    undefined
  );
  const [isReversing, setIsReversing] = useState(false);

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

  const removeImage = (
    images: URLBlob[],
    setImages: React.Dispatch<React.SetStateAction<URLBlob[]>>,
    index: number
  ) => {
    const newImages = images.slice();

    newImages.splice(index, 1);

    setImages(newImages);
  };

  const moveImageUp = (
    images: URLBlob[],
    setImages: React.Dispatch<React.SetStateAction<URLBlob[]>>,
    index: number
  ) => {
    if (index === 0) return;

    const newImages = images.slice();

    newImages.splice(index, 1);
    newImages.splice(index - 1, 0, images[index]);

    setImages(newImages);
  };

  const moveImageDown = (
    images: URLBlob[],
    setImages: React.Dispatch<React.SetStateAction<URLBlob[]>>,
    index: number
  ) => {
    if (index === images.length - 1) return;

    const newImages = images.slice();

    newImages.splice(index, 1);
    newImages.splice(index + 1, 0, images[index]);

    setImages(newImages);
  };

  const moveImageSide = (
    images: URLBlob[],
    setImages: React.Dispatch<React.SetStateAction<URLBlob[]>>,
    otherImages: URLBlob[],
    setOtherImages: React.Dispatch<React.SetStateAction<URLBlob[]>>,
    index: number
  ) => {
    const newOtherImages = otherImages.slice();
    newOtherImages.splice(index, 0, images[index]);

    const newImages = images.slice();
    newImages.splice(index, 1);

    setImages(newImages);
    setOtherImages(newOtherImages);
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
      <GridImage
        key={url + "_img"}
        src={url}
        sideDirection="right"
        onClose={() => removeImage(leftImages, setLeftImages, i)}
        onUp={() => moveImageUp(leftImages, setLeftImages, i)}
        onDown={() => moveImageDown(leftImages, setLeftImages, i)}
        onSide={() =>
          moveImageSide(
            leftImages,
            setLeftImages,
            rightImages,
            setRightImages,
            i
          )
        }
      />
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
      <GridImage
        key={url + "_img"}
        src={url}
        sideDirection="left"
        onClose={() => removeImage(rightImages, setRightImages, i)}
        onUp={() => moveImageUp(rightImages, setRightImages, i)}
        onDown={() => moveImageDown(rightImages, setRightImages, i)}
        onSide={() =>
          moveImageSide(
            rightImages,
            setRightImages,
            leftImages,
            setLeftImages,
            i
          )
        }
      />
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

  const onComposedImageOk = () => {
    if (isReversing) return;

    setIsReversing(true);

    reverseProjection(composedImage!.blob)
      .then((image) =>
        setReversedImage({ blob: image, url: URL.createObjectURL(image) })
      )
      .finally(() => setIsReversing(false));
  };

  const onComposeButtonClick = () => {
    if (isComposing) return;

    setIsComposing(true);

    composeBaseImages(
      leftImages.map((image) => image.blob),
      rightImages.map((image) => image.blob)
    )
      .then((image) =>
        setComposedImage({ blob: image, url: URL.createObjectURL(image) })
      )
      .finally(() => setIsComposing(false));
  };

  return (
    <div
      className={twMerge(className, "flex flex-col gap-2 overflow-y-scroll")}
      {...props}
    >
      <div className="flex gap-2">
        {composedImage !== undefined && (
          <div className="flex shrink grow basis-0 flex-col gap-2">
            <h3 className="inline-block bg-yellow-300 px-1 text-lg">
              🔍️ Step 2: Check if composed image looks good
            </h3>
            <GridImage
              src={composedImage.url}
              onClose={() => setComposedImage(undefined)}
              onOk={onComposedImageOk}
            />
          </div>
        )}
        {reversedImage !== undefined && (
          <div className="flex shrink grow basis-0 flex-col gap-2">
            <h3 className="inline-block bg-yellow-300 px-1 text-lg">
              🔍️ Step 3: Check if rotated image looks good
            </h3>
            <GridImage
              src={reversedImage.url}
              onClose={() => setReversedImage(undefined)}
              onOk={() => onDone?.(reversedImage.blob)}
            />
          </div>
        )}
      </div>
      <div className="flex items-baseline justify-between">
        <h3 className="inline-block bg-yellow-300 px-1 text-lg">
          🔍️ Step 1: Select images to compose
        </h3>
        <button
          className={twJoin(
            !isComposing && "cursor-pointer hover:bg-blue-600",
            "bg-blue-400 px-2 py-1 text-base font-bold text-white"
          )}
          onClick={onComposeButtonClick}
        >
          {isComposing ? (
            <div
              className="text-surface inline-block h-6 w-6 animate-spin rounded-full border-4 border-solid border-current border-e-transparent align-middle motion-reduce:animate-[spin_1.5s_linear_infinite] dark:text-white"
              role="status"
            ></div>
          ) : (
            <>Compose</>
          )}
        </button>
      </div>

      <div className="flex w-full gap-2">
        <div className="flex flex-1 flex-col gap-2">{leftColumn}</div>
        <div className="flex flex-1 flex-col gap-2">{rightColumn}</div>
      </div>
    </div>
  );
};

export default ComposeBaseImages;
export type { Props };
