import { NextPage } from "next";
import styles from "../styles/authform.module.scss";
import Logo from "../public/logo.png";
import { SubmitHandler, useForm } from "react-hook-form";
import Link from "next/link";
import Head from "next/head";

interface IFormInputs {
  email: string;
  password: string;
}

const Login: NextPage = () => {
  const {
    register,
    formState: { errors },
    handleSubmit,
  } = useForm<IFormInputs>();
  const onSubmit: SubmitHandler<IFormInputs> = () => {}; // TODO: implement log in

  return (
    <div className={"flex justify-center items-center h-screen"}>
      <Head>
        <title>Log In - NerdTree</title>
      </Head>
      <div className={styles["login-form"]}>
        <div>
          <img
            className={"bg-black rounded-full"}
            src={Logo.src}
            alt={"NerdTree Logo"}
          />
        </div>
        <Link href={"/"}>
          <h1 className={"cursor-pointer"}>NerdTree</h1>
        </Link>
        <form onSubmit={handleSubmit(onSubmit)}>
          <div className={styles["input-container"]}>
            <label className={"block"}>Email</label>
            <input
              type={"email"}
              placeholder={"user@example.com"}
              {...register("email", {
                required: true,
                pattern: /^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$/g,
              })}
            />
            <span className={styles["error-info"]}>
              {errors.email && "Invalid or missing email"}
            </span>
          </div>
          <div className={styles["input-container"]}>
            <label className={"block"}>Password</label>
            <input
              type={"password"}
              placeholder={"Your Password"}
              {...register("password", { required: true })}
            />
            <span className={styles["error-info"]}>
              {errors.password && "Password is required"}
            </span>
          </div>
          <div className={"flex justify-end"}>
            <input
              className={styles["action-button"]}
              type={"submit"}
              value={"Log In"}
            />
          </div>
          <div className={"flex justify-center"}>
            <Link href={"/register"}>
              <span className={`${styles["other-form-link"]} cursor-pointer`}>
                Join NerdTree
              </span>
            </Link>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Login;
