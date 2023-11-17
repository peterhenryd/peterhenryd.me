import {fetchById} from "$lib/api/blog_posts";

/** @type {import('../../../../../.svelte-kit/types/src/routes').PageLoad} */
export async function load({ fetch, params }: any) {
    return {
        post: await fetchById(fetch, params.id)
    }
}