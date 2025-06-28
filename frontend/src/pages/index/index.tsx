import PageLink from "../../components/PageLink";

const IndexPage: React.FC = () => {
  return (
    <main className="absolute top-0 left-0 h-full w-full">
      <div className="relative top-[40%] left-1/2 flex -translate-1/2 flex-col items-center gap-2 text-center">
        <h1 className="text-3xl font-semibold">cocsim</h1>
        <p>The following tools are available:</p>
        <PageLink href="compose-base-images">Compose base images</PageLink>
        <PageLink href="map-editor">Map editor</PageLink>
      </div>
    </main>
  );
};

export default IndexPage;
