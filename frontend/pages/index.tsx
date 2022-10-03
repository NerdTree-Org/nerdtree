import type { NextPage } from "next";
import styles from "../styles/Index.module.scss";
import Logo from "../public/logo.png";
import Link from "next/link";
import BlogCard from "../components/blog_card";
import Image from "next/image";
import HorizontalScroll from "../components/horizontal_scroll";
import RepoCard from "../components/repo_card";
import Footer from "../components/footer";
import Head from "next/head";
import AuthStatus from "../components/auth_status";

const Home: NextPage = () => {
  return (
    <div>
      <Head>
        <title>NerdTree</title>
      </Head>
      <div
        className={`${styles["toplevel"]} flex flex-col items-center min-w-screen min-h-screen justify-between`}
      >
        <div className={"flex justify-end p-4 w-screen"}>
          <AuthStatus />
        </div>
        <div className={`${styles["greeting-container"]} w-full`}>
          <div>
            <h1>We&apos;re NerdTree</h1>
            <h2>
              An organization aiming to make a better programming community for
              students
            </h2>
          </div>
          <Image
            src={Logo.src}
            height={Logo.height}
            width={Logo.width}
            alt="NerdTree Logo"
          />
        </div>
        <div className={styles["nav-links-container"]}>
          <button id="index">/</button>
          <Link href="/blogs">
            <span className={styles["nav-link"]}>Blogs</span>
          </Link>
          <Link href="/workshops">
            <span className={styles["nav-link"]}>Workshops</span>
          </Link>
          <Link href="/about">
            <span className={styles["nav-link"]}>About us</span>
          </Link>
        </div>
      </div>
      <div className="p-5">
        <h2>Latest Blogs</h2>
        <HorizontalScroll>
          {Array(10)
            .fill(0)
            .map((value, index, array) => (
              <BlogCard
                key={index}
                blog_id={"1234"}
                blog_author={{
                  firstname: "MD Gaziur Rahman",
                  lastname: "Noor",
                  discord_token: "1234",
                  email: "mdgaziurrahmannoor@gmail.com",
                  has_used_discord_token: true,
                  username: "mdgaziur001",
                  is_admin: true,
                  profile_pic: null,
                  id: "1234",
                }}
                blog_image={"https://source.unsplash.com/random/400x300"}
                blog_title={"Blog Test 1"}
                blog_votes={100200}
              />
            ))}
        </HorizontalScroll>
      </div>
      <div className={"p-5"}>
        <h2>Github Projects</h2>
        <HorizontalScroll>
          <RepoCard
            link={"https://github.com/NerdTree-Org/NerdTree"}
            issues={"https://github.com/NerdTree-Org/NerdTree/issues"}
            maintainers={["mdgaziur001"]}
            repo={"NerdTree-Org/NerdTree"}
          />
          <RepoCard
            link={"https://github.com/NerdTree-Org/NerdTree-Wishlist"}
            issues={"https://github.com/NerdTree-Org/NerdTree-Wishlist/issues"}
            maintainers={["mdgaziur001"]}
            repo={"NerdTree-Org/NerdTree-Wishlist"}
          />
        </HorizontalScroll>
      </div>
      <Footer />
    </div>
  );
};

export default Home;
