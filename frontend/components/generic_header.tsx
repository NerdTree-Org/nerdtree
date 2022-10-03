import Logo from "../public/logo.png";
import styles from "../styles/generic_header.module.scss";
import Link from "next/link";
import AuthStatus from "./auth_status";
import { useState } from "react";

const GenericHeader = () => {
  const [isSidebarVisible, setIsSidebarVisible] = useState<boolean>(false);

  return (
    <>
      <div className={styles["generic-header"]}>
        <Link href={"/"}>
          <img
            className={styles["logo"] + " cursor-pointer"}
            src={Logo.src}
            alt={"NerdTree logo"}
          />
        </Link>
        <div
          className={`${styles["actions"]} ${
            isSidebarVisible && styles["actions-visible"]
          }`}
        >
          <div
            className={
              styles["cross-container"] + " " + "flex w-full justify-end"
            }
          >
            <div
              className={styles["cross"]}
              onClick={() => setIsSidebarVisible(!isSidebarVisible)}
            >
              <div></div>
              <div></div>
            </div>
          </div>
          <div className={styles["actions-links"]}>
            <Link href={"/blogs"}>
              <span
                className={styles["action-link"]}
                onClick={() => setIsSidebarVisible(false)}
              >
                Blogs
              </span>
            </Link>
            <Link href={"/about"}>
              <span
                className={styles["action-link"]}
                onClick={() => setIsSidebarVisible(false)}
              >
                About Us
              </span>
            </Link>
          </div>
          <AuthStatus />
        </div>
        <div
          className={styles["breadcrumb"]}
          onClick={() => setIsSidebarVisible(!isSidebarVisible)}
        >
          <div></div>
          <div></div>
          <div></div>
        </div>
      </div>
      <div style={{ height: "50px" }}></div>
    </>
  );
};

export default GenericHeader;
