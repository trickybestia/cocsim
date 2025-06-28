import ComposeBaseImages from "../components/ComposeBaseImages";
import Header from "../components/Header";

const ComposeBaseImagesPage: React.FC = () => {
  return (
    <>
      <Header />
      <main className="p-4">
        <ComposeBaseImages onComposed={console.log} />
      </main>
    </>
  );
};

export default ComposeBaseImagesPage;
