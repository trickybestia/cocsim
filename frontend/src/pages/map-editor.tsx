import { saveAs } from "file-saver";
import { useEffect, useState } from "react";

import Header from "../components/Header";
import MapEditor from "../components/MapEditor";
import { BuildingTypesContext } from "../hooks/use-building-types";
import useBuildingTypesSWR from "../hooks/use-building-types-swr";
import { UnitTypesContext } from "../hooks/use-unit-types";
import useUnitTypesSWR from "../hooks/use-unit-types-swr";
import type { Map } from "../types";
import { exportToZip, importFromZip } from "../utils/map-editor";
import readFiles from "../utils/read-files";

const MapEditorPage: React.FC = () => {
  const [loadedData, setLoadedData] = useState<
    | { image: HTMLImageElement; imageBlob: Blob; map: Map | undefined }
    | undefined
  >(undefined);
  const buildingTypes = useBuildingTypesSWR();
  const unitTypes = useUnitTypesSWR();

  const onCreateNewButtonClick = () => {
    readFiles(
      (images) => {
        const image = new Image();

        image.src = URL.createObjectURL(images[0]);
        image.addEventListener("load", () =>
          setLoadedData({ image: image, imageBlob: images[0], map: undefined })
        );
      },
      "image/*",
      false
    );
  };

  const onOpenExistingButtonClick = () => {
    readFiles(
      (files) => {
        importFromZip(files[0]).then((data) => {
          const image = new Image();

          image.src = URL.createObjectURL(data.image);
          image.addEventListener("load", () =>
            setLoadedData({
              image: image,
              imageBlob: data.image,
              map: data.map
            })
          );
        });
      },
      "application/zip",
      false
    );
  };

  if (import.meta.env.PROD) {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    useEffect(() => {
      if (loadedData === undefined) return;

      const onBeforeUnload = (e: BeforeUnloadEvent) => {
        e.preventDefault();
        e.returnValue = "";
      };

      window.addEventListener("beforeunload", onBeforeUnload);

      return () => {
        window.removeEventListener("beforeunload", onBeforeUnload);
      };
    }, [loadedData]);
  }

  return (
    <>
      <Header />
      <main className="grow p-4">
        {loadedData === undefined ? (
          <div className="absolute top-0 left-0 h-full w-full">
            <div className="relative top-[40%] left-1/2 flex -translate-1/2 flex-col items-center gap-2 text-center">
              <h1 className="text-3xl font-semibold">Map editor</h1>
              <button
                className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
                onClick={onCreateNewButtonClick}
              >
                Create new (choose base image)
              </button>
              <button
                className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
                onClick={onOpenExistingButtonClick}
              >
                Open existing (choose .zip file)
              </button>
            </div>
          </div>
        ) : (
          buildingTypes !== undefined &&
          unitTypes !== undefined && (
            <div className="flex h-full flex-col items-center">
              <div className="w-full grow lg:max-w-[var(--breakpoint-lg)]">
                <BuildingTypesContext value={buildingTypes}>
                  <UnitTypesContext value={unitTypes}>
                    <MapEditor
                      image={loadedData.image}
                      imageBlob={loadedData.imageBlob}
                      map={loadedData.map}
                      onExport={(map, imageUrl) =>
                        exportToZip(map, imageUrl).then((zip) => {
                          saveAs(
                            zip,
                            `cocsim-map-${new Date().toISOString()}.zip`
                          );
                        })
                      }
                    />
                  </UnitTypesContext>
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
