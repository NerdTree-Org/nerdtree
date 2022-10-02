import styles from "../styles/repo_card.module.scss";
import NerdTreeButton, { ButtonType } from "./nerdtree_button";
import Link from "next/link";

interface Props {
  repo: string;
  link: string;
  issues: string;
  maintainers: string[];
}

const RepoCard = (props: Props) => {
  return (
    <div className={styles["repo-card"]}>
      {/*
        TODO: find a better way to grab repo social media preview
      */}
      <img
        src={
          "https://opengraph.githubassets.com/e99ccd45d06538a422ca2012bb9e73f14e9a07ddeb56cc70f673f4fd8129e849/" +
          props.repo
        }
        alt="repo-preview"
      />
      <div>
        <h3>{props.repo}</h3>
        <h4>Maintainers: {props.maintainers.join(", ")}</h4>
        <div className={"flex gap-3.5"}>
          <Link href={props.link}>
            <a>
              <NerdTreeButton type={ButtonType.Hazy}>Repo</NerdTreeButton>
            </a>
          </Link>
          <Link href={props.issues}>
            <a>
              <NerdTreeButton type={ButtonType.Hazy}>Issues</NerdTreeButton>
            </a>
          </Link>
        </div>
      </div>
    </div>
  );
};

export default RepoCard;
