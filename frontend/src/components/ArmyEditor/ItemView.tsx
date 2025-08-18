import type { WithCount, WithLevel, WithName } from "../../types";
import IntegerNumberInput from "../IntegerNumberInput";

type Props = {
  value: {
    value: WithName & WithLevel;
  } & WithCount;
  onRemove: () => void;
  onCountChange: (value: number) => void;
};

const ItemView: React.FC<Props> = ({
  value,
  onRemove,
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
      <p>{value.value.name}</p>
      <p>Lvl. {value.value.level + 1}</p>
      <IntegerNumberInput
        text="Count:"
        min={1}
        max={1000}
        defaultValue={value.count}
        onChange={onCountChange}
      />
    </div>
  );
};

export default ItemView;
export type { Props };
