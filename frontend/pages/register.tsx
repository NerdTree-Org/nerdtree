import { NextPage } from "next";
import styles from "../styles/authform.module.scss";
import Logo from "../public/logo.png";
import { SubmitHandler, useForm } from "react-hook-form";
import Link from "next/link";
import { useState } from "react";
import Head from "next/head";

interface IFormInputs {
  firstName: string;
  lastName: string;
  email: string;
  username: string;
  facebook_id: string;
  password: string;
  confirm_password: string;
}
const Register: NextPage = () => {
  const {
    register,
    formState: { errors },
    handleSubmit,
    trigger,
    getValues,
  } = useForm<IFormInputs>();
  const onSubmit: SubmitHandler<IFormInputs> = () => {}; // TODO: implement registration
  const [isFirstPart, setIsFirstPart] = useState<boolean>(true);

  return (
    <div className={"flex justify-center items-center h-screen"}>
      <Head>
        <title>Register - NerdTree</title>
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
          <h1 className={"cursor-pointer"}>Join NerdTree</h1>
        </Link>
        <form onSubmit={handleSubmit(onSubmit)}>
          {isFirstPart && (
            <>
              <div className={styles["input-container"]}>
                <label className={"block"}>First name</label>
                <input
                  className={errors.firstName && styles["contains-error"]}
                  type={"text"}
                  placeholder={"John"}
                  {...register("firstName", {
                    required: true,
                    minLength: 3,
                    maxLength: 255,
                    setValueAs(value) {
                      return value.trim();
                    },
                  })}
                />
                <span className={styles["error-info"]}>
                  {errors.firstName?.type == "maxLength" &&
                    "Maximum length for firstname is 255 characters"}
                  {errors.firstName?.type == "required" &&
                    "First name is required"}
                  {errors.firstName?.type == "minLength" &&
                    "Minimum length for firstname is 3 characters"}
                </span>
              </div>
              <div className={styles["input-container"]}>
                <label className={"block"}>Last name</label>
                <input
                  className={errors.lastName && styles["contains-error"]}
                  type={"text"}
                  placeholder={"Doe"}
                  {...register("lastName", {
                    required: true,
                    minLength: 3,
                    maxLength: 255,
                    setValueAs(value) {
                      return value.trim();
                    },
                  })}
                />
                <span className={styles["error-info"]}>
                  {errors.lastName?.type == "maxLength" &&
                    "Maximum length for lastname is 255 characters"}
                  {errors.lastName?.type == "required" &&
                    "Last name is required"}
                  {errors.lastName?.type == "minLength" &&
                    "Minimum length for lastname is 3 characters"}
                </span>
              </div>
              <div className={styles["input-container"]}>
                <label className={"block"}>Email</label>
                <input
                  className={errors.email && styles["contains-error"]}
                  type={"email"}
                  placeholder={"user@example.com"}
                  {...register("email", {
                    required: true,
                    pattern: /^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$/g,
                    setValueAs(value) {
                      return value.trim();
                    },
                  })}
                />
                <span className={styles["error-info"]}>
                  {errors.email?.type == "pattern" && "Invalid email"}
                  {errors.email?.type == "required" && "Email is required"}
                </span>
              </div>
              <div className={"flex justify-end"}>
                <button
                  className={styles["action-button"]}
                  onClick={() => {
                    trigger().then((is_valid) => {
                      if (is_valid) {
                        setIsFirstPart(!isFirstPart);
                      }
                    });
                  }}
                >
                  Next
                </button>
              </div>
            </>
          )}
          {!isFirstPart && (
            <>
              <div className={styles["input-container"]}>
                <label className={"block"}>Username</label>
                <input
                  className={errors.username && styles["contains-error"]}
                  type={"text"}
                  placeholder={"johndoe1234"}
                  {...register("username", {
                    required: true,
                    minLength: 3,
                    maxLength: 25,
                    validate(value) {
                      return !value.includes(" ");
                    },
                  })}
                />
                <span className={styles["error-info"]}>
                  {errors.username?.type == "maxLength" &&
                    "Maximum length for username is 25 characters"}
                  {errors.username?.type == "required" &&
                    "Username is required"}
                  {errors.username?.type == "minLength" &&
                    "Minimum length for username is 3 characters"}
                  {errors.username?.type == "validate" &&
                    "Username cannot contain spaces"}
                </span>
              </div>
              <div className={styles["input-container"]}>
                <label className={"block"}>Password</label>
                <input
                  className={errors.password && styles["contains-error"]}
                  type={"password"}
                  placeholder={"Password"}
                  {...register("password", {
                    required: true,
                    minLength: 8,
                    maxLength: 80,
                  })}
                />
                <span className={styles["error-info"]}>
                  {errors.password?.type == "maxLength" &&
                    "Maximum length for password is 80 characters"}
                  {errors.password?.type == "required" &&
                    "Password is required"}
                  {errors.password?.type == "minLength" &&
                    "Minimum length for password is 8 characters"}
                </span>
              </div>
              <div className={styles["input-container"]}>
                <label className={"block"}>Confirm password</label>
                <input
                  className={
                    errors.confirm_password && styles["contains-error"]
                  }
                  type={"password"}
                  placeholder={"Confirm password"}
                  {...register("confirm_password", {
                    required: true,
                    minLength: 8,
                    maxLength: 80,
                    validate: (value) => value == getValues("password"),
                  })}
                />
                <span className={styles["error-info"]}>
                  {errors.confirm_password?.type == "maxLength" &&
                    "Maximum length for password is 80 characters"}
                  {errors.confirm_password?.type == "required" &&
                    "Password is required"}
                  {errors.confirm_password?.type == "minLength" &&
                    "Minimum length for password is 8 characters"}
                  {errors.confirm_password?.type == "validate" &&
                    "Passwords must be equal"}
                </span>
              </div>
              <div className={"flex justify-end"}>
                <button
                  className={styles["action-button"]}
                  onClick={() => setIsFirstPart(!isFirstPart)}
                >
                  Back
                </button>
                <input
                  className={styles["action-button"]}
                  type={"submit"}
                  value={"Register"}
                />
              </div>
            </>
          )}
          <div className={"flex justify-center"}>
            <Link href={"/login"}>
              <span className={`${styles["other-form-link"]} cursor-pointer`}>
                Already a member?
              </span>
            </Link>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Register;
