import { User } from "../interfaces/user";
import styles from "../styles/blog_card.module.scss";
import Link from "next/link";

interface Props {
  blog_id: string;
  blog_title: string;
  blog_image: string;
  blog_votes: number;
  blog_author: User | null;
}

function round_number(n: number): string {
  let is_negative = n < 0;
  let abs_n = Math.abs(n);
  let result = is_negative ? "-" : "";

  if (abs_n / 1_000_000 >= 1) {
    result += `${(abs_n / 1_000_000).toFixed(2)}m`;
  } else if (abs_n / 1000 >= 1) {
    result += `${(abs_n / 1000).toFixed(2)}k`;
  } else {
    result += abs_n;
  }

  return result;
}

const BlogCard = (props: Props) => {
  return (
    <div className={styles["blog-card"]}>
      <Link
        href={`/u/${
          props.blog_author ? props.blog_author.username : "deleted_user"
        }`}
      >
        <h3>
          u/{props.blog_author ? props.blog_author.username : "deleted_user"}
        </h3>
      </Link>
      <Link href={`/blogs/${props.blog_id}`}>
        <h1>{props.blog_title}</h1>
      </Link>
      <h4>{`${round_number(props.blog_votes)} upvotes`}</h4>
      <img src={props.blog_image} alt={"Blog Thumbnail"} />
    </div>
  );
};

export default BlogCard;
