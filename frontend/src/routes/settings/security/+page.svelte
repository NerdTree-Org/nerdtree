<script lang="ts">
    import SideBar from '../../../components/user_settings_sidebar.svelte';
    import { createForm } from 'svelte-forms-lib';
    import * as yup from 'yup';
    import { get } from 'svelte/store';
    import { API } from '../../../api_wrapper';
    import NerdTreeButton, { ButtonType } from '../../../components/nerdtree_button.svelte';
    import { onDestroy, onMount } from 'svelte';
    import { AuthenticationStatus } from '../../../stores/user';
    import { getAccessToken } from '../../../api_wrapper/common/store_auth_info_cookie';
    import { goto } from '$app/navigation';
    import { ENV } from '../../../env';

    let unsubscribe: () => void = () => {
        /**/
    };
    onMount(() => {
        unsubscribe = AuthenticationStatus.subscribe((status) => {
            if (!status.info && typeof getAccessToken() === 'undefined') {
                goto('/');
            }
        });
    });

    let updateError = '';
    const { form, errors, isValid, handleSubmit, isSubmitting } = createForm({
        initialValues: {
            OldPassword: '',
            NewPassword: '',
            ConfirmPassword: ''
        },
        validationSchema: yup.object({
            OldPassword: yup.string().required(),
            NewPassword: yup.string().trim().min(8).max(80).required(),
            ConfirmPassword: yup.string().trim().min(8).max(80).required()
        }),
        onSubmit: async (values) => {
            updateError = '';
            if (values.NewPassword !== values.ConfirmPassword) {
                errors.set({
                    ...get(errors),
                    ConfirmPassword: "Must be equal to 'New Password' field"
                });
            }

            const result = await API.user.update.password({
                new_password: values.NewPassword,
                old_password: values.OldPassword
            });

            if (result.success) {
                alert('Successfully updated your password!');
                $form = {};
            } else {
                updateError = `Failed to update password: ${result.error}`;
            }
        }
    });

    onDestroy(() => {
        unsubscribe();
    });
</script>

<svelte:head>
    <title>Security Information - NerdTree</title>
</svelte:head>

<div class="top-container flex flex-wrap">
    <SideBar idx="2" />
    <form class="flex gap-7 flex-col p-10 flex-1 max-w-full" on:submit={handleSubmit}>
        <span class="error">{updateError}</span>
        <div class="input-container">
            <label for="old_password">Old password</label>
            <span class="error">{$errors.OldPassword}</span>
            <input
                id="old_password"
                type="password"
                bind:value={$form.OldPassword}
                placeholder="Enter previous password"
            />
        </div>
        <div class="input-container">
            <label for="new_password">New password</label>
            <span class="error">{$errors.NewPassword}</span>
            <input
                id="new_password"
                type="password"
                bind:value={$form.NewPassword}
                placeholder="Enter a strong 8 character password"
            />
        </div>
        <div class="input-container">
            <label for="confirm_password">Confirm password</label>
            <span class="error">{$errors.ConfirmPassword}</span>
            <input
                id="confirm_password"
                type="password"
                bind:value={$form.ConfirmPassword}
                placeholder="Type the same password again"
            />
        </div>
        <div class="p-10 pl-0">
            <NerdTreeButton type={ButtonType.Smooth}>
                {$isSubmitting ? 'Updating...' : 'Update'}
            </NerdTreeButton>
        </div>
    </form>
</div>

<style lang="scss">
    .top-container {
        min-height: 87vh;
    }

    p {
        font-weight: 600;
        font-size: 20px;
    }

    .error {
        color: #ff2929;
        padding: 0.5em 0;
    }

    .input-container {
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 0.2em;
        max-width: min(700px, 100%);

        label {
            font-size: 24px;
            color: #8d8d8d;
            font-weight: 600;
        }

        input {
            background: none;
            outline: none;
            font-size: 36px;
            font-weight: 600;
            letter-spacing: -2.5%;
            font-family: 'Poppins', sans-serif;
            border: 2px solid #282828;
            border-bottom: 5px solid #303030;
            transition: 200ms;
            padding: 0.6em 1em 0.6em 1em;
            border-radius: 9px;

            &:focus {
                border-bottom: 5px solid #606060;
            }
        }
    }

    .action-button {
        color: white;
        background: linear-gradient(104.74deg, #0b0b0b 21.31%, #212121 154.74%);
        border-radius: 9px;
        padding: 0.5em 1.5em 0.5em 1.5em;
        margin-right: 0.5em;
        cursor: pointer;
        transition: 200ms;
        border: 1px solid #202020;

        &:active {
            background-size: 150%;
        }
    }

    @media only screen and (min-width: 650px) {
        .root-nodes-list {
            @apply pl-5;
        }
    }
</style>
