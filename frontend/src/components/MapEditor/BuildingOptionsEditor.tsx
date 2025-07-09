import { create } from "mutative";
import { Fragment } from "react";

import useBuildingTypes from "../../hooks/use-building-types";
import type { Building } from "../../types";
import IntegerNumberInput from "./IntegerNumberInput";

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
      <IntegerNumberInput
        text="Level:"
        min={1}
        max={buildingType.levels}
        defaultValue={building.level + 1}
        onChange={(value) =>
          onChange(
            create(building, (draft) => {
              draft.level = value - 1;
            })
          )
        }
      />
      {buildingType.options.map(({ name, values }) => (
        <Fragment key={name}>
          <p>{name}:</p>
          <select
            value={building[name]}
            onChange={(e) =>
              onChange(
                create(building, (draft) => {
                  draft[name] = e.target.value;
                })
              )
            }
          >
            {values.map((value) => (
              <option key={value} value={value}>
                {value}
              </option>
            ))}
          </select>
        </Fragment>
      ))}
    </>
  );
};

export default BuildingOptionsEditor;
export type { Props };
