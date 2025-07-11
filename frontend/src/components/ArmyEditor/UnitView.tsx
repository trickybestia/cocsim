import type { Unit } from "../../types";
import IntegerNumberInput from "../IntegerNumberInput";

type Props = {
  unit: Unit;
  onRemove: () => void;
  defaultCount: number;
  onCountChange: (value: number) => void;
};

const UnitView: React.FC<Props> = ({
  unit,
  onRemove,
  defaultCount,
  onCountChange
}: Props) => {
  return (
    <div className="inline-flex flex-col gap-1 bg-gray-200 p-2 text-center">
      <div className="flex justify-end">
        <button
          className="size-5 cursor-pointer bg-red-500 font-mono text-sm font-bold text-white hover:bg-red-700"
          onClick={onRemove}
        >
          X
        </button>
      </div>
      <p>{unit.name}</p>
      <p>Lvl. {unit.level + 1}</p>
      <IntegerNumberInput
        text="Count:"
        min={1}
        max={1000}
        defaultValue={defaultCount}
        onChange={onCountChange}
      />
    </div>
  );
};

export default UnitView;
export type { Props };
