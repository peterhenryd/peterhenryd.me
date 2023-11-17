<script lang="ts">
    import {short_date_options} from "$lib";

    export let post: BlogPost;
</script>
<a href="/blog/{post.id}">
    <div class="rounded-xl transition-shadow shadow-lg hover:shadow-xl gap-3 p-4 flex flex-col">
        <h1 class="text-3xl">{post.title}</h1>
        <div class="flex flex-row">
            <p>{post.tagline}</p>
            <p class="ml-auto">{post.published_at.toLocaleDateString(undefined, short_date_options)}</p>
        </div>
    </div>
</a>