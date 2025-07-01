import { useState } from "react";
import { twJoin } from "tailwind-merge";

import fuzzySearch from "../../../utils/fuzzy-search";

type Props = {
  values: string[];
  onSelected: (value: string) => void;
};

const StringSelector: React.FC<Props> = ({ values, onSelected }: Props) => {
  const [selectedValue, setSelectedValue] = useState<string>(values[0]);
  const [inputValue, setInputValue] = useState("");

  const displayedValues = fuzzySearch(values, inputValue);

  const onOkButtonClick = () => {
    if (selectedValue !== undefined) {
      onSelected(selectedValue);
    }
  };

  const onInputValueChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setInputValue(e.target.value);
    setSelectedValue(fuzzySearch(values, e.target.value)[0]);
  };

  const onInputKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      onSelected(selectedValue);
    } else if (e.key === "ArrowUp" || e.key === "ArrowDown") {
      e.preventDefault();

      const selectedValueIndex = displayedValues.indexOf(selectedValue);

      if (e.key === "ArrowUp") {
        if (selectedValueIndex > 0) {
          setSelectedValue(displayedValues[selectedValueIndex - 1]);
        }
      } else {
        if (selectedValueIndex < displayedValues.length - 1) {
          setSelectedValue(displayedValues[selectedValueIndex + 1]);
        }
      }
    }
  };

  return (
    <div className="flex flex-col gap-2">
      <div className="flex gap-2">
        <input
          className="border border-gray-300 px-2"
          type="text"
          placeholder="search for a building"
          value={inputValue}
          onChange={onInputValueChange}
          onKeyDown={onInputKeyDown}
          autoFocus
        />
        <button
          onClick={onOkButtonClick}
          className="cursor-pointer bg-blue-400 px-2 py-1 text-sm font-bold text-white hover:bg-blue-600"
        >
          OK
        </button>
      </div>
      <div className="flex h-100 flex-col overflow-y-scroll">
        {displayedValues.map((value, index) => {
          let backgroundColorStyle;

          if (value === selectedValue) {
            backgroundColorStyle = "bg-blue-400";
          } else {
            backgroundColorStyle =
              index % 2 == 1 ? "bg-gray-200" : "bg-gray-100";
          }

          return (
            <p
              key={value}
              className={twJoin(
                backgroundColorStyle,
                "cursor-pointer px-2 py-1 select-none"
              )}
              onClick={() => setSelectedValue(value)}
              onDoubleClick={() => onSelected(value)}
            >
              {value}
            </p>
          );
        })}
      </div>
    </div>
  );
};

export default StringSelector;
export type { Props };
