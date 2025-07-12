import { useEffect, useState } from "react";
import useWebSocket from "react-use-websocket";

import { getOptimizeAttackWebSocketUrl } from "../api";
import ArmyEditor from "../components/ArmyEditor";
import GameRenderer from "../components/GameRenderer";
import Header from "../components/Header";
import { UnitTypesContext } from "../hooks/use-unit-types";
import useUnitTypesSWR from "../hooks/use-unit-types-swr";
import type { Map, OptimizeAttackMessage, Unit } from "../types";
import { importFromZip } from "../utils/map-editor";
import readFiles from "../utils/read-files";

const AttackOptimizerPage: React.FC = () => {
  const unitTypes = useUnitTypesSWR();
  const [units, setUnits] = useState<Unit[] | undefined>(undefined);
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
        webSocket.send(JSON.stringify(units!));
      }
    },
    units !== undefined
  );

  const gameRendererFrames =
    lastJsonMessage !== null && lastJsonMessage.type === "result"
      ? lastJsonMessage.result
      : undefined;

  useEffect(() => {
    if (lastJsonMessage !== null && lastJsonMessage.type === "progress") {
      const progressMessage = `${new Date().toLocaleTimeString()}: ${lastJsonMessage.progress}`;

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
        <div className="flex h-full flex-col items-center">
          <div className="w-full grow lg:max-w-[var(--breakpoint-lg)]">
            {mapData === undefined ? (
              <button
                className="cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
                onClick={onOpenMapButtonClick}
              >
                Open map
              </button>
            ) : units === undefined ? (
              unitTypes !== undefined && (
                <UnitTypesContext value={unitTypes}>
                  <ArmyEditor onOk={setUnits} />
                </UnitTypesContext>
              )
            ) : gameRendererFrames === undefined ? (
              <>
                {progressMessageHistory.map((progressMessage, index) => (
                  <p key={index}>{progressMessage}</p>
                ))}
              </>
            ) : (
              <GameRenderer
                frames={gameRendererFrames}
                baseImage={mapData.image}
              />
            )}
          </div>
        </div>
      </main>
    </>
  );
};

export default AttackOptimizerPage;
