import { type ChangeEventHandler, useState } from "react";
import { twJoin } from "tailwind-merge";

type Props = {
  text: string;
  defaultValue: number;
  min: number;
  max: number;
  step: number;
  onChange: (value: number) => void;
};

const FloatNumberInput: React.FC<Props> = ({
  text,
  defaultValue,
  min,
  max,
  step,
  onChange
}: Props) => {
  const [inputValue, setInputValue] = useState(defaultValue.toString());
  const [invalidInput, setInvalidInput] = useState(false);

  const onInputChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    setInputValue(e.target.value);

    try {
      const number = parseFloat(e.target.value);

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
        step={step}
        value={inputValue}
        onChange={onInputChange}
      />
    </>
  );
};

export default FloatNumberInput;
export type { Props };
