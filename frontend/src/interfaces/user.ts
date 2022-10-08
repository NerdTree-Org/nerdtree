export interface User {
    id: string;
    username: string;
    firstname: string;
    lastname: string;
    email: string;
    profile_pic: string | null;
    is_admin: boolean;
    facebook_id: string;
    discord_token: string;
    has_used_discord_token: boolean;
    joined_date: string;
}
