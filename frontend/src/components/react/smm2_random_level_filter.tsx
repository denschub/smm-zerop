import { useEffect, useState } from "react";
import FancySelect from "./fancy_select";

type Smm2LevelFilterKey =
  | "year"
  | "style"
  | "theme"
  | "clear_condition_group"
  | "tag"
  | "min_attempts"
  | "max_attempts"
  | "min_clearcheck_ms"
  | "max_clearcheck_ms";

export type Smm2LevelFilters = Partial<Record<Smm2LevelFilterKey, string>>;

function getDefaultSmm2LevelFilters(): Smm2LevelFilters {
  return {
    year: "2022",
    clear_condition_group: "",
  };
}

export function getInititalSmm2LevelFilters(): Smm2LevelFilters {
  const defaults = getDefaultSmm2LevelFilters();

  let filterState: Smm2LevelFilters = {};
  for (const filterKey of Object.keys(availableSmm2Filters) as Smm2LevelFilterKey[]) {
    const stored = window.localStorage.getItem(`filters.smm2.${filterKey}`);
    if (typeof stored == "string" && availableSmm2Filters[filterKey].map((fv) => fv[0]).includes(stored)) {
      filterState[filterKey] = stored;
    } else {
      filterState[filterKey] = defaults[filterKey];
    }
  }

  return filterState;
}

function storeFilterState(filters: Smm2LevelFilters) {
  for (const filterKey of Object.keys(availableSmm2Filters) as Smm2LevelFilterKey[]) {
    if (filters[filterKey] !== undefined) {
      window.localStorage.setItem(`filters.smm2.${filterKey}`, filters[filterKey]);
    } else {
      window.localStorage.removeItem(`filters.smm2.${filterKey}`);
    }
  }
}

interface Smm2RandomLevelFilterProps {
  onChange?: Smm2RandomLevelFilterOnChange;
}

interface Smm2RandomLevelFilterOnChange {
  (newFilters: Smm2LevelFilters): void;
}

const availableSmm2Filters: Record<Smm2LevelFilterKey, string[][]> = {
  year: [
    ["", "Any"],
    ["2022", "2022"],
    ["2023", "2023"],
  ],
  style: [
    ["", "Any"],
    ["smb1", "SMB1"],
    ["smb3", "SMB3"],
    ["smw", "SMW"],
    ["nsmbu", "NSMBU"],
    ["sm3dw", "SM3DW"],
  ],
  theme: [
    ["", "Any"],
    ["airship", "Airship"],
    ["castle", "Castle"],
    ["desert", "Desert"],
    ["forest", "Forest"],
    ["ghost_house", "Ghost House"],
    ["overworld", "Overworld"],
    ["sky", "Sky"],
    ["snow", "Snow"],
    ["underground", "Underground"],
  ],
  clear_condition_group: [
    ["none", "None"],
    ["", "Any"],
    ["no_jumping", "No jumping/landing"],
    ["no_damage", "No taking damage"],
    ["defeating_enemies", "Defeating enemies"],
    ["powerup_finish", "Finish with power-up"],
    ["holding_activating", "Hold or activate items"],
    ["collecting", "Collect items"],
  ],
  tag: [
    ["", "Any"],
    ["art", "Art"],
    ["auto_mario", "Auto Mario"],
    ["autoscroll", "Autoscroll"],
    ["boss_battle", "Boss Battle"],
    ["link", "Link"],
    ["multiplayer_versus", "Multiplayer Versus"],
    ["music", "Music"],
    ["puzzle_solving", "Puzzle Solving"],
    ["shooter", "Shooter"],
    ["short_and_sweet", "Short And Sweet"],
    ["single_player", "Single Player"],
    ["speedrun", "Speedrun"],
    ["standard", "Standard"],
    ["technical", "Technical"],
    ["themed", "Themed"],
  ],
  min_attempts: [
    ["", "All"],
    ["50", "50"],
    ["100", "100"],
    ["200", "200"],
    ["500", "500"],
    ["1000", "1000"],
  ],
  max_attempts: [
    ["", "All"],
    ["50", "50"],
    ["100", "100"],
    ["200", "200"],
    ["500", "500"],
    ["1000", "1000"],
  ],
  min_clearcheck_ms: [
    ["", "All"],
    ["30000", "30 seconds"],
    ["60000", "60 seconds"],
    ["120000", "2 minutes"],
    ["240000", "4 minutes"],
  ],
  max_clearcheck_ms: [
    ["", "All"],
    ["30000", "30 seconds"],
    ["60000", "60 seconds"],
    ["120000", "2 minutes"],
    ["240000", "4 minutes"],
  ],
};

export default function Smm2RandomLevelFilter({ onChange }: Smm2RandomLevelFilterProps) {
  const [selectedFilters, setSelectedFilters] = useState(getInititalSmm2LevelFilters());

  useEffect(() => {
    onChange && onChange(selectedFilters);
    storeFilterState(selectedFilters);
  }, [selectedFilters]);

  return (
    <section className="box">
      <h2>Filters</h2>

      <div className="fancyselect-list">
        <label htmlFor="year" className="caption">
          Year
        </label>
        <FancySelect
          id="year"
          options={availableSmm2Filters.year}
          defaultSelection={selectedFilters.year}
          onChange={(value) => {
            setSelectedFilters((prev) => {
              return {
                ...prev,
                year: value,
              };
            });
          }}
        />

        <label htmlFor="style" className="caption">
          Game style
        </label>
        <FancySelect
          id="style"
          options={availableSmm2Filters.style}
          defaultSelection={selectedFilters.style}
          onChange={(value) => {
            setSelectedFilters((prev) => {
              return {
                ...prev,
                style: value,
              };
            });
          }}
        />

        <label htmlFor="theme" className="caption">
          Level theme
        </label>
        <FancySelect
          id="theme"
          options={availableSmm2Filters.theme}
          defaultSelection={selectedFilters.theme}
          onChange={(value) => {
            setSelectedFilters((prev) => {
              return {
                ...prev,
                theme: value,
              };
            });
          }}
        />

        <label htmlFor="clear_condition_group" className="caption">
          Clear Condition
        </label>
        <FancySelect
          id="clear_condition_group"
          options={availableSmm2Filters.clear_condition_group}
          defaultSelection={selectedFilters.clear_condition_group}
          onChange={(value) => {
            setSelectedFilters((prev) => {
              return {
                ...prev,
                clear_condition_group: value,
              };
            });
          }}
        />

        <label htmlFor="tag" className="caption">
          Tag
        </label>
        <FancySelect
          id="tag"
          options={availableSmm2Filters.tag}
          defaultSelection={selectedFilters.tag}
          onChange={(value) => {
            setSelectedFilters((prev) => {
              return {
                ...prev,
                tag: value,
              };
            });
          }}
        />

        <label htmlFor="min_attempts" className="caption">
          Min. attempts
        </label>
        <FancySelect
          id="min_attempts"
          options={availableSmm2Filters.min_attempts}
          defaultSelection={selectedFilters.min_attempts}
          onChange={(value) => {
            setSelectedFilters((prev) => {
              return {
                ...prev,
                min_attempts: value,
              };
            });
          }}
        />

        <label htmlFor="max_attempts" className="caption">
          Max. attempts
        </label>
        <FancySelect
          id="max_attempts"
          options={availableSmm2Filters.max_attempts}
          defaultSelection={selectedFilters.max_attempts}
          onChange={(value) => {
            setSelectedFilters((prev) => {
              return {
                ...prev,
                max_attempts: value,
              };
            });
          }}
        />

        <label htmlFor="min_clearcheck_ms" className="caption">
          Min. clear check time
        </label>
        <FancySelect
          id="min_clearcheck_ms"
          options={availableSmm2Filters.min_clearcheck_ms}
          defaultSelection={selectedFilters.min_clearcheck_ms}
          onChange={(value) => {
            setSelectedFilters((prev) => {
              return {
                ...prev,
                min_clearcheck_ms: value,
              };
            });
          }}
        />

        <label htmlFor="max_clearcheck_ms" className="caption">
          Max. clear check time
        </label>
        <FancySelect
          id="max_clearcheck_ms"
          options={availableSmm2Filters.max_clearcheck_ms}
          defaultSelection={selectedFilters.max_clearcheck_ms}
          onChange={(value) => {
            setSelectedFilters((prev) => {
              return {
                ...prev,
                max_clearcheck_ms: value,
              };
            });
          }}
        />
      </div>

      <div className="level-actions" style={{ marginTop: "2rem" }}>
        <button
          className="button"
          onClick={() => {
            setSelectedFilters(() => {
              return getDefaultSmm2LevelFilters();
            });
            window.location.reload();
          }}
        >
          <i className="fa-solid fa-trash"></i> Reset Filters
        </button>
      </div>
    </section>
  );
}
