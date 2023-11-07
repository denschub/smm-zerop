import { useState } from "react";
import Smm1RandomLevelFilter, { getDefaultSmm1LevelFilters } from "./smm1_random_level_filter";
import Smm1RandomLevelResult from "./smm1_random_level_result";
import DefaultQueryProvider from "./default_query_provider";

export default function Smm1RandomLevel() {
  const [renderTimestamp, setRenderTimestamp] = useState<number>(Date.now());
  const [activeFilters, setActiveFilters] = useState(getDefaultSmm1LevelFilters());
  const [stagingFilters, setStagingFilters] = useState(getDefaultSmm1LevelFilters());

  return (
    <DefaultQueryProvider>
      <Smm1RandomLevelResult filter={activeFilters} render_timestamp={renderTimestamp} />
      <button
        className="button section-button"
        onClick={() => {
          setActiveFilters(stagingFilters);
          setRenderTimestamp(Date.now());
        }}
      >
        <i className="fa-solid fa-rotate-right"></i> Load New Level
      </button>
      <Smm1RandomLevelFilter
        onChange={(newFilters) => {
          setStagingFilters(newFilters);
        }}
      />
    </DefaultQueryProvider>
  );
}
