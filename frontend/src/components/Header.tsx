import { Link } from "wouter";

const Header: React.FC = () => {
  return (
    <header className="sticky top-0 left-0 z-30 w-full bg-blue-400 p-1 shadow-md">
      <Link
        className="cursor-pointer px-2 py-1 text-lg font-bold text-white"
        href="/"
      >
        cocsim
      </Link>
    </header>
  );
};

export default Header;
