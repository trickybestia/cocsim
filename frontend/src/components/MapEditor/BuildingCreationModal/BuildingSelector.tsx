import { useState } from "react";
import { twJoin } from "tailwind-merge";

import type { BuildingType } from "../../../types";

type Props = {
  buildingTypes: BuildingType[];
  onSelected: (buildingType: BuildingType) => void;
};

const BuildingSelector: React.FC<Props> = ({
  buildingTypes,
  onSelected
}: Props) => {
  const [selectedBuildingTypeName, setSelectedBuildingTypeName] = useState<
    string | undefined
  >(undefined);

  const displayedBuildingTypes = buildingTypes;

  const onOkButtonClick = () => {
    if (selectedBuildingTypeName !== undefined) {
      onSelected(
        buildingTypes.find(
          (buildingType) => buildingType.name === selectedBuildingTypeName
        )!
      );
    }
  };

  return (
    <div className="flex flex-col gap-2">
      <div className="flex gap-2">
        <input
          className="border border-gray-300 px-2"
          placeholder="search for a building"
          type="text"
        />
        <button
          onClick={onOkButtonClick}
          className="cursor-pointer bg-blue-400 px-2 py-1 text-sm font-bold text-white hover:bg-blue-600"
        >
          OK
        </button>
      </div>
      <div className="flex flex-col">
        {displayedBuildingTypes.map((buildingType, index) => {
          let backgroundColorStyle;

          if (buildingType.name === selectedBuildingTypeName) {
            backgroundColorStyle = "bg-blue-400";
          } else {
            backgroundColorStyle =
              index % 2 == 1 ? "bg-gray-200" : "bg-gray-100";
          }

          return (
            <p
              key={buildingType.name}
              className={twJoin(
                backgroundColorStyle,
                "cursor-pointer px-2 py-1 select-none"
              )}
              onClick={() => setSelectedBuildingTypeName(buildingType.name)}
              onDoubleClick={() => onSelected(buildingType)}
            >
              {buildingType.name}
            </p>
          );
        })}
      </div>
    </div>
  );
};

export default BuildingSelector;
export type { Props };
