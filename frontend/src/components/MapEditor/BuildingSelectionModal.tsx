import { useState } from "react";
import Modal from "react-modal";
import { twJoin } from "tailwind-merge";

import type { BuildingType } from "../../types";
import getBuildingTypesWithSize from "../../utils/get-building-types-with-size";

type Props = {
  isOpen: boolean;
  buildingTypes: BuildingType[];
  selection: { width: number; height: number } | undefined;
  onClose: (buildingType: BuildingType | undefined) => void;
};

const BuildingSelectionModal: React.FC<Props> = ({
  isOpen,
  buildingTypes,
  selection,
  onClose
}: Props) => {
  const [selectedBuildingTypeName, setSelectedBuildingTypeName] = useState<
    string | undefined
  >(undefined);

  if (!isOpen && selectedBuildingTypeName !== undefined) {
    // fucking react-modal - can't use conditional rendering
    // just joking, someone wrote it for free after all - better
    // than implementing it myself

    setSelectedBuildingTypeName(undefined);
  }

  const displayedBuildingTypes =
    selection === undefined
      ? []
      : getBuildingTypesWithSize(
          buildingTypes,
          selection.width,
          selection.height
        );

  const onOkButtonClick = () => {
    if (selectedBuildingTypeName === undefined) {
      onClose(undefined);
    } else {
      onClose(
        buildingTypes.find(
          (buildingType) => buildingType.name === selectedBuildingTypeName
        )
      );
    }
  };

  return (
    <Modal
      isOpen={isOpen}
      onRequestClose={() => {
        onClose(undefined);
      }}
      style={{
        content: {
          top: "50%",
          left: "50%",
          right: "auto",
          bottom: "auto",
          marginRight: "-50%",
          transform: "translate(-50%, -50%)"
        }
      }}
    >
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
                onDoubleClick={() => onClose(buildingType)}
              >
                {buildingType.name}
              </p>
            );
          })}
        </div>
      </div>
    </Modal>
  );
};

export default BuildingSelectionModal;
export type { Props };
