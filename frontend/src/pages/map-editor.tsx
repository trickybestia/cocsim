import { saveAs } from "file-saver";
import { useEffect, useState } from "react";

import Header from "../components/Header";
import MapEditor from "../components/MapEditor";
import { BuildingTypesContext } from "../hooks/use-building-types";
import useBuildingTypesSWR from "../hooks/use-building-types-swr";
import { exportToZip } from "../utils/map-editor";
import readFiles from "../utils/read-files";

const MapEditorPage: React.FC = () => {
  const [image, setImage] = useState<
    { image: HTMLImageElement; imageBlob: Blob } | undefined
  >(undefined);
  const buildingTypes = useBuildingTypesSWR();

  const onCreateNewButtonClick = () => {
    readFiles(
      (images) => {
        const image = new Image();

        image.src = URL.createObjectURL(images[0]);
        image.addEventListener("load", () =>
          setImage({ image: image, imageBlob: images[0] })
        );
      },
      "image/*",
      false
    );
  };

  if (import.meta.env.PROD) {
    // eslint-disable-next-line react-hooks/rules-of-hooks
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
  }

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
          buildingTypes !== undefined && (
            <div className="flex h-full flex-col items-center">
              <div className="w-full grow lg:max-w-[var(--breakpoint-lg)]">
                <BuildingTypesContext value={buildingTypes}>
                  <MapEditor
                    image={image.image}
                    imageBlob={image.imageBlob}
                    onExport={(map, imageUrl) =>
                      exportToZip(map, imageUrl).then((zip) => {
                        saveAs(
                          zip,
                          `cocsim-map-${new Date().toISOString()}.zip`
                        );
                      })
                    }
                  />
                </BuildingTypesContext>
              </div>
            </div>
          )
        )}
      </main>
    </>
  );
};

export default MapEditorPage;
