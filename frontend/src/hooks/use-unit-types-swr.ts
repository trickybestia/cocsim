import useSWR from "swr";

import { getUnitTypes } from "../api";
import type { UnitType } from "../types";

const useUnitTypesSWR = (): UnitType[] | undefined => {
  const { data } = useSWR("get-unit-types", getUnitTypes, {
    revalidateIfStale: false,
    revalidateOnFocus: false,
    revalidateOnReconnect: false
  });

  return data;
};

export default useUnitTypesSWR;
