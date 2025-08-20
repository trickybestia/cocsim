import { create } from "mutative";
import { useState } from "react";
import { twMerge } from "tailwind-merge";

import type { WithCount, WithLevel, WithName } from "../../types";
import getGameType from "../../utils/get-game-type";
import UnitCreationModal from "./ItemCreationModal";
import ItemView from "./ItemView";

type ItemType = {
  name: string;
  levels: number;
  housingSpace: number;
};

type Item = {
  value: WithLevel & WithName;
} & WithCount;

type Props = {
  className?: string;
  items: Item[];
  setItems: (items: Item[]) => void;
  types: ItemType[];
};

const ArmyEditor: React.FC<Props> = ({
  className,
  items,
  setItems,
  types
}: Props) => {
  const [isUnitCreationModalOpen, setIsUnitCreationModalOpen] = useState(false);

  let occupiedSpace = 0;

  items.forEach((item) => {
    occupiedSpace +=
      getGameType(types, item.value.name).housingSpace * item.count;
  });

  const itemViews = items.map((item, index) => (
    <ItemView
      key={index}
      value={item}
      onCountChange={(newCount) =>
        setItems(
          create(items, (draft) => {
            draft[index].count = newCount;
          })
        )
      }
      onRemove={() => {
        setItems(
          create(items, (draft) => {
            draft.splice(index, 1);
          })
        );
      }}
    />
  ));

  const onAddButtonClick = () => {
    setIsUnitCreationModalOpen(true);
  };

  const onItemCreationModalClose = (
    value: { name: string; level: number } | undefined
  ) => {
    setIsUnitCreationModalOpen(false);

    if (value === undefined) return;

    setItems(
      create(items, (draft) => {
        draft.push({
          value,
          count: 1
        });
      })
    );
  };

  return (
    <div className={twMerge("flex flex-col gap-2", className)}>
      <div className="flex flex-wrap gap-2">
        {itemViews}
        <button
          className="w-20 cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
          key="0"
          onClick={onAddButtonClick}
        >
          +
        </button>
      </div>
      <p>Occupied space: {occupiedSpace}</p>

      <UnitCreationModal
        isOpen={isUnitCreationModalOpen}
        types={types}
        onClose={onItemCreationModalClose}
      />
    </div>
  );
};

export default ArmyEditor;
export type { Props };
