import { useEffect, useState } from "react";
import FancySelect from "./fancy_select";

export type Smm2LevelFilters = {
  year: string;
  style?: string;
  min_attempts?: string;
  max_attempts?: string;
  min_clearcheck_ms?: string;
  max_clearcheck_ms?: string;
};

export function getDefaultSmm2LevelFilters(): Smm2LevelFilters {
  return {
    year: "2020",
  };
}

interface Smm2RandomLevelFilterProps {
  onChange?: Smm2RandomLevelFilterOnChange;
}

interface Smm2RandomLevelFilterOnChange {
  (newFilters: Smm2LevelFilters): void;
}

const availableYears = [
  ["2020", "2020"],
  ["2021", "2021"],
  ["2022", "2022"],
];

const availableStyles = [
  ["", "Any"],
  ["smb1", "SMB1"],
  ["smb3", "SMB3"],
  ["smw", "SMW"],
  ["nsmbu", "NSMBU"],
  ["sm3dw", "SM3DW"],
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
