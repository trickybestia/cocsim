import useSWR from "swr";

import { getBuildingTypes } from "../api";
import type { BuildingType } from "../types";

const useBuildingTypesSWR = (): BuildingType[] | undefined => {
  const { data } = useSWR("get-building-types", getBuildingTypes, {
    revalidateIfStale: false,
    revalidateOnFocus: false,
    revalidateOnReconnect: false
  });

  return data;
};

export default useBuildingTypesSWR;
