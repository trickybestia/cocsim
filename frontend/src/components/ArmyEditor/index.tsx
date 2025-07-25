import { create } from "mutative";
import { useState } from "react";

import useUnitTypes from "../../hooks/use-unit-types";
import type { UnitType, UnitWithCount } from "../../types";
import UnitCreationModal from "./UnitCreationModal";
import UnitView from "./UnitView";

type Props = {
  onOk: (units: UnitWithCount[]) => void;
};

const ArmyEditor: React.FC<Props> = ({ onOk }: Props) => {
  const { getUnitType } = useUnitTypes();
  const [units, setUnits] = useState<UnitWithCount[]>([]);

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

  const onOkButtonClick = () => {
    onOk(units);
  };

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
    <div className="flex flex-col gap-2">
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
      <button
        onClick={onOkButtonClick}
        className="w-min cursor-pointer bg-blue-400 px-2 py-1 text-sm font-bold text-white hover:bg-blue-600"
      >
        OK
      </button>

      <UnitCreationModal
        isOpen={isUnitCreationModalOpen}
        onClose={onUnitCreationModalClose}
      />
    </div>
  );
};

export default ArmyEditor;
export type { Props };
