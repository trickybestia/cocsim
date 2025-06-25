type Props = React.HTMLAttributes<HTMLDivElement> & {
  src: string;
  sideText: string;

  onClose?: () => void;
  onUp?: () => void;
  onDown?: () => void;
  onSide?: () => void;
};

const GridImage: React.FC<Props> = (props: Props) => {
  return (
    <div {...props}>
      <img src={props.src} />
    </div>
  );
};

export default GridImage;
export type { Props };
