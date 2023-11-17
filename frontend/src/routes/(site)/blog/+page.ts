import {fetchAll} from "$lib/api/blog_posts";

/** @type {import('./+page.svelte').PageLoad} */
export async function load({ fetch }: any) {
    return {
        posts: await fetchAll(fetch)
    };
}