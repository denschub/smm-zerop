import { useState } from "react";
import Smm2RandomLevelFilter, { getInititalSmm2LevelFilters } from "./smm2_random_level_filter";
import Smm2RandomLevelResult from "./smm2_random_level_result";
import DefaultQueryProvider from "./default_query_provider";

export default function Smm2RandomLevel() {
  const [renderTimestamp, setRenderTimestamp] = useState<number>(Date.now());
  const [activeFilters, setActiveFilters] = useState(getInititalSmm2LevelFilters());
  const [stagingFilters, setStagingFilters] = useState(getInititalSmm2LevelFilters());

  return (
    <DefaultQueryProvider>
      <Smm2RandomLevelResult filter={activeFilters} render_timestamp={renderTimestamp} />
      <button
        className="button section-button"
        onClick={() => {
          setActiveFilters(stagingFilters);
          setRenderTimestamp(Date.now());
        }}
      >
        <i className="fa-solid fa-rotate-right"></i> Load New Level
      </button>
      <Smm2RandomLevelFilter
        onChange={(newFilters) => {
          setStagingFilters(newFilters);
        }}
      />
    </DefaultQueryProvider>
  );
}
