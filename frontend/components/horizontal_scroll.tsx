import { ReactNode } from "react";
import styles from "../styles/horizontal_scroll.module.scss";

interface Props {
  children: ReactNode;
}

const HorizontalScroll = (props: Props) => {
  return (
    <div className={`${styles["horizontal-scroll"]} overflow-x-scroll`}>
      {props.children}
    </div>
  );
};

export default HorizontalScroll;
