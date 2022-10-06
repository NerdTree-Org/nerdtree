export interface Post {
    id: string;
    is_approved: string;
    title: string;
    thumbnail: string | null;
    body: string;
    creation_date: string;
    approval_date: string | null;
    post_author: string | null;
}
