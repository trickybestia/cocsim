import { type RefObject, useState } from "react";
import Modal from "react-modal";

import type { Building, BuildingType } from "../../types";
import getBuildingTypesWithSize from "../../utils/get-building-types-with-size";

type Props = {
  buildingTypes: BuildingType[];
  onBuildingSelected: (building: Building) => void;
  openRef: RefObject<
    ((selectionWidth: number, selectionHeight: number) => void) | undefined
  >;
};

const BuildingSelectionModal: React.FC<Props> = ({
  buildingTypes,
  onBuildingSelected,
  openRef
}: Props) => {
  const [isOpen, setIsOpen] = useState(false);
  const [availableBuildingTypes, setAvailableBuildingTypes] = useState([]);

  openRef.current = (selectionWidth: number, selectionHeight: number) => {
    setIsOpen(true);
    setAvailableBuildingTypes(getBuildingTypesWithSize(buildingTypes));
  };

  const onOkButtonClick = () => {
    setIsOpen(false);
  };

  return (
    <Modal
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
      isOpen={isOpen}
      onRequestClose={() => setIsOpen(false)}
    >
      <div className="flex gap-2">
        <input
          className="border border-gray-300"
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
    </Modal>
  );
};

export default BuildingSelectionModal;
export type { Props };
