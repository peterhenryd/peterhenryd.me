<script lang="ts">
    import BlogPost from "./BlogPost.svelte";

    export let data;
</script>
<div class="flex flex-col gap-5">
    {#each data.posts as post}
        <BlogPost {post} />
    {/each}
</div>