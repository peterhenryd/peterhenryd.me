<script lang="ts">
    import AdminDashboard from "./AdminDashboard.svelte";
    import AdminLogin from "./AdminLogin.svelte";

    export let data;
</script>
{#if data.isAdmin}
    <AdminDashboard />
{:else}
    <AdminLogin />
{/if}