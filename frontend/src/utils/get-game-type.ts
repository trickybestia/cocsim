import type { WithName } from "../types";

const getGameType = <T extends WithName>(types: T[], name: string): T => {
  const result = types.find((type) => type.name === name);

  if (result === undefined)
    throw new Error(`Type with name === "${name}" not found`);

  return result;
};

export default getGameType;
