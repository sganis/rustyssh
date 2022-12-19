<script>
  import 'bootstrap/dist/css/bootstrap.min.css';
  import 'bootstrap-icons/font/bootstrap-icons.css'
  import "../app.css";
  // import 'bootstrap/dist/js/bootstrap.bundle';
  
  import Header from '../lib/Header.svelte'
  import Footer from '../lib/Footer.svelte'

  
</script>

<main class="full">
  <Header />
  <slot />
  <Footer />
</main>
