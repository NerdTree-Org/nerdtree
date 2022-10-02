import { ReactNode } from "react";
import styles from "../styles/nerdtree_button.module.scss";

interface Props {
  type: ButtonType;
  children: ReactNode;
}

export enum ButtonType {
  Hazy = "hazy-button",
}

const NerdTreeButton = (props: Props) => {
  return (
    <span className={`${styles["button"]} ${styles[props.type]}`}>
      {props.children}
    </span>
  );
};

export default NerdTreeButton;
