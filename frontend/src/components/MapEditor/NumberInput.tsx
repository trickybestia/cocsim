import { type ChangeEventHandler, useState } from "react";
import { twJoin } from "tailwind-merge";

type Props = {
  text: string;
  defaultValue: number;
  min: number;
  max: number;
  onChange: (value: number) => void;
};

const NumberInput: React.FC<Props> = ({
  text,
  defaultValue: value,
  min,
  max,
  onChange
}: Props) => {
  const [inputValue, setInputValue] = useState(value.toString());
  const [invalidInput, setInvalidInput] = useState(false);

  const onInputChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    setInputValue(e.target.value);

    try {
      const number = parseInt(e.target.value);

      if (number >= min && number <= max) {
        onChange(number);
        setInvalidInput(false);

        return;
      }
    } catch {
      // input value is not a number
    }

    setInvalidInput(true);
  };

  return (
    <>
      <p>{text}</p>
      <input
        className={twJoin(invalidInput && "bg-red-500")}
        type="number"
        min={min}
        max={max}
        value={inputValue}
        onChange={onInputChange}
      />
    </>
  );
};

export default NumberInput;
export type { Props };
