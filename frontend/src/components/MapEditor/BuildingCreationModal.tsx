import { useContext, useState } from "react";
import Modal from "react-modal";

import { GameTypesContext } from "../../hooks/use-game-types";
import type { BuildingType } from "../../types";
import getBuildingTypesWithSize from "../../utils/get-building-types-with-size";
import getGameType from "../../utils/get-game-type";
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
  const gameTypes = useContext(GameTypesContext);

  const [selectedBuildingType, setSelectedBuildingType] = useState<
    BuildingType | undefined
  >(undefined);

  if (!isOpen && selectedBuildingType !== undefined) {
    setSelectedBuildingType(undefined);
  }

  const availableBuildingTypes =
    selection === undefined
      ? []
      : getBuildingTypesWithSize(
          gameTypes.buildings,
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
            setSelectedBuildingType(getGameType(gameTypes.buildings, value))
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
