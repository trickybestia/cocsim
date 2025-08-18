import useSWR from "swr";

import { getSpellTypes } from "../api";
import type { SpellType } from "../types";

const useSpellTypesSWR = (): SpellType[] | undefined => {
  const { data } = useSWR("get-spell-types", getSpellTypes, {
    revalidateIfStale: false,
    revalidateOnFocus: false,
    revalidateOnReconnect: false
  });

  return data;
};

export default useSpellTypesSWR;
