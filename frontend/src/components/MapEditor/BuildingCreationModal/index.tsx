import Modal from "react-modal";

import type { BuildingType } from "../../../types";
import getBuildingTypesWithSize from "../../../utils/get-building-types-with-size";
import BuildingSelector from "./BuildingSelector";

type Props = {
  isOpen: boolean;
  buildingTypes: BuildingType[];
  selection: { width: number; height: number } | undefined;
  onClose: (buildingType: BuildingType | undefined) => void;
};

const BuildingCreationModal: React.FC<Props> = ({
  isOpen,
  buildingTypes,
  selection,
  onClose
}: Props) => {
  const availableBuildingTypes =
    selection === undefined
      ? []
      : getBuildingTypesWithSize(
          buildingTypes,
          selection.width,
          selection.height
        );

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
      {isOpen && (
        <BuildingSelector
          buildingTypes={availableBuildingTypes}
          onSelected={onClose}
        />
      )}
    </Modal>
  );
};

export default BuildingCreationModal;
export type { Props };
