import { saveAs } from "file-saver";

import ComposeBaseImages from "../components/ComposeBaseImages";
import Header from "../components/Header";

const ComposeBaseImagesPage: React.FC = () => {
  return (
    <>
      <Header />
      <main className="flex flex-col items-center p-4">
        <ComposeBaseImages
          className="w-full grow lg:max-w-[var(--breakpoint-lg)]"
          onDone={(image) =>
            saveAs(image, `cocsim-composed-${new Date().toISOString()}`)
          }
        />
      </main>
    </>
  );
};

export default ComposeBaseImagesPage;
