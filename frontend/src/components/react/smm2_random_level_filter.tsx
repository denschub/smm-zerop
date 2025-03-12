import { useEffect, useState } from "react";
import FancySelect from "./fancy_select";

export type Smm2LevelFilters = {
  year?: string;
  style?: string;
  min_attempts?: string;
  max_attempts?: string;
  min_clearcheck_ms?: string;
  max_clearcheck_ms?: string;
};

export function getDefaultSmm2LevelFilters(): Smm2LevelFilters {
  return {
    year: "2022",
  };
}

interface Smm2RandomLevelFilterProps {
  onChange?: Smm2RandomLevelFilterOnChange;
}

interface Smm2RandomLevelFilterOnChange {
  (newFilters: Smm2LevelFilters): void;
}

const availableYears = [
  ["", "Any"],
  ["2021", "2021"],
  ["2022", "2022"],
  ["2023", "2023"],
];

const availableStyles = [
  ["", "Any"],
  ["smb1", "SMB1"],
  ["smb3", "SMB3"],
  ["smw", "SMW"],
  ["nsmbu", "NSMBU"],
  ["sm3dw", "SM3DW"],
];

const availableThemes = [
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
];

const availableCCGroups = [
  ["none", "None"],
  ["", "Any"],
  ["no_jumping", "No jumping/landing"],
  ["no_damage", "No taking damage"],
  ["defeating_enemies", "Defeating enemies"],
  ["powerup_finish", "Finish with power-up"],
  ["holding_activating", "Hold or activate items"],
  ["collecting", "Collect items"],
];

const availableTags = [
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
];

const attemptOptions = [
  ["", "All"],
  ["50", "50"],
  ["100", "100"],
  ["200", "200"],
  ["500", "500"],
  ["1000", "1000"],
];

const clearcheckOptions = [
  ["", "All"],
  ["30000", "30 seconds"],
  ["60000", "60 seconds"],
  ["120000", "2 minutes"],
  ["240000", "4 minutes"],
];

export default function Smm2RandomLevelFilter({ onChange }: Smm2RandomLevelFilterProps) {
  const [selectedFilters, setSelectedFilters] = useState(getDefaultSmm2LevelFilters());

  useEffect(() => {
    onChange && onChange(selectedFilters);
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
          options={availableYears}
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
          options={availableStyles}
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
          options={availableThemes}
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
          options={availableCCGroups}
          defaultSelection=""
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
          options={availableTags}
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
          options={attemptOptions}
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
          options={attemptOptions}
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
          options={clearcheckOptions}
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
          options={clearcheckOptions}
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
    </section>
  );
}
