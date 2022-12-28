<script lang="ts">
    import SideBar from '../../components/user_settings_sidebar.svelte';
    import type { User } from '../../interfaces/user';
    import { onDestroy, onMount } from 'svelte';
    import { AuthenticationStatus } from '../../stores/user';
    import { getAccessToken } from '../../api_wrapper/common/store_auth_info_cookie';
    import { goto } from '$app/navigation';
    import { get } from 'svelte/store';
    import DefaultAvatar from '../../images/default-avatar.png';
    import { ENV } from '../../env.js';
    import NerdTreeButton, { ButtonType } from '../../components/nerdtree_button.svelte';
    import * as yup from 'yup';
    import { API } from '../../api_wrapper';

    let user: User | null = null;
    let has_selected_profile_pic = false;
    let profile_pic_file_name = '';
    let profile_pic = '';
    let is_updating = false;
    let errors = null;

    let unsubscribe: () => void = () => {
        /**/
    };
    onMount(() => {
        unsubscribe = AuthenticationStatus.subscribe((status) => {
            if (!status.info && typeof getAccessToken() === 'undefined') {
                goto('/');
            }

            user = get(AuthenticationStatus).info?.user;
            if (!has_selected_profile_pic)
                profile_pic = `${ENV.api_address}/static/${
                    user?.profile_pic ? user.profile_pic : DefaultAvatar
                }`;
        });
    });

    async function updateInfo() {
        if (is_updating) return;
        is_updating = true;

        const firstname = document.querySelector('#firstname').value;
        const lastname = document.querySelector('#lastname').value;
        const email = document.querySelector('#email').value;

        let schema = yup.object({
            Firstname: yup.string().trim().min(3).max(255).required(),
            Lastname: yup.string().trim().min(3).max(255).required(),
            Email: yup.string().trim().email().required()
        });
        let validation;
        try {
            validation = await schema.validate({
                Firstname: firstname,
                Lastname: lastname,
                Email: email
            });
            errors = null;
        } catch (e) {
            errors = e;
        }

        const firstname_validated = validation.Firstname;
        const lastname_validated = validation.Lastname;
        const email_validated = validation.Email;

        try {
            {
                const result = await API.user.update.firstname({
                    firstname: firstname_validated
                });

                if (!result.success) {
                    errors = {
                        errors: [result.error]
                    };
                    return;
                }
            }

            {
                const result = await API.user.update.lastname({
                    lastname: lastname_validated
                });

                if (!result.success) {
                    errors = {
                        errors: [result.error]
                    };
                    return;
                }
            }

            {
                const result = await API.user.update.email({
                    email: email_validated
                });

                if (!result.success) {
                    errors = {
                        errors: [result.error]
                    };
                    return;
                }
            }

            {
                if (has_selected_profile_pic) {
                    let profile_pic_input =
                        document.querySelector<HTMLInputElement>('#profile-pic-selector');
                    let profile_pic = profile_pic_input.files?.item(0);
                    let form_data = new FormData();
                    form_data.append('profile_pic', profile_pic);

                    const result = await API.user.update.profile_pic({
                        form_data
                    });

                    if (!result.success) {
                        alert(JSON.stringify(result));
                        errors = {
                            errors: [result.message]
                        };
                        return;
                    }
                }
            }
        } catch (e) {
            errors = {
                errors: [`Failed to update information: ${e}`]
            };
        }

        if (!errors) {
            alert('Successfully updated information!');
            location.reload();
        }

        is_updating = false;
    }

    function selectProfilePic() {
        let input = document.querySelector<HTMLInputElement>('#profile-pic-selector');
        has_selected_profile_pic = true;
        profile_pic_file_name = input.value.split('\\').pop();

        if (input.files && input.files[0]) {
            if (input.files[0].size / (1024 * 1024) > 2) {
                alert('Max file size is 2MiB');
                input.value = null;
                return;
            }

            let reader = new FileReader();
            reader.onload = (e) => {
                profile_pic = <string>e.target.result;
            };
            reader.readAsDataURL(input.files[0]);
        }
    }

    onDestroy(() => {
        unsubscribe();
    });
</script>

<svelte:head>
    <title>Basic Settings - NerdTree</title>
</svelte:head>

<div class="top-container flex flex-wrap">
    <SideBar />
    <div class="p-2 flex-1 max-w-full">
        <div class="profile-pic-upload flex flex-wrap items-center gap-10">
            <img class="profile-pic-preview" src={profile_pic} alt="Profile picture" />
            <div>
                <h1 class="block">Change Profile Picture</h1>
                <span>Must be a valid format and size should not exceed 2MiB</span>
                <div class="flex gap-5 items-center flex-wrap">
                    <label for="profile-pic-selector">Select Picture</label>
                    {#if has_selected_profile_pic}
                        <span>Currently selected: {profile_pic_file_name}</span>
                    {/if}
                    <input
                        on:change={selectProfilePic}
                        accept="image/png"
                        class="hidden"
                        type="file"
                        id="profile-pic-selector"
                    />
                </div>
            </div>
        </div>
        {#if errors}
            {#each errors?.errors as error}
                <span class="error">{error}</span>
            {/each}
        {/if}
        <div class="inputs p-5 flex flex-col gap-10">
            <div class="input-container">
                <label for="firstname">First Name</label>
                <input id="firstname" value={user?.firstname} placeholder="John" />
            </div>
            <div class="input-container">
                <label for="lastname">Last Name</label>
                <input id="lastname" value={user?.lastname} placeholder="Doe" />
            </div>
            <div class="input-container">
                <label for="email">Email</label>
                <input id="email" value={user?.email} placeholder="johndoe@example.com" />
            </div>
        </div>
        <div class="p-10 pl-5">
            <NerdTreeButton on_click={updateInfo} type={ButtonType.Smooth}>
                {is_updating ? 'Updating...' : 'Update'}
            </NerdTreeButton>
        </div>
    </div>
</div>

<style lang="scss">
    .top-container {
        min-height: 87vh;
    }

    .error {
        display: block;
        margin: 1em;
        padding: 1em 1em;
        background: #ff2929;
        border-radius: 0.5em;
    }

    .profile-pic-upload {
        background: linear-gradient(100.75deg, #2f2f2f 30.53%, rgba(47, 47, 47, 0) 108%);
        border-radius: 20px;
        padding: 2em;
        margin: 1em;

        img {
            height: 200px;
            width: 200px;
            border-radius: 50%;
        }

        h1 {
            color: #c8c8c8;
        }
        span {
            font-weight: 600;
            font-size: 24px;
            color: #8d8d8d;
            letter-spacing: -2.5%;
        }

        div {
            padding-top: 1em;
            label {
                background: linear-gradient(102.61deg, #4b4b4b 25.57%, rgba(75, 75, 75, 0) 108.53%);
                border-radius: 9px;
                padding: 0.5em 1em;
                font-weight: 600;
                font-size: 21.78px;
                cursor: pointer;
            }

            span {
                font-size: 21.78px;
            }
        }
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
</style>
