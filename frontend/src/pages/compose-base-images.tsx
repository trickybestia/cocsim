import { saveAs } from "file-saver";
import { useEffect } from "react";

import ComposeBaseImages from "../components/ComposeBaseImages";
import Header from "../components/Header";

const ComposeBaseImagesPage: React.FC = () => {
  useEffect(() => {
    const onBeforeUnload = (e: BeforeUnloadEvent) => {
      e.preventDefault();
      e.returnValue = "";
    };

    window.addEventListener("beforeunload", onBeforeUnload);

    return () => {
      window.removeEventListener("beforeunload", onBeforeUnload);
    };
  });

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
