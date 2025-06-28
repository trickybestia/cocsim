import { saveAs } from "file-saver";

import ComposeBaseImages from "../components/ComposeBaseImages";
import Header from "../components/Header";

const ComposeBaseImagesPage: React.FC = () => {
  return (
    <>
      <Header />
      <main className="p-4">
        <ComposeBaseImages
          onDone={(image) =>
            saveAs(image, `cocsim-composed-${new Date().toISOString()}`)
          }
        />
      </main>
    </>
  );
};

export default ComposeBaseImagesPage;
