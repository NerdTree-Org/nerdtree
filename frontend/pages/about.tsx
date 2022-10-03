import { NextPage } from "next";
import GenericHeader from "../components/generic_header";
import Logo from "../public/logo.png";
import styles from "../styles/about.module.scss";
import Link from "next/link";
import Footer from "../components/footer";
import Head from "next/head";

const About: NextPage = () => {
  return (
    <>
      <Head>
        <title>About - NerdTree</title>
      </Head>
      <GenericHeader />
      <div className={"flex flex-col items-center justify-center pt-10"}>
        <h1>About Us</h1>
        <div className={styles["about-container"]}>
          <img
            className={"bg-black rounded-2xl"}
            src={Logo.src}
            alt={"NerdTree Logo"}
          />
          <div>
            <p className={"inline text-2xl"}>
              NerdTree started as a group in the IT club of Ongko Organization
              in 2019. With more than 50 active members, NerdTree was one of the
              most active group in Ongko. More than 10 workshops were done
              within 2 years.
              <br />
              <br />
              Now NerdTree is an independent organization. Aiming to spread the
              knowledge of programming among general people. With our own
              website with blogging, it’s never been easier to do that.
            </p>
          </div>
        </div>
        <div className={styles["contact-info"] + " grid p-10"}>
          <h1 className={"text-center"}>Want to contact us?</h1>
          <h3>
            Email:{" "}
            <Link href={"mailto:nerdtree.org@gmail.com"}>
              nerdtree.org@gmail.com
            </Link>
          </h3>
          <h3>
            Facebook:{" "}
            <Link href={"https://facebook.com/nerdtree"}>
              https://facebook.com/nerdtree
            </Link>
          </h3>
        </div>
      </div>
      <Footer />
    </>
  );
};

export default About;
