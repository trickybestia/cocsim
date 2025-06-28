import PageLink from "../components/PageLink";

const NotFoundPage: React.FC = () => {
  return (
    <main className="absolute top-0 left-0 h-full w-full">
      <div className="relative top-[40%] left-1/2 flex -translate-1/2 flex-col items-center gap-2 text-center">
        <h1 className="text-3xl font-semibold">404 - Not Found</h1>
        <p>The page you are looking for does not exist.</p>
        <PageLink href="/">Go to /</PageLink>
      </div>
    </main>
  );
};

export default NotFoundPage;
