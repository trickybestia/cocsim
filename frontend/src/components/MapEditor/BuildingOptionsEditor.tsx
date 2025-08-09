import { create } from "mutative";
import { Fragment } from "react";

import useBuildingTypes from "../../hooks/use-building-types";
import type { Building, UnitWithCount } from "../../types";
import ArmyEditor from "../ArmyEditor";
import IntegerNumberInput from "../IntegerNumberInput";

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
    <fieldset className="grid-col grid grid-cols-[auto_min-content] gap-1 border-2 pr-1 pb-1 pl-1 text-end text-nowrap">
      <legend className="px-1">{building.name}</legend>
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
      {building.name === "ClanCastle" && (
        <>
          <p className="col-span-2 text-start">Troops inside:</p>
          <ArmyEditor
            className="col-span-2"
            units={building.units as UnitWithCount[]}
            setUnits={(units) =>
              onChange(
                create(building, (draft) => {
                  draft.units = units;
                })
              )
            }
          />
        </>
      )}
    </fieldset>
  );
};

export default BuildingOptionsEditor;
export type { Props };
