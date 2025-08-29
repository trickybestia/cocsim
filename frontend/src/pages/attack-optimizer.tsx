import { useContext, useEffect, useState } from "react";
import { twJoin } from "tailwind-merge";

import api, { type ApiStream } from "../api";
import ArmyEditor from "../components/ArmyEditor";
import GameRenderer from "../components/GameRenderer";
import Header from "../components/Header";
import { GameTypesContext } from "../hooks/use-game-types";
import type {
  Frame,
  Map,
  OptimizeAttackMessage,
  SpellWithCount,
  UnitWithCount
} from "../types";
import { importFromZip } from "../utils/map-editor";
import readFiles from "../utils/read-files";

const AttackOptimizerPage: React.FC = () => {
  const gameTypes = useContext(GameTypesContext);

  const [units, setUnits] = useState<UnitWithCount[] | undefined>(undefined);
  const [spells, setSpells] = useState<SpellWithCount[] | undefined>(undefined);
  const [optimizeAttackStream, setOptimizeAttackStream] = useState<
    ApiStream | undefined
  >(undefined);
  const [progressMessageHistory, setMessageHistory] = useState<string[]>([]);
  const [gameRendererFrames, setGameRendererFrames] = useState<
    Frame[] | undefined
  >(undefined);
  const [mapData, setMapData] = useState<
    { map: Map; image: HTMLImageElement } | undefined
  >(undefined);

  useEffect(
    () => () => {
      if (optimizeAttackStream !== undefined) {
        optimizeAttackStream.close();
      }
    },
    [optimizeAttackStream]
  );

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

  const onOptimizeAttackButtonClick = () => {
    if (units !== undefined && units.length != 0) {
      setOptimizeAttackStream(
        api.optimizeAttack.connect(
          (stream) => {
            stream.send(JSON.stringify(mapData!.map));
            stream.send(JSON.stringify(units));
            stream.send(JSON.stringify(spells));
          },
          (data) => {
            const message = JSON.parse(data) as OptimizeAttackMessage;

            if (message.type === "progress") {
              const progressMessage = `[${new Date().toLocaleTimeString()}] ${message.progress}`;

              setMessageHistory((prev) => prev.concat(progressMessage));
            } else {
              setGameRendererFrames(message.result);
            }
          }
        )
      );
    }
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
              {optimizeAttackStream === undefined ? (
                <div className="flex flex-col gap-2">
                  <h3 className="text-xl">Troops</h3>
                  <ArmyEditor
                    items={units === undefined ? [] : units}
                    setItems={setUnits}
                    types={gameTypes.units}
                  />
                  <h3 className="text-xl">Spells</h3>
                  <ArmyEditor
                    items={spells === undefined ? [] : spells}
                    setItems={setSpells}
                    types={gameTypes.spells}
                  />
                  <button
                    onClick={onOptimizeAttackButtonClick}
                    className="w-min cursor-pointer bg-blue-400 px-2 py-1 text-sm font-bold text-white hover:bg-blue-600"
                  >
                    OK
                  </button>
                </div>
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
                            {unit.count}x {unit.value.name} lvl.{" "}
                            {unit.value.level + 1}
                          </p>
                        ))}
                      </div>
                      <h4 className="text-lg">Spells:</h4>
                      <div>
                        {spells!.map((spell, index) => (
                          <p
                            key={index}
                            className={twJoin(
                              index % 2 == 1 ? "bg-gray-200" : "bg-gray-100",
                              "px-1 py-0.5"
                            )}
                          >
                            {spell.count}x {spell.value.name} lvl.{" "}
                            {spell.value.level + 1}
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
