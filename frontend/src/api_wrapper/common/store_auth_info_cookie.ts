import { getCookie, removeCookie, setCookie } from 'typescript-cookie';

export function storeAuthInfoCookie(accesstoken: string, refreshtoken: string, lastLogin: Date) {
    setCookie('accesstoken', accesstoken);
    setCookie('refreshtoken', refreshtoken);
    setCookie('lastLogin', lastLogin.toString());
}

export function removeAuthInfo() {
    removeCookie('accesstoken');
    removeCookie('refreshtoken');
    removeCookie('lastLogin');
}

export function getAccessToken(): string | undefined {
    return getCookie('accesstoken');
}

export function getRefreshToken(): string | undefined {
    return getCookie('refreshtoken');
}

export function getLastLogin(): Date | undefined {
    let date = getCookie('lastLogin');
    if (!date) {
        return undefined;
    } else {
        return new Date(date);
    }
}
