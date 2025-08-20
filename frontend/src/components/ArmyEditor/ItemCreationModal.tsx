import { useState } from "react";
import Modal from "react-modal";

import type { WithLevel, WithName } from "../../types";
import getGameType from "../../utils/get-game-type";
import StringSelector from "../StringSelector";

type ItemType = {
  name: string;
  levels: number;
};

type Item = WithName & WithLevel;

type Props = {
  isOpen: boolean;
  types: ItemType[];
  onClose: (item: Item | undefined) => void;
};

const UnitCreationModal: React.FC<Props> = ({
  isOpen,
  types,
  onClose
}: Props) => {
  const [selectedType, setSelectedType] = useState<ItemType | undefined>(
    undefined
  );

  if (!isOpen && selectedType !== undefined) {
    setSelectedType(undefined);
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
      {isOpen && selectedType === undefined && (
        <StringSelector
          values={types.map((type) => type.name)}
          onSelected={(value) => {
            setSelectedType(getGameType(types, value));
          }}
          inputPlaceholder="search by name"
        />
      )}
      {isOpen && selectedType !== undefined && (
        <StringSelector
          values={Array(selectedType.levels)
            .fill(0)
            .map((_, index) => (index + 1).toString())}
          onSelected={(value) =>
            onClose({
              name: selectedType.name,
              level: parseInt(value) - 1 // "value" IS NOT raw user input, it's always valid integer (see "values" prop few lines above). TODO: Refactor this in future to use generics.
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
