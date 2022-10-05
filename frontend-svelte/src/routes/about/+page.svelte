<script lang="ts">
    import Logo from '../../images/logo.png';
</script>

<svelte:head>
    <title>About Us</title>
</svelte:head>

<div class="parent-container flex flex-col items-center justify-center p-20 h-screen">
    <h1>About Us</h1>
    <div class="about-container">
        <img
                class="bg-black rounded-2xl"
                src={Logo}
                alt="NerdTree Logo"
        />
        <div>
            <p class="inline text-2xl">
                NerdTree started as a group in the IT club of Ongko Organization
                in 2019. With more than 50 active members, NerdTree was one of the
                most active group in Ongko. More than 10 workshops were done
                within 2 years.
                <br />
                <br />
                Now NerdTree is an independent organization. Aiming to spread the
                knowledge of programming among general people. With our own
                website with blogging, it’s never been easier to do that.
            </p>
        </div>
    </div>
    <div class="contact-info grid p-10">
        <h1 class="text-center">Want to contact us?</h1>
        <h3>
            Email:
            <a href="mailto:nerdtree.org@gmail.com">
                nerdtree.org@gmail.com
            </a>
        </h3>
        <h3>
            Facebook:
            <a href="https://facebook.com/nerdtree">
                https://facebook.com/nerdtree
            </a>
        </h3>
    </div>
</div>

<style lang="scss">
  .parent-container {
    min-height: 95vh;
  }
  .about-container {
    display: flex;
    padding: 1em;
    gap: 5em;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    margin-top: 5em;
    font-weight: 600;
    letter-spacing: -0.035em;

    div {
      max-width: 500px;
    }
  }

  .contact-info {
    display: grid;
    padding: 5em;

    a {
      color: #4f95ff;
    }
  }
</style>