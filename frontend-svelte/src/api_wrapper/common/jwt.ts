export function getUserIdFromAccessToken(accesstoken: string): string {
    const base64EncryptedPayload = accesstoken.split('.')[1];

    return JSON.parse(window.atob(base64EncryptedPayload)).id;
}
