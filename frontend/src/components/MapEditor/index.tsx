import Konva from "konva";
import type { KonvaEventObject } from "konva/lib/Node";
import { create } from "mutative";
import { useContext, useEffect, useRef, useState } from "react";
import { Image, Layer, Rect, Stage } from "react-konva";
import { twMerge } from "tailwind-merge";

import { BuildingTypesContext } from "../../hooks/use-building-types";
import type { Building, BuildingType, Map } from "../../types";
import clamp from "../../utils/clamp";
import {
  checkIntersection,
  createBuildingsGrid,
  cropImage
} from "../../utils/map-editor";
import sortSelection from "../../utils/sort-selection";
import Hint from "../Hint";
import IntegerNumberInput from "../IntegerNumberInput";
import BuildingCreationModal from "./BuildingCreationModal";
import BuildingOptionsEditor from "./BuildingOptionsEditor";
import BuildingsLayer from "./BuildingsLayer";
import DrawCoordsLayer from "./DrawCoordsLayer";
import DrawGridLayer from "./DrawGridLayer";

type Props = React.HTMLAttributes<HTMLDivElement> & {
  image: HTMLImageElement;
  imageBlob: Blob;
  map?: Map;
  onExport: (map: Map, image: Blob) => void;
};

const MapEditor: React.FC<Props> = ({
  className,

  image,
  imageBlob,
  map,
  onExport,

  ...props
}: Props) => {
  const canvasWrapperRef = useRef<HTMLDivElement | null>(null);
  const canvasRef = useRef<Konva.Stage>(null);
  const [canvasSize, setCanvasSize] = useState(1);

  useEffect(() => {
    const onResize = () => {
      if (canvasWrapperRef.current === null) return;

      setCanvasSize(
        Math.min(
          canvasWrapperRef.current.offsetHeight,
          canvasWrapperRef.current.offsetWidth
        )
      );
    };

    onResize();

    window.addEventListener("resize", onResize);

    return () => {
      window.removeEventListener("resize", onResize);
    };
  }, []);

  const [buildings, setBuildings] = useState<Building[]>(
    map === undefined ? [] : map.buildings
  );

  const [drawGrid, setDrawGrid] = useState(false);
  const [drawCoords, setDrawCoords] = useState(false);
  const [baseSize, setBaseSize] = useState(
    map === undefined ? 44 : map.baseSize
  );
  const [borderSize, setBorderSize] = useState(
    map === undefined ? 4 : map.borderSize
  );
  const [startX, setStartX] = useState(0);
  const [startY, setStartY] = useState(0);
  const [endX, setEndX] = useState(image.width - 1);
  const [endY, setEndY] = useState(image.height - 1);

  const [cursorPosition, setCursorPosition] = useState<
    { x: number; y: number } | undefined
  >(undefined);
  const [selectionStartPosition, setSelectionStartPosition] = useState<
    { x: number; y: number } | undefined
  >(undefined);
  const [selectedBuildingPosition, setSelectedBuildingPosition] = useState<
    { x: number; y: number } | undefined
  >(undefined);
  const [isBuildingCreationModalOpen, setIsBuildingCreationModalOpen] =
    useState(false);

  const buildingTypes = useContext(BuildingTypesContext);
  const pixelsPerTile = canvasSize / (baseSize + 2 * borderSize);
  const buildingsGrid = createBuildingsGrid(
    buildings,
    buildingTypes,
    baseSize + 2 * borderSize
  );
  const highlightedBuilding =
    cursorPosition === undefined
      ? undefined
      : buildingsGrid[cursorPosition.x][cursorPosition.y];
  const selectedBuilding =
    selectedBuildingPosition === undefined
      ? undefined
      : buildingsGrid[selectedBuildingPosition.x][selectedBuildingPosition.y];
  const selectedBuildingIndex =
    selectedBuilding === undefined
      ? undefined
      : buildings.indexOf(selectedBuilding);
  const selection = sortSelection(cursorPosition, selectionStartPosition);
  const selectionIntersectsBuilding =
    selection === undefined
      ? false
      : checkIntersection(buildingsGrid, selection);

  const canvasOnWheel = (e: KonvaEventObject<WheelEvent>) => {
    const stage = e.target.getStage();

    if (stage === null) return;

    e.evt.preventDefault();

    const oldScale = stage.scaleX();
    const pointer = stage.getPointerPosition()!;

    const mousePointTo = {
      x: (pointer.x - stage.x()) / oldScale,
      y: (pointer.y - stage.y()) / oldScale
    };

    // how to scale? Zoom in? Or zoom out?
    const direction = e.evt.deltaY > 0 ? -1 : 1;

    const STAGE_SCALE_FACTOR = 1.1;

    const newScale = clamp(
      0.5,
      direction > 0
        ? oldScale * STAGE_SCALE_FACTOR
        : oldScale / STAGE_SCALE_FACTOR,
      10
    );
    stage.scale({ x: newScale, y: newScale });

    const newPos = {
      x: pointer.x - mousePointTo.x * newScale,
      y: pointer.y - mousePointTo.y * newScale
    };

    stage.position(newPos);
  };

  const canvasOnPointerMove = (e: KonvaEventObject<PointerEvent>) => {
    const stage = e.target.getStage();

    if (stage === null) return;

    if (e.evt.buttons === 0) {
      const pointer = stage.getRelativePointerPosition();

      if (pointer === null) return;

      if (
        pointer.x < 0 ||
        pointer.x >= canvasSize ||
        pointer.y < 0 ||
        pointer.y >= canvasSize
      ) {
        setCursorPosition(undefined);

        return;
      }

      const tileX = Math.floor(pointer.x / pixelsPerTile);
      const tileY = Math.floor(pointer.y / pixelsPerTile);

      setCursorPosition({ x: tileX, y: tileY });

      return;
    }

    if ((e.evt.buttons & 0x4) !== 0) {
      e.evt.preventDefault();

      const position = stage.getPosition();

      stage.position({
        x: position.x + e.evt.movementX,
        y: position.y + e.evt.movementY
      });
    }
  };

  const canvasOnClick = (e: KonvaEventObject<MouseEvent>) => {
    if (cursorPosition === undefined)
      // clicked on map background
      return;

    if (e.evt.button === 0) {
      if (selectionStartPosition === undefined) {
        if (highlightedBuilding !== undefined) {
          setSelectedBuildingPosition({
            x: highlightedBuilding.x,
            y: highlightedBuilding.y
          });
        } else {
          setSelectionStartPosition(cursorPosition);
        }
      } else if (!selectionIntersectsBuilding) {
        // create new building

        setIsBuildingCreationModalOpen(true);
      }

      return;
    }

    if (e.evt.button === 2) {
      if (selectionStartPosition !== undefined) {
        setSelectionStartPosition(undefined);
      } else {
        // remove selected building

        const building = buildingsGrid[cursorPosition.x][cursorPosition.y];

        if (building === undefined) return;

        const newBuildings = buildings.slice();

        newBuildings.splice(
          buildings.findIndex((b) => b.x === building.x && b.y === building.y)!,
          1
        );

        setBuildings(newBuildings);
      }
    }
  };

  const onResetCameraButtonClick = () => {
    if (canvasRef.current === null) return;

    canvasRef.current.scale({ x: 1, y: 1 });
    canvasRef.current.setPosition({ x: 0, y: 0 });
  };

  const onExportButtonClick = async () => {
    let exportImageBlob;

    if (
      startX === 0 &&
      startY === 0 &&
      endX === image.width - 1 &&
      endY === image.height - 1
    ) {
      exportImageBlob = imageBlob;
    } else {
      exportImageBlob = await cropImage(
        image,
        startX,
        startY,
        endX - startX + 1,
        endY - startY + 1
      );
    }

    onExport(
      {
        baseSize: baseSize,
        borderSize: borderSize,
        buildings: buildings
      },
      exportImageBlob
    );
  };

  /**
   * BuildingSelectionModal callback
   */
  const onBuildingCreationModalClose = (
    building: { buildingType: BuildingType; level: number } | undefined
  ) => {
    setIsBuildingCreationModalOpen(false);
    setSelectionStartPosition(undefined);

    if (building === undefined) return;

    const newBuildings = buildings.slice();

    if (building.buildingType.name === "Wall") {
      for (
        let tileX = selection!.leftTop.x;
        tileX != selection!.rightBottom.x + 1;
        tileX++
      ) {
        for (
          let tileY = selection!.leftTop.y;
          tileY != selection!.rightBottom.y + 1;
          tileY++
        ) {
          newBuildings.push({
            name: "Wall",
            x: tileX,
            y: tileY,
            level: building.level
          });
        }
      }
    } else {
      const options: { [option: string]: unknown } = {};

      building.buildingType.options.forEach(({ name, values }) => {
        options[name] = values[0];
      });

      if (building.buildingType.name === "ClanCastle") {
        options["units"] = [];
      }

      newBuildings.push({
        name: building.buildingType.name,
        x: selection!.leftTop.x,
        y: selection!.leftTop.y,
        level: building.level,
        ...options
      });
    }

    setBuildings(newBuildings);
    setSelectedBuildingPosition({
      x: selection!.leftTop.x,
      y: selection!.leftTop.y
    });
  };

  return (
    <div className="flex h-full w-full flex-col gap-2">
      <Hint>
        Click LMB on canvas to create buildings, RMB to remove, drag with mouse
        wheel pressed to move, scroll mouse wheel to zoom
      </Hint>
      <div
        className={twMerge(
          className,
          "flex grow justify-between gap-2",
          highlightedBuilding !== undefined && "cursor-pointer"
        )}
        {...props}
      >
        <div className="flex grow-[0.3] basis-0 flex-col gap-1">
          <div className="grid-col grid grid-cols-[auto_min-content] gap-1 text-end text-nowrap">
            <p>Draw grid:</p>
            <input
              type="checkbox"
              checked={drawGrid}
              onChange={(e) => setDrawGrid(e.target.checked)}
            />
            <p>Draw coords:</p>
            <input
              type="checkbox"
              checked={drawCoords}
              onChange={(e) => setDrawCoords(e.target.checked)}
            />
            <IntegerNumberInput
              text="Base size:"
              min={1}
              max={44}
              defaultValue={baseSize}
              onChange={setBaseSize}
            />
            <IntegerNumberInput
              text="Border size:"
              min={2}
              max={4}
              defaultValue={borderSize}
              onChange={setBorderSize}
            />
            <IntegerNumberInput
              text="Start X:"
              min={0}
              max={image.width - 1}
              defaultValue={startX}
              onChange={setStartX}
            />
            <IntegerNumberInput
              text="Start Y:"
              min={0}
              max={image.width - 1}
              defaultValue={startY}
              onChange={setStartY}
            />
            <IntegerNumberInput
              text="End X:"
              min={0}
              max={image.width - 1}
              defaultValue={endX}
              onChange={setEndX}
            />
            <IntegerNumberInput
              text="End Y:"
              min={0}
              max={image.width - 1}
              defaultValue={endY}
              onChange={setEndY}
            />
            <button
              className="col-span-2 cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
              onClick={onResetCameraButtonClick}
            >
              Reset camera
            </button>
            <Hint className="col-span-2 text-left text-sm">
              Click Export to download .zip
            </Hint>
            <button
              className="col-span-2 cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
              onClick={onExportButtonClick}
            >
              Export
            </button>
          </div>

          {selectedBuilding !== undefined && (
            <BuildingOptionsEditor
              key={`${selectedBuilding.x},${selectedBuilding.y}`} // to reset <input/> values on selected building change
              building={selectedBuilding}
              onChange={(value) =>
                setBuildings(
                  create(buildings, (draft) => {
                    draft[selectedBuildingIndex!] = value;
                  })
                )
              }
            />
          )}
        </div>

        <div
          className="relative flex grow-[0.7] justify-around"
          ref={canvasWrapperRef}
        >
          <Stage
            className="absolute bg-green-900"
            ref={canvasRef}
            width={canvasSize}
            height={canvasSize}
            onWheel={canvasOnWheel}
            onPointerMove={canvasOnPointerMove}
            onClick={canvasOnClick}
            onContextMenu={(e) => e.evt.preventDefault()}
            listening={false}
          >
            <Layer>
              <Image
                scaleX={canvasSize / image.width}
                scaleY={canvasSize / image.height}
                crop={{
                  x: startX,
                  y: startY,
                  width: endX - startX + 1,
                  height: endY - startY + 1
                }}
                image={image}
              />
            </Layer>
            {drawGrid && (
              <DrawGridLayer
                totalSize={baseSize + 2 * borderSize}
                canvasSize={canvasSize}
              />
            )}
            {drawCoords && (
              <DrawCoordsLayer
                totalSize={baseSize + 2 * borderSize}
                canvasSize={canvasSize}
              />
            )}
            <BuildingsLayer
              buildings={buildings}
              selectedBuilding={selectedBuilding}
              pixelsPerTile={pixelsPerTile}
            />
            <Layer>
              {cursorPosition !== undefined && (
                <Rect
                  x={cursorPosition.x * pixelsPerTile}
                  y={cursorPosition.y * pixelsPerTile}
                  width={pixelsPerTile}
                  height={pixelsPerTile}
                  stroke="black"
                  strokeWidth={1}
                />
              )}
              {selection !== undefined && (
                <Rect
                  x={selection.leftTop.x * pixelsPerTile}
                  y={selection.leftTop.y * pixelsPerTile}
                  width={
                    (selection.rightBottom.x - selection.leftTop.x + 1) *
                    pixelsPerTile
                  }
                  height={
                    (selection.rightBottom.y - selection.leftTop.y + 1) *
                    pixelsPerTile
                  }
                  stroke={selectionIntersectsBuilding ? "red" : "black"}
                  strokeWidth={1}
                />
              )}
            </Layer>
          </Stage>
        </div>

        <BuildingCreationModal
          isOpen={isBuildingCreationModalOpen}
          selection={
            selection === undefined
              ? undefined
              : {
                  width: selection.rightBottom.x - selection.leftTop.x + 1,
                  height: selection.rightBottom.y - selection.leftTop.y + 1
                }
          }
          onClose={onBuildingCreationModalClose}
        />
      </div>
    </div>
  );
};

export default MapEditor;
export type { Props };
