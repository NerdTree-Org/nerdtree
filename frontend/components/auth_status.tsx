import Link from "next/link";
import { useDispatch, useSelector } from "react-redux";
import { selectAuthUser } from "../store/authSlice";
import styles from "../styles/auth_status.module.scss";

const AuthStatus = () => {
  const user = useSelector(selectAuthUser);

  return (
    <div className={`${styles["account-links"]} flex justify-end`}>
      {user ? (
        <div>
          <Link href={`/profile/${user.username}`}>
            <span className={styles["link"]}>
              {user.firstname + " " + user.lastname}
            </span>
          </Link>
        </div>
      ) : (
        <div className="flex gap-3">
          <Link href="/login">
            <span className={styles["link"]}>Log In</span>
          </Link>
          <span className="font-extralight p-2">or</span>
          <Link href="/register">
            <span className={styles["link"]} id={styles["register"]}>
              Register
            </span>
          </Link>
        </div>
      )}
    </div>
  );
};

export default AuthStatus;
