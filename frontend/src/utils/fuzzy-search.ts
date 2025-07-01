import Fuse from "fuse.js";

const fuzzySearch = (values: string[], pattern: string): string[] => {
  let result = new Fuse(values)
    .search(pattern)
    .map((searchResult) => searchResult.item);

  if (result.length === 0) result = values;

  return result;
};

export default fuzzySearch;
