import type { RequestResult } from '../common/request';
import { ENV } from '../../env';
import { getAccessToken } from '../common/store_auth_info_cookie';

const GET_DISCORD_TOKEN_ROUTE = 'user/get/discord_token';

export async function getDiscordToken(): Promise<RequestResult<string>> {
    let req;
    try {
        req = await fetch(`${ENV.api_address}/${GET_DISCORD_TOKEN_ROUTE}`, {
            method: 'POST',
            headers: {
                authorization: `bearer ${getAccessToken()}`
            }
        });
    } catch (e) {
        return {
            success: false,
            value: null,
            error: 'Failed to connect to the server'
        };
    }

    return {
        success: true,
        error: '',
        value: await req.text()
    };
}
