<script lang="ts">
    import '../../styles/authform.scss';
    import { goto } from '$app/navigation';
    import { AuthenticationStatus } from '../../stores/user';
    import Logo from '../../images/logo.png';
    import { createForm } from 'svelte-forms-lib';
    import * as yup from 'yup';
    import { get } from 'svelte/store';
    import { API } from '../../api_wrapper';
    import { onDestroy } from 'svelte';

    let registrationInfo;
    let registrationError;
    let firstPart = true;

    const { form, errors, isValid, handleSubmit, touched, isSubmitting, validateField } =
        createForm({
            initialValues: {
                Firstname: '',
                Lastname: '',
                FacebookID: '',
                Email: '',
                Username: '',
                Password: '',
                ConfirmPassword: ''
            },
            validationSchema: yup.object({
                Firstname: yup.string().trim().min(3).max(255).required(),
                Lastname: yup.string().trim().min(3).max(255).required(),
                FacebookID: yup.string().trim().max(80).required(),
                Email: yup.string().trim().email().required(),
                Username: yup.string().trim().min(3).max(25).required(),
                Password: yup.string().trim().min(8).max(80).required(),
                ConfirmPassword: yup.string().trim().min(8).max(80).required()
            }),
            onSubmit: async (values) => {
                if (values.ConfirmPassword !== values.Password) {
                    errors.set({
                        ...get(errors),
                        ConfirmPassword: "Must be equal to 'Password' field"
                    });
                    return;
                }

                const result = await API.auth.register({
                    firstname: values.Firstname,
                    lastname: values.Lastname,
                    username: values.Username,
                    email: values.Email,
                    facebook_id: values.FacebookID,
                    password: values.Password
                });

                if (result.success) {
                    $form = {};
                    registrationInfo =
                        'Registered successfully! Please go to login page to log into your new account';
                    registrationError = '';
                } else {
                    registrationInfo = '';
                    registrationError = result.error;
                }
                firstPart = true;
            }
        });

    const unsubscribe = AuthenticationStatus.subscribe((status) => {
        if (status.info) {
            goto('/', { replaceState: true });
        }
    });

    onDestroy(() => {
        unsubscribe();
    });
</script>

<svelte:head>
    <title>Register - NerdTree</title>
</svelte:head>

<div class="flex justify-center items-center h-screen">
    <div class="form-container">
        <div>
            <img class="bg-black rounded-full" src={Logo} alt="NerdTree logo" />
        </div>
        <a href="/">
            <h1>NerdTree</h1>
        </a>
        {#if registrationError}
            <span class="error-info">{registrationError}</span>
        {:else if registrationInfo}
            <span class="text-center">{registrationInfo}</span>
        {/if}
        <form class:valid={isValid} on:submit={handleSubmit}>
            <div class={!firstPart && 'hidden'}>
                <div class="input-container">
                    <label class="block" for="Firstname">Firstname</label>
                    <input
                        id="Firstname"
                        class={$errors.Firstname && 'contains-error'}
                        name="Firstname"
                        placeholder="John"
                        bind:value={$form.Firstname}
                    />
                    {#if $errors.Firstname}
                        <span class="error-info">{$errors.Firstname}</span>
                    {/if}
                </div>
                <div class="input-container">
                    <label class="block" for="Lastname">Lastname</label>
                    <input
                        id="Lastname"
                        class={$errors.Lastname && 'contains-error'}
                        name="Lastname"
                        placeholder="Doe"
                        bind:value={$form.Lastname}
                    />
                    {#if $errors.Lastname}
                        <span class="error-info">{$errors.Lastname}</span>
                    {/if}
                </div>
                <div class="input-container">
                    <label class="block" for="Email">Email</label>
                    <input
                        id="Email"
                        class={$errors.Email && 'contains-error'}
                        name="Email"
                        placeholder="johndoe@example.com"
                        bind:value={$form.Email}
                    />
                    {#if $errors.Email}
                        <span class="error-info">{$errors.Email}</span>
                    {/if}
                </div>
                <div class="flex justify-end">
                    <button
                        class="action-button"
                        on:click|preventDefault={async () => {
                            await validateField('Firstname');
                            await validateField('Lastname');
                            await validateField('Email');
                            if (!$errors.Email && !$errors.Firstname && !$errors.Lastname) {
                                firstPart = false;
                            }
                        }}>Next</button
                    >
                </div>
            </div>
            <div class={firstPart && 'hidden'}>
                <div class="input-container">
                    <label class="block" for="FacebookID">Facebook ID</label>
                    <input
                        id="FacebookID"
                        class={$errors.FacebookID && 'contains-error'}
                        name="FacebookID"
                        placeholder="john_doe"
                        bind:value={$form.FacebookID}
                    />
                    {#if $errors.FacebookID}
                        <span class="error-info">{$errors.FacebookID}</span>
                    {/if}
                </div>
                <div class="input-container">
                    <label class="block" for="Username">Username</label>
                    <input
                        id="Username"
                        class={$errors.Username && 'contains-error'}
                        name="Username"
                        placeholder="johndoe"
                        bind:value={$form.Username}
                    />
                    {#if $errors.Username}
                        <span class="error-info">{$errors.Username}</span>
                    {/if}
                </div>
                <div class="input-container">
                    <label class="block" for="Password">Password</label>
                    <input
                        id="Password"
                        class={$errors.Password && 'contains-error'}
                        name="Password"
                        placeholder="Your strong password here"
                        type="password"
                        bind:value={$form.Password}
                    />
                    {#if $errors.Password}
                        <span class="error-info">{$errors.Password}</span>
                    {/if}
                </div>
                <div class="input-container">
                    <label class="block" for="ConfirmPassword">Confirm Password</label>
                    <input
                        id="ConfirmPassword"
                        class={$errors.ConfirmPassword && 'contains-error'}
                        name="ConfirmPassword"
                        placeholder="Confirm the password"
                        type="password"
                        bind:value={$form.ConfirmPassword}
                    />
                    {#if $errors.ConfirmPassword}
                        <span class="error-info">{$errors.ConfirmPassword}</span>
                    {/if}
                </div>
                <div class="flex justify-end">
                    <button class="action-button" on:click|preventDefault={() => (firstPart = true)}
                        >Back</button
                    >
                    <input
                        class="action-button"
                        type="submit"
                        value={`${$isSubmitting ? 'Registering...' : 'Register'}`}
                    />
                </div>
            </div>
            <div class="flex justify-center mt-5">
                <a href="/login">
                    <span class="other-form-link cursor-pointer">Already a member?</span>
                </a>
            </div>
        </form>
    </div>
</div>
