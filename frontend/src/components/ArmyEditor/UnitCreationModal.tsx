import { useState } from "react";
import Modal from "react-modal";

import useUnitTypes from "../../hooks/use-unit-types";
import type { UnitType } from "../../types";
import StringSelector from "../StringSelector";

type Props = {
  isOpen: boolean;
  onClose: (unit: { unitType: UnitType; level: number } | undefined) => void;
};

const UnitCreationModal: React.FC<Props> = ({ isOpen, onClose }: Props) => {
  const { unitTypes, getUnitType } = useUnitTypes();
  const [selectedUnitType, setSelectedUnitType] = useState<
    UnitType | undefined
  >(undefined);

  if (!isOpen && selectedUnitType !== undefined) {
    setSelectedUnitType(undefined);
  }

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
      {isOpen && selectedUnitType === undefined && (
        <StringSelector
          values={unitTypes.map((unitType) => unitType.name)}
          onSelected={(value) => setSelectedUnitType(getUnitType(value))}
          inputPlaceholder="search for a unit"
        />
      )}
      {isOpen && selectedUnitType !== undefined && (
        <StringSelector
          values={Array(selectedUnitType.levels)
            .fill(0)
            .map((_, index) => (index + 1).toString())}
          onSelected={(value) =>
            onClose({
              unitType: selectedUnitType,
              level: parseInt(value) - 1
            })
          }
          inputPlaceholder="select level"
        />
      )}
    </Modal>
  );
};

export default UnitCreationModal;
export type { Props };
