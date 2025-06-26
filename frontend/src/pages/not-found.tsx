import { Link } from "wouter";

const NotFound: React.FC = () => {
  return (
    <div className="fixed top-0 left-0 h-full w-full">
      <div className="relative top-[40%] left-1/2 flex -translate-1/2 flex-col items-center gap-2 text-center">
        <h1 className="text-3xl font-semibold">404 - Not Found</h1>
        <p>The page you are looking for does not exist.</p>
        <div className="flex items-baseline gap-2">
          <p className="inline">{">>>"}</p>
          <Link
            className="inline cursor-pointer bg-blue-400 px-2 py-1 text-base font-bold text-white hover:bg-blue-600"
            to="/"
            replace
          >
            Go to /
          </Link>
          <p className="inline">{"<<<"}</p>
        </div>
      </div>
    </div>
  );
};

export default NotFound;
