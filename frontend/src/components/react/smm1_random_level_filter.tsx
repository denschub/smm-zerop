import { useEffect, useState } from "react";
import FancySelect from "./fancy_select";

export type Smm1LevelFilters = {
  year: string;
  min_attempts?: string;
  max_attempts?: string;
};

export function getDefaultSmm1LevelFilters(): Smm1LevelFilters {
  return {
    year: "2017",
  };
}

interface Smm1RandomLevelFilterProps {
  onChange?: Smm1RandomLevelFilterOnChange;
}

interface Smm1RandomLevelFilterOnChange {
  (newFilters: Smm1LevelFilters): void;
}

const availableYears = [["2017", "2017"]];

const attemptOptions = [
  ["", "All"],
  ["50", "50"],
  ["100", "100"],
  ["200", "200"],
  ["500", "500"],
  ["1000", "1000"],
];

export default function Smm1RandomLevelFilter({ onChange }: Smm1RandomLevelFilterProps) {
  const [selectedFilters, setSelectedFilters] = useState(getDefaultSmm1LevelFilters());

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
          onChange={(value) => {
            setSelectedFilters((prev) => {
              return {
                ...prev,
                year: value,
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
      </div>
    </section>
  );
}
