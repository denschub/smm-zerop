import { useEffect, useState } from "react";

interface FancySelectProps {
  id: string;
  options: string[][];
  onChange?: FancySelectOnChange;
}

interface FancySelectOnChange {
  (value: string): void;
}

export default function FancySelect({ id, options, onChange }: FancySelectProps) {
  const [selectedIndex, setSelectedIndex] = useState(0);

  // we need the value to pass back up, and also to update the currently selected value for the select field. So this
  // state is only ever updated by the useEffect hook below that updates whenever the index is run. This isn't the best
  // impl for performance, but this is good enough for this use-case.
  const [selectedValue, setSelectedValue] = useState(options[0][0]);
  useEffect(() => {
    const newValue = options[selectedIndex][0];
    setSelectedValue(newValue);
    onChange && onChange(newValue);
  }, [selectedIndex]);

  return (
    <div className="fancy-select">
      <button
        aria-label="previous"
        className={selectedIndex < 1 ? "disabled" : ""}
        onClick={() => {
          setSelectedIndex((prevIndex) => (prevIndex < 1 ? prevIndex : prevIndex - 1));
        }}
      >
        <i className="fa-solid fa-triangle fa-rotate-270"></i>
      </button>
      <select
        id={id}
        value={selectedValue}
        onChange={(ev) => {
          const targetIndex = options.findIndex((opt) => opt[0] == ev.target.value);
          setSelectedIndex(targetIndex);
        }}
      >
        {options.map((opt) => (
          <option value={opt[0]} key={opt[0]}>
            {opt[1]}
          </option>
        ))}
      </select>
      <button
        aria-label="next"
        className={selectedIndex >= options.length - 1 ? "disabled" : ""}
        onClick={() => {
          setSelectedIndex((prevIndex) => (prevIndex >= options.length - 1 ? prevIndex : prevIndex + 1));
        }}
      >
        <i className="fa-solid fa-triangle fa-rotate-90"></i>
      </button>
    </div>
  );
}
