<script lang="ts">
    import Logo from '../../images/logo.png';
</script>

<svelte:head>
    <title>About Us</title>
</svelte:head>

<div class="parent-container flex flex-col justify-center gap-10 mt-10">
    <h1 class="text-center">About Us</h1>
    <div class="about-container p-5">
        <img class="bg-black rounded-2xl" src={Logo} alt="NerdTree Logo" />
        <div>
            <p class="inline text-2xl">
                NerdTree started as a group in the IT club of Ongko Organization in 2019. With more
                than 50 active members, NerdTree was one of the most active group in Ongko. More
                than 10 workshops were done within 2 years.
                <br />
                <br />
                Now NerdTree is an independent organization. Aiming to spread the knowledge of programming
                among general people. With our own website with blogging, it’s never been easier to do
                that.
            </p>
        </div>
    </div>
    <div class="contact-info flex items-center flex-col">
        <h1 class="text-center">Want to contact us?</h1>
        <span class="text-center">
            Email:
            <a href="mailto:nerdtree.org@gmail.com"> nerdtree.org@gmail.com </a>
        </span>
        <span class="text-center">
            Facebook:
            <a href="https://facebook.com/nerdtree"> https://facebook.com/nerdtree </a>
        </span>
    </div>
</div>

<style lang="scss">
    .parent-container {
        min-height: 95vh;
    }
    .about-container {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        font-weight: 600;
        letter-spacing: -0.035em;
        gap: 5em;

        div {
            max-width: 500px;
        }
    }

    .contact-info {
        display: grid;
        margin-bottom: 2em;

        span {
            font-size: 1.2em;
            font-weight: 600;
        }
        a {
            word-break: break-word;
            color: #4f95ff;
        }
    }
</style>
