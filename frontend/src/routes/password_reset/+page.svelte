<script lang="ts">
    import '../../styles/authform.scss';
    import Logo from '../../images/logo.png';
    import { createForm } from 'svelte-forms-lib';
    import * as yup from 'yup';
    import { API } from '../../api_wrapper';
    import { AuthenticationStatus } from '../../stores/user';
    import { goto } from '$app/navigation';
    import { onDestroy } from 'svelte';

    const unsubscribe = AuthenticationStatus.subscribe((status) => {
        if (status.info) {
            goto('/', { replaceState: true });
        }
    });

    let requestDone = false;
    let requestMessage = '';
    let requestFailed = false;
    const { form, errors, handleSubmit, isSubmitting } = createForm({
        initialValues: {
            Email: ''
        },
        validationSchema: yup.object({
            Email: yup.string().email().required()
        }),
        onSubmit: async (values) => {
            const result = await API.auth.password_reset.request({
                email: values.Email
            });

            requestDone = true;
            requestFailed = !result.success;

            if (!requestFailed) {
                requestMessage =
                    'Successfully sent password reset request. Please check your inbox including spam.';
            } else {
                requestMessage = result.error;
            }
        }
    });

    onDestroy(() => {
        unsubscribe();
    });
</script>

<svelte:head>
    <title>Reset Password - NerdTree</title>
</svelte:head>

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
                    <label class="block" for="Email">Email</label>
                    <input
                        id="Email"
                        class={$errors.Email && 'contains-error'}
                        placeholder="user@example.com"
                        bind:value={$form.Email}
                    />
                    {#if $errors.Email}
                        <span class="error-info">{$errors.Email}</span>
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
