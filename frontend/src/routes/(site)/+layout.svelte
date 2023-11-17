<script>
    import Header from "./Header.svelte";
    import Footer from "./Footer.svelte";
</script>
<div class="page flex flex-col gap-14">
    <Header />
    <div>
        <slot />
    </div>
    <Footer />
</div>
<style>
    .page {
        margin: 50px 25%;
    }
</style>