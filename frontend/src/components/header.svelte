<script lang="ts">
    import AuthStatus from '../components/auth_status.svelte';
    import Logo from '../images/logo.png';

    let isSidebarVisible = false;
</script>

<div class="generic-header">
    <a href="/">
        <img class="logo cursor-pointer" src={Logo} alt="NerdTree logo" />
    </a>
    <div class="actions {isSidebarVisible && 'actions-visible'}">
        <div class="cross-container flex w-full justify-end">
            <button class="cross" on:click={() => (isSidebarVisible = false)}>
                <div />
                <div />
            </button>
        </div>
        <div class="actions-links">
            <a href="/blogs">
                <button class="action-link" on:click={() => (isSidebarVisible = false)}>
                    Blogs
                </button>
            </a>
            <a href={'/about'}>
                <button class="action-link" on:click={() => (isSidebarVisible = false)}>
                    About Us
                </button>
            </a>
        </div>
        <AuthStatus />
    </div>
    <button class="breadcrumb" on:click={() => (isSidebarVisible = true)}>
        <div />
        <div />
        <div />
    </button>
</div>
<div style="height: 50px" />

<style lang="scss">
    .generic-header {
        height: 50px;
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: #101010;
        color: #868686;
        letter-spacing: -0.025em;
        font-weight: 600;
        position: fixed;
        top: 0;
        padding: 1em;
        width: 100vw;
        z-index: 999999999999;

        .actions {
            display: flex;
            justify-content: space-between;
            align-items: center;
            flex: 1;
            div {
                display: flex;
                gap: 10px;
            }

            .cross-container {
                display: none;
            }
        }

        img {
            width: 50px;
        }
    }

    @media only screen and (max-width: 700px) {
        .actions-visible {
            display: flex !important;
            margin-top: 0 !important;
        }
        .cross-container {
            display: block !important;
        }
        .actions {
            margin-top: -1000%;
            display: flex !important;
            flex-direction: column;
            align-items: flex-start !important;
            padding: 2em;
            top: 0;
            left: 0;
            position: absolute;
            height: 100vh;
            width: 100vw;
            background: #202020;
            transition: 200ms;

            .cross {
                position: relative;
                height: 30px;
                width: 30px;

                div {
                    top: 15px;
                    left: 7px;
                    position: absolute;
                    background: white;
                    height: 2px;
                    width: 20px;

                    &:nth-child(1) {
                        transform: rotate(45deg);
                    }
                    &:nth-child(2) {
                        transform: rotate(-45deg);
                    }
                }
            }

            .actions-links {
                display: block !important;

                .action-link {
                    font-size: 2em;
                    display: block !important;
                }
            }

            &:nth-child(2) {
                button {
                    font-size: 1.5em;
                }
            }
        }

        .breadcrumb {
            padding: 1em;
            cursor: pointer;

            div {
                height: 2px;
                width: 20px;
                background: white;

                &:nth-child(2),
                &:nth-child(3) {
                    margin-top: 5px;
                }
            }
        }
    }
</style>
