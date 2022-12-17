<script>
  import "../app.css";
  import Header from '../lib/Header.svelte'
  import Footer from '../lib/Footer.svelte'

  
</script>

<main class="full">
  <Header />
  <slot />
  <Footer />
</main>
