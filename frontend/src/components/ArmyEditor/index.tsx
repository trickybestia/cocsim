import { create } from "mutative";
import { useState } from "react";
import { twMerge } from "tailwind-merge";

import useUnitTypes from "../../hooks/use-unit-types";
import type { UnitType, UnitWithCount } from "../../types";
import UnitCreationModal from "./UnitCreationModal";
import UnitView from "./UnitView";

type Props = {
  className?: string;
  units: UnitWithCount[];
  setUnits: (units: UnitWithCount[]) => void;
};

const ArmyEditor: React.FC<Props> = ({ className, units, setUnits }: Props) => {
  const { getUnitType } = useUnitTypes();

  const [isUnitCreationModalOpen, setIsUnitCreationModalOpen] = useState(false);

  let occupiedSpace = 0;

  units.forEach((unit) => {
    occupiedSpace += getUnitType(unit.unit).housingSpace * unit.count;
  });

  const unitViews = units.map((unit, index) => (
    <UnitView
      key={index}
      unit={unit.unit}
      defaultCount={unit.count}
      onCountChange={(newCount) =>
        setUnits(
          create(units, (draft) => {
            draft[index].count = newCount;
          })
        )
      }
      onRemove={() => {
        setUnits(
          create(units, (draft) => {
            draft.splice(index, 1);
          })
        );
      }}
    />
  ));

  const onAddUnitButtonClick = () => {
    setIsUnitCreationModalOpen(true);
  };

  const onUnitCreationModalClose = (
    unit: { unitType: UnitType; level: number } | undefined
  ) => {
    setIsUnitCreationModalOpen(false);

    if (unit === undefined) return;

    setUnits(
      create(units, (draft) => {
        draft.push({
          unit: { name: unit.unitType.name, level: unit.level },
          count: 1
        });
      })
    );
  };

  return (
    <div className={twMerge("flex flex-col gap-2", className)}>
      <div className="flex flex-wrap gap-2">
        {unitViews}
        <button
          className="w-20 cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
          key="0"
          onClick={onAddUnitButtonClick}
        >
          +
        </button>
      </div>
      <p>Occupied space: {occupiedSpace}</p>

      <UnitCreationModal
        isOpen={isUnitCreationModalOpen}
        onClose={onUnitCreationModalClose}
      />
    </div>
  );
};

export default ArmyEditor;
export type { Props };
