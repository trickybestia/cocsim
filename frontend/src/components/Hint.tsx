import type { ReactNode } from "react";
import { twMerge } from "tailwind-merge";

type Props = {
  className?: string;
  children?: ReactNode;
};

const Hint: React.FC<Props> = ({ className, children }: Props) => {
  return (
    <h3
      className={twMerge(
        "inline-block bg-yellow-300 px-1 text-lg wrap-anywhere whitespace-normal",
        className
      )}
    >
      🔍️ {children}
    </h3>
  );
};

export default Hint;
export type { Props };
