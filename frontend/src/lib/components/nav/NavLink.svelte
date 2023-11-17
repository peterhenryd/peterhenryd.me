<script lang="ts">
    import {page} from "$app/stores";

    export let href: string;
    export let target: string | undefined = undefined;

    $:currentRoute = $page.url.pathname;
</script>
<span class="uppercase transition hover:text-rose-700 text-xl" class:active={currentRoute === href}>
    <a {href} {target}>
        <slot />
    </a>
</span>
<style lang="postcss">
    .active {
        color: theme("textColor.rose.400");
    }
</style>