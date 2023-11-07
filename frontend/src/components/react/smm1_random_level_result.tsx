import { getDefaultSmm1LevelFilters, type Smm1LevelFilters } from "./smm1_random_level_filter";
import { useQuery } from "@tanstack/react-query";
import LoadingSpinner from "./loading_spinner";
import Smm1Level from "./smm1_level";

interface Smm1RandomLevelResultProps {
  render_timestamp: number;
  filter: Smm1LevelFilters;
}

function removeEmptyFilterValues(raw: Smm1LevelFilters): Smm1LevelFilters {
  return Object.assign(
    {},
    getDefaultSmm1LevelFilters(),
    Object.fromEntries(Object.entries(raw).filter(([_key, value]) => value != "")),
  );
}

export default function Smm1RandomLevelResult({ render_timestamp, filter }: Smm1RandomLevelResultProps) {
  const cleanedFilters = removeEmptyFilterValues(filter);

  const { isLoading, error, data } = useQuery({
    queryKey: ["/smm1/random_level", cleanedFilters, render_timestamp],
    queryFn: async () => {
      const filterParams = new URLSearchParams(cleanedFilters);
      const res = await fetch(
        [`${import.meta.env.PUBLIC_SMM_ZEROP_API_ROOT}/smm1/random_level`, filterParams.toString()].join("?"),
      );

      if (res.ok) {
        return {
          status: "ok",
          responseCode: res.status,
          data: await res.json(),
        };
      }

      return Promise.reject({
        status: "err",
        responseCode: res.status,
        data: await res.text(),
      });
    },
  });

  return (
    <>
      {isLoading && <LoadingSpinner />}
      {error && (
        <section className="box">
          <h2>Oh no!</h2>
          {(error as any).responseCode == 404 ? (
            <p>
              <strong>No level found</strong>! Be sure to double-check your filters.
            </p>
          ) : (
            <p>
              <strong>Something went wrong</strong>: {error.message}
            </p>
          )}
        </section>
      )}
      {data && <Smm1Level level={data.data} />}
    </>
  );
}
