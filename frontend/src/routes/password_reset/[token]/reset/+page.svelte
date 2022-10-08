<script lang="ts">
    import '../../../../styles/authform.scss';
    import { page } from '$app/stores';
    import Logo from '../../../../images/logo.png';
    import { createForm } from 'svelte-forms-lib';
    import * as yup from 'yup';
    import { API } from '../../../../api_wrapper';
    import { getUserIdFromAccessToken } from '../../../../api_wrapper/common/jwt';
    import { get } from 'svelte/store';
    import { AuthenticationStatus } from '../../../../stores/user';
    import { goto } from '$app/navigation';
    import { onDestroy } from 'svelte';

    const unsubscribe = AuthenticationStatus.subscribe((status) => {
        if (status.info) {
            goto('/', { replaceState: true });
        }
    });
    let requestDone = false;
    let requestFailed = false;
    let requestMessage = '';
    const reset_token = $page.params.token;
    const { form, errors, isValid, handleSubmit, touched, isSubmitting, validateField } =
        createForm({
            initialValues: {
                Password: '',
                ConfirmPassword: ''
            },
            validationSchema: yup.object({
                Password: yup.string().min(8).max(80).required(),
                ConfirmPassword: yup.string().min(8).max(80).required()
            }),
            onSubmit: async (values) => {
                if (values.Password != values.ConfirmPassword) {
                    get(errors).ConfirmPassword = "Must be equal to 'Password' field";
                }

                const result = await API.auth.password_reset.token({
                    user_id: getUserIdFromAccessToken(reset_token),
                    password: values.Password,
                    reset_token: reset_token
                });

                requestDone = true;
                requestFailed = !result.success;

                if (!requestFailed) {
                    requestMessage =
                        'Successfully resetted your password! Go to login page in order to log in';
                } else {
                    requestMessage = result.error;
                }
            }
        });

    onDestroy(() => {
        unsubscribe();
    });
</script>

<div class="flex justify-center items-center h-screen">
    <div class="form-container">
        <div>
            <img class="bg-black rounded-full" src={Logo} alt="NerdTree logo" />
        </div>
        <a href="/">
            <h1>NerdTree</h1>
        </a>
        <h3>Reset Password</h3>
        {#if !requestDone}
            <form on:submit={handleSubmit}>
                <div class="input-container">
                    <label class="block" for="Password">Password</label>
                    <input
                        id="Password"
                        class={$errors.Password && 'contains-error'}
                        placeholder="Enter a strong password"
                        bind:value={$form.Password}
                        type="password"
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
                        placeholder="Repeat the password here"
                        bind:value={$form.ConfirmPassword}
                        type="password"
                    />
                    {#if $errors.ConfirmPassword}
                        <span class="error-info">{$errors.ConfirmPassword}</span>
                    {/if}
                </div>
                <div class="flex justify-end">
                    <input
                        class="action-button"
                        type="submit"
                        value={`${$isSubmitting ? 'Sending request...' : 'Send request'}`}
                    />
                </div>
            </form>
        {:else}
            <h5 class={`${requestFailed && 'error-info'} text-center p-10`}>{requestMessage}</h5>
        {/if}
    </div>
</div>
