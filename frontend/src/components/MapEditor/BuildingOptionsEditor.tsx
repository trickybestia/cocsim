import { create } from "mutative";

import useBuildingTypes from "../../hooks/use-building-types";
import type { Building } from "../../types";
import NumberInput from "./NumberInput";

type Props = {
  building: Building;
  onChange: (newBuilding: Building) => void;
};

const BuildingOptionsEditor: React.FC<Props> = ({
  building,
  onChange
}: Props) => {
  const buildingType = useBuildingTypes().getBuildingType(building);

  return (
    <>
      <p>Selected:</p>
      <p>{building.name}</p>
      <NumberInput
        text="Level:"
        min={1}
        max={buildingType!.levels}
        defaultValue={building.level + 1}
        onChange={(value) =>
          onChange(
            create(building, (draft) => {
              draft.level = value - 1;
            })
          )
        }
      />
    </>
  );
};

export default BuildingOptionsEditor;
export type { Props };
