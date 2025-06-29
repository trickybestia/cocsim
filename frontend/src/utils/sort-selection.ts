const sortSelection = (
  pos1: { x: number; y: number } | undefined,
  pos2: { x: number; y: number } | undefined
):
  | { leftTop: { x: number; y: number }; rightBottom: { x: number; y: number } }
  | undefined => {
  if (pos1 === undefined || pos2 == undefined) return;

  return {
    leftTop: { x: Math.min(pos1.x, pos2.x), y: Math.min(pos1.y, pos2.y) },
    rightBottom: { x: Math.max(pos1.x, pos2.x), y: Math.max(pos1.y, pos2.y) }
  };
};

export default sortSelection;
