import { useState } from "react";

import ArmyEditor from "../components/ArmyEditor";
import GameRenderer from "../components/GameRenderer";
import Header from "../components/Header";
import { UnitTypesContext } from "../hooks/use-unit-types";
import useUnitTypesSWR from "../hooks/use-unit-types-swr";
import type { Unit } from "../types";

const AttackOptimizerPage: React.FC = () => {
  const unitTypes = useUnitTypesSWR();
  const [units, setUnits] = useState<Unit[] | undefined>(undefined);

  return (
    <>
      <Header />
      <main className="grow p-4">
        <div className="flex h-full flex-col items-center">
          <div className="w-full grow lg:max-w-[var(--breakpoint-lg)]">
            {units === undefined ? (
              unitTypes !== undefined && (
                <UnitTypesContext value={unitTypes}>
                  <ArmyEditor onOk={setUnits} />
                </UnitTypesContext>
              )
            ) : (
              <GameRenderer frames={[]} baseImage={undefined} />
            )}
          </div>
        </div>
      </main>
    </>
  );
};

export default AttackOptimizerPage;
