import type { ReactNode } from "react";
import { twMerge } from "tailwind-merge";
import { Link } from "wouter";

type Props = React.HTMLAttributes<HTMLDivElement> & {
  href: string;
  children: ReactNode;
};

const PageLink: React.FC<Props> = ({
  className,

  href,
  children,

  ...props
}: Props) => {
  return (
    <div className={twMerge(className, "flex items-baseline gap-2")} {...props}>
      <p className="inline">{">>>"}</p>
      <Link
        className="inline cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
        href={href}
      >
        {children}
      </Link>
      <p className="inline">{"<<<"}</p>
    </div>
  );
};

export default PageLink;
export type { Props };
