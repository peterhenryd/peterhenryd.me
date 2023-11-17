import {api_endpoint, type Fetch} from "$lib/api/index";

export interface BlogPost {
    id: number,
    title: String,
    tagline: String,
    content: String,
    created_at: Date,
    published_at?: Date,
    edited_at?: Date,
}

function jsonToBlogPost(json: any): BlogPost {
    json.created_at = new Date(json.created_at);
    if (json.published_at) json.published_at = new Date(json.published_at);
    if (json.edited_at) json.edited_at = new Date(json.edited_at);

    return json;
}
export function fetchAll(fetch: Fetch): Promise<BlogPost[]> {
    return fetch(api_endpoint + "/blog-posts/all")
        .then(res => res.json())
        .then((array: any[]) => array.map(jsonToBlogPost));
}

export function fetchById(fetch: Fetch, id: number): Promise<BlogPost> {
    return fetch(api_endpoint + `/blog-posts/id/${id}`)
        .then(res => res.json())
        .then(jsonToBlogPost);
}