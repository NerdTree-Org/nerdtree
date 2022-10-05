<script lang="ts">
    import AuthStatus from '../components/auth_status.svelte';
    import HorizontalScroll from '../components/horizontal_scroll.svelte';
    import RepoCard from '../components/repo_card.svelte';
    import Logo from '../images/logo.png';
</script>

<svelte:head>
    <title>NerdTree</title>
</svelte:head>

<div>
    <div class="toplevel flex flex-col items-center min-w-screen min-h-screen justify-between">
        <div class="flex justify-end p-4 w-screen">
            <AuthStatus />
        </div>
        <div class="greeting-container w-full">
            <div>
                <h1>We&apos;re NerdTree</h1>
                <h2>An organization aiming to make a better programming community for students</h2>
            </div>
            <img src={Logo} alt="NerdTree Logo" />
        </div>
        <div class="nav-links-container">
            <button id="index">/</button>
            <a href="/blogs">
                <span class="nav-link">Blogs</span>
            </a>
            <a href="/workshops">
                <span class="nav-link">Workshops</span>
            </a>
            <a href="/about">
                <span class="nav-link">About us</span>
            </a>
        </div>
    </div>
    <div class="p-5">
        <h2>Latest Blogs</h2>
        <HorizontalScroll />
    </div>
    <div class="p-5">
        <h2>Github Projects</h2>
        <HorizontalScroll>
            <RepoCard
                link={'https://github.com/NerdTree-Org/NerdTree'}
                issues={'https://github.com/NerdTree-Org/NerdTree/issues'}
                maintainers={['mdgaziur001']}
                repo={'NerdTree-Org/NerdTree'}
            />
            <RepoCard
                link={'https://github.com/NerdTree-Org/NerdTree-Wishlist'}
                issues={'https://github.com/NerdTree-Org/NerdTree-Wishlist/issues'}
                maintainers={['mdgaziur001']}
                repo={'NerdTree-Org/NerdTree-Wishlist'}
            />
        </HorizontalScroll>
    </div>
</div>

<style lang="scss">
    .toplevel {
        background: radial-gradient(
            67.11% 67.11% at 47.48% 18.31%,
            #333333 0%,
            rgba(36, 36, 36, 0) 100%
        );
    }

    .greeting-container {
        padding: 2.5rem;
        gap: 50px;
        display: flex;
        align-content: center;
        justify-content: space-around;
        flex-wrap: wrap-reverse;

        div {
            display: flex;
            align-items: flex-start;
            justify-content: center;
            flex-direction: column;

            h1 {
                display: block;
            }

            h2 {
                display: block;
                font-size: 25px;
                max-width: 400px;
            }
        }

        img {
            border-radius: 100%;
        }
    }

    .nav-links-container {
        background: #343434;
        border: 1px solid rgba(255, 255, 255, 0.24);
        border-radius: 13px;
        padding: 0.5em;
        margin: 1em;
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 10px;

        button {
            padding: 0.5em 1em 0.5em 1em;
            background: #646464;
            border: 1px solid rgba(255, 255, 255, 0.16);
            border-radius: 8px;
            text-align: left;
        }

        .nav-link {
            padding: 0.5em 1em 0.5em 1em;
            font-weight: 600;
            font-size: 100%;
            cursor: pointer;
            user-select: none;
            background-color: transparent;
            border-radius: 8px;
            transition: 200ms;

            &:hover {
                background: #101010;
            }
        }
    }

    @media only screen and (max-width: 423px) {
        .nav-links-container {
            flex-direction: column;
            width: 95%;
        }
    }
</style>
