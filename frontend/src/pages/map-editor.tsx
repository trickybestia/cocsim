import { useEffect, useState } from "react";

import Header from "../components/Header";
import MapEditor from "../components/MapEditor";
import readFiles from "../utils/read-files";

const MapEditorPage: React.FC = () => {
  const [image, setImage] = useState<HTMLImageElement | undefined>(undefined);

  const onCreateNewButtonClick = () => {
    readFiles(
      (images) => {
        const image = new Image();

        image.src = URL.createObjectURL(images[0]);
        image.addEventListener("load", () => setImage(image));
      },
      "image/*",
      false
    );
  };

  useEffect(() => {
    if (image === undefined) return;

    const onBeforeUnload = (e: BeforeUnloadEvent) => {
      e.preventDefault();
      e.returnValue = "";
    };

    window.addEventListener("beforeunload", onBeforeUnload);

    return () => {
      window.removeEventListener("beforeunload", onBeforeUnload);
    };
  }, [image]);

  return (
    <>
      <Header />
      <main className="grow p-4">
        {image === undefined ? (
          <div className="absolute top-0 left-0 h-full w-full">
            <div className="relative top-[40%] left-1/2 flex -translate-1/2 flex-col items-center gap-2 text-center">
              <h1 className="text-3xl font-semibold">Map editor</h1>
              <button
                className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
                onClick={onCreateNewButtonClick}
              >
                Create new
              </button>
              <button className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600">
                Open existing
              </button>
            </div>
          </div>
        ) : (
          <div className="flex h-full flex-col items-center">
            <div className="w-full grow lg:max-w-[var(--breakpoint-lg)]">
              <MapEditor image={image} />
            </div>
          </div>
        )}
      </main>
    </>
  );
};

export default MapEditorPage;
