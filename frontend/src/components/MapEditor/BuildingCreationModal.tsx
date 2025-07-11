import { useState } from "react";
import Modal from "react-modal";

import useBuildingTypes from "../../hooks/use-building-types";
import type { BuildingType } from "../../types";
import getBuildingTypesWithSize from "../../utils/get-building-types-with-size";
import StringSelector from "../StringSelector";

type Props = {
  isOpen: boolean;
  selection: { width: number; height: number } | undefined;
  onClose: (
    building: { buildingType: BuildingType; level: number } | undefined
  ) => void;
};

const BuildingCreationModal: React.FC<Props> = ({
  isOpen,
  selection,
  onClose
}: Props) => {
  const [selectedBuildingType, setSelectedBuildingType] = useState<
    BuildingType | undefined
  >(undefined);
  const { buildingTypes, getBuildingType } = useBuildingTypes();

  if (!isOpen && selectedBuildingType !== undefined) {
    setSelectedBuildingType(undefined);
  }

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
      {isOpen && selectedBuildingType === undefined && (
        <StringSelector
          values={availableBuildingTypes.map(
            (buildingType) => buildingType.name
          )}
          onSelected={(value) =>
            setSelectedBuildingType(getBuildingType(value))
          }
          inputPlaceholder="search for a building"
        />
      )}
      {isOpen && selectedBuildingType !== undefined && (
        <StringSelector
          values={Array(selectedBuildingType.levels)
            .fill(0)
            .map((_, index) => (index + 1).toString())}
          onSelected={(value) =>
            onClose({
              buildingType: selectedBuildingType,
              level: parseInt(value) - 1
            })
          }
          inputPlaceholder="select level"
        />
      )}
    </Modal>
  );
};

export default BuildingCreationModal;
export type { Props };
