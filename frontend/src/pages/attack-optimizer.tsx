import { useEffect, useState } from "react";
import useWebSocket from "react-use-websocket";
import { twJoin } from "tailwind-merge";

import { getOptimizeAttackWebSocketUrl } from "../api";
import ArmyEditor from "../components/ArmyEditor";
import GameRenderer from "../components/GameRenderer";
import Header from "../components/Header";
import { UnitTypesContext } from "../hooks/use-unit-types";
import useUnitTypesSWR from "../hooks/use-unit-types-swr";
import type { Map, OptimizeAttackMessage, UnitWithCount } from "../types";
import { importFromZip } from "../utils/map-editor";
import readFiles from "../utils/read-files";

const AttackOptimizerPage: React.FC = () => {
  const unitTypes = useUnitTypesSWR();
  const [units, setUnits] = useState<UnitWithCount[] | undefined>(undefined);
  const [optimizationStarted, setOptimizationStarted] = useState(false);
  const [progressMessageHistory, setMessageHistory] = useState<string[]>([]);
  const [mapData, setMapData] = useState<
    { map: Map; image: HTMLImageElement } | undefined
  >(undefined);

  const { lastJsonMessage } = useWebSocket<OptimizeAttackMessage | null>(
    getOptimizeAttackWebSocketUrl(),
    {
      onOpen: (event) => {
        const webSocket: WebSocket = event.target as WebSocket;

        webSocket.send(JSON.stringify(mapData!.map));
        webSocket.send(JSON.stringify(units));
        console.log(units);
      }
    },
    optimizationStarted
  );

  const gameRendererFrames =
    lastJsonMessage !== null && lastJsonMessage.type === "result"
      ? lastJsonMessage.result
      : undefined;

  useEffect(() => {
    if (lastJsonMessage !== null && lastJsonMessage.type === "progress") {
      const progressMessage = `[${new Date().toLocaleTimeString()}] ${lastJsonMessage.progress}`;

      setMessageHistory((prev) => prev.concat(progressMessage));
    }
  }, [lastJsonMessage]);

  const onOpenMapButtonClick = () => {
    readFiles(
      (files) => {
        importFromZip(files[0]).then((data) => {
          const image = new Image();

          image.src = URL.createObjectURL(data.image);
          image.addEventListener("load", () =>
            setMapData({ map: data.map, image: image })
          );
        });
      },
      "application/zip",
      false
    );
  };

  return (
    <>
      <Header />
      <main className="grow p-4">
        {mapData === undefined ? (
          <div className="absolute top-0 left-0 h-full w-full">
            <div className="relative top-[40%] left-1/2 flex -translate-1/2 flex-col items-center gap-2 text-center">
              <h1 className="text-3xl font-semibold">Attack optimizer</h1>
              <button
                className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
                onClick={onOpenMapButtonClick}
              >
                Open map (choose .zip file)
              </button>
            </div>
          </div>
        ) : (
          <div className="flex h-full flex-col items-center">
            <div className="w-full grow lg:max-w-[var(--breakpoint-lg)]">
              {!optimizationStarted ? (
                unitTypes !== undefined && (
                  <div className="flex flex-col gap-2">
                    <h3 className="text-xl">Choose troops</h3>
                    <UnitTypesContext value={unitTypes}>
                      <ArmyEditor
                        units={units === undefined ? [] : units}
                        setUnits={setUnits}
                      />
                      <button
                        onClick={() => {
                          if (units !== undefined && units.length != 0) {
                            setOptimizationStarted(true);
                          }
                        }}
                        className="w-min cursor-pointer bg-blue-400 px-2 py-1 text-sm font-bold text-white hover:bg-blue-600"
                      >
                        OK
                      </button>
                    </UnitTypesContext>
                  </div>
                )
              ) : (
                <div className="flex h-full gap-2">
                  <div className="flex grow-1 basis-0 flex-col gap-2">
                    <div className="flex flex-col gap-2">
                      <h4 className="text-lg">Troops:</h4>
                      <div>
                        {units!.map((unit, index) => (
                          <p
                            key={index}
                            className={twJoin(
                              index % 2 == 1 ? "bg-gray-200" : "bg-gray-100",
                              "px-1 py-0.5"
                            )}
                          >
                            {unit.count}x {unit.unit.name} lvl.{" "}
                            {unit.unit.level + 1}
                          </p>
                        ))}
                      </div>
                    </div>
                    <div className="font-mono text-sm">
                      {progressMessageHistory.map((progressMessage, index) => (
                        <p
                          key={index}
                          className={twJoin(
                            index % 2 == 1 ? "bg-gray-200" : "bg-gray-100",
                            "px-1 py-0.5"
                          )}
                        >
                          {progressMessage}
                        </p>
                      ))}
                    </div>
                  </div>
                  {gameRendererFrames !== undefined && (
                    <GameRenderer
                      className="grow-[3] basis-0"
                      frames={gameRendererFrames}
                      baseImage={mapData.image}
                    />
                  )}
                </div>
              )}
            </div>
          </div>
        )}
      </main>
    </>
  );
};

export default AttackOptimizerPage;
