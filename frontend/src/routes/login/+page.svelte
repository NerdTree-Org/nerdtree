<script lang="ts">
    import * as yup from 'yup';
    import { createForm } from 'svelte-forms-lib';
    import { API } from '../../api_wrapper';
    import { goto } from '$app/navigation';
    import '../../styles/authform.scss';
    import Logo from '../../images/logo.png';
    import { AuthenticationStatus } from '../../stores/user';
    import { onDestroy } from 'svelte';

    const unsubscribe = AuthenticationStatus.subscribe((status) => {
        if (status.info) {
            goto('/', { replaceState: true });
        }
    });

    let loginError = '';
    const { form, errors, isValid, handleSubmit, touched, isSubmitting } = createForm({
        initialValues: {
            Username: '',
            Password: ''
        },
        validationSchema: yup.object().shape({
            Username: yup.string().required(),
            Password: yup.string().required()
        }),
        onSubmit: async (values) => {
            const result = await API.auth.login({
                username: values.Username,
                password: values.Password
            });

            if (result.success) {
                await goto('/', { replaceState: true });
            } else {
                loginError = result.error;
            }
        }
    });

    onDestroy(() => {
        unsubscribe();
    });
</script>

<svelte:head>
    <title>Log In - NerdTree</title>
</svelte:head>

<div class="flex justify-center items-center h-screen">
    <div class="form-container">
        <div>
            <img class="bg-black rounded-full" src={Logo} alt="NerdTree logo" />
        </div>
        <a href="/">
            <h1>NerdTree</h1>
        </a>
        {#if loginError}
            <span class="error-info">{loginError}</span>
        {/if}
        <form class:valid={$isValid} on:submit={handleSubmit}>
            <div class="input-container">
                <label class="block" for="Username">Username</label>
                <input
                    id="Username"
                    class={$errors.Username && 'contains-error'}
                    name="Username"
                    placeholder="user1234"
                    bind:value={$form.Username}
                />
                {#if $errors.Username}
                    <span class="error-info">{$errors.Username}</span>
                {/if}
            </div>
            <div class="input-container">
                <label class="block" for="Password">Password</label>
                <input
                    class={$errors.Password && 'contains-error'}
                    name="Password"
                    id="Password"
                    type="password"
                    placeholder="password1234"
                    bind:value={$form.Password}
                />
                {#if $errors.Password}
                    <span class="error-info">{$errors.Password}</span>
                {/if}
            </div>
            <div class="flex justify-end items-center gap-2">
                <a href="/password_reset">
                    <span class="other-form-link cursor-pointer">Forgot Password?</span>
                </a>
                <input
                    class="action-button"
                    type="submit"
                    value={`${$isSubmitting ? 'Logging in...' : 'Log In'}`}
                />
            </div>
            <div class="flex justify-center mt-5">
                <a href="/register">
                    <span class="other-form-link cursor-pointer">Join NerdTree</span>
                </a>
            </div>
        </form>
    </div>
</div>
