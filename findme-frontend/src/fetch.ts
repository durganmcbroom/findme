export const SERVER = "http://localhost:8000"

export interface PostStub {
    post_id: number;
    name: string;
    image: string;
    description: string;
    age: number;
    last_seen: number;
    last_location: LatLon;
    disaster_type: DisasterType;
}

export interface PostIn {
    name: string;
    image: string;
    description: string;
    age: number;
    last_seen: number;
    disaster_type: string;
    last_location: LatLon;
}


interface LatLon {
    lat: number;
    lon: number;
}

export enum DisasterType {
    CarCrash
    // Define your enum values here, matching the Rust enum
}


export async function fetch_stubs(lat: number, lon: number): Promise<PostStub[]> {
    let f = await fetch(SERVER + `/posts/stubs?start=0&end=100&lat=${lat}&lon=${lon}`, {});

    return f.json()
}

export function image_url(path: string): string {
    console.log(path)
    return SERVER + "/posts/media/" + path
}

export async function submit_image(file: File): Promise<string> {
    const formData = new FormData()
    formData.append("file", file)
    formData.append("extension", file.name.substring(file.name.indexOf(".") + 1, file.name.length))

    return (await fetch(SERVER + `/posts/media/`, {
        method: "POST",
        body: formData
    })).text()
}

export async function submit_report(post: PostIn) {
    console.log(JSON.stringify(post))
    await fetch(SERVER + "/posts", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(post)
    })
}

export interface Post {
    id: number;
    name: string;
    image: string; // Assuming you want to store the path as a string
    description: string;
    age: number;
    last_seen: number;
    disaster_type: DisasterType;
    last_location: LatLon;
    comments: PostComment[];
    status: PostStatus;
}

export enum PostStatus {
    NotFound,
    Injured = 1, // You can specify values for specific enum variants
    Found,
}

export enum PostCommentType {
    Extra,
    Informational,
    Concrete,
}

export interface PostComment {
    author: string;
    value: string;
    status: string;
}

export async function get_post(id: number): Promise<Post> {
    return (await fetch(SERVER + `/posts/${id}`)).json()
}

export async function submit_comment(postId: number, value: PostComment) {
    console.log(value)

    await fetch(SERVER + `/posts/comment?post=${postId}`, {
        method: "PUT",
        body: JSON.stringify(value)
    })
}