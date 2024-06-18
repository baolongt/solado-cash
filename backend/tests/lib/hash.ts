import crypto from 'crypto';

export function hashString(s: string): number[] {
    const hash = crypto.createHash('sha256');
    hash.update(s);
    const digest = hash.digest();
    return Array.from(digest);
}