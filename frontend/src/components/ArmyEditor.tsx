import type { Unit } from "../types";

type Props = {
  onOk: (units: Unit[]) => void;
};

const ArmyEditor: React.FC<Props> = ({ onOk }: Props) => {
  return <div></div>;
};

export default ArmyEditor;
export type { Props };
