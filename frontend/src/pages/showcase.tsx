import useSWR from "swr";

import api from "../api";
import GameRenderer from "../components/GameRenderer";
import Header from "../components/Header";

const ShowcasePage: React.FC = () => {
  const { data: frames } = useSWR(
    "get-showcase-attack",
    api.getShowcaseAttack,
    {
      revalidateIfStale: false,
      revalidateOnFocus: false,
      revalidateOnReconnect: false
    }
  );
  const { data: baseImage } = useSWR(
    "get-showcase-attack-base-image",
    api.getShowcaseAttackBaseImage,
    {
      revalidateIfStale: false,
      revalidateOnFocus: false,
      revalidateOnReconnect: false
    }
  );

  return (
    <>
      <Header />
      <main className="grow p-4">
        <div className="flex h-full flex-col items-center">
          <div className="w-full grow lg:max-w-[var(--breakpoint-lg)]">
            {frames === undefined || baseImage === undefined ? (
              <p>Loading...</p>
            ) : (
              <GameRenderer frames={frames} baseImage={baseImage} />
            )}
          </div>
        </div>
      </main>
    </>
  );
};

export default ShowcasePage;
