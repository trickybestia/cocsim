import useSWR from "swr";

import { getShowcaseAttack, getShowcaseAttackBaseImage } from "../api";
import GameRenderer from "../components/GameRenderer";
import Header from "../components/Header";

const ShowcasePage: React.FC = () => {
  const { data: frames } = useSWR("get-showcase-attack", getShowcaseAttack, {
    revalidateIfStale: false,
    revalidateOnFocus: false,
    revalidateOnReconnect: false
  });
  const { data: baseImage } = useSWR(
    "get-showcase-attack-base-image",
    getShowcaseAttackBaseImage,
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
        {frames !== undefined && baseImage !== undefined && (
          <div className="flex h-full flex-col items-center">
            <div className="w-full grow lg:max-w-[var(--breakpoint-lg)]">
              <GameRenderer frames={frames} baseImage={baseImage} />
            </div>
          </div>
        )}
      </main>
    </>
  );
};

export default ShowcasePage;
