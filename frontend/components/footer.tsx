import styles from "../styles/footer.module.scss";
import Link from "next/link";

const Footer = () => {
  return (
    <div className={styles["footer"]}>
      <h1>NerdTree</h1>
      <div>
        <Link href={"https://github.com/NerdTree-Org/NerdTree"}>Github</Link>
        <Link href={"https://www.facebook.com/nerdtree"}>Facebook</Link>
        <Link href={"https://discord.gg/EKsmr9UBfD"}>Discord</Link>
      </div>
    </div>
  );
};

export default Footer;
