interface Key {
	label: string;
	key: number[];
	iv?: number[];
}

interface CipherTag {
	label: string;
	value: string;
}

interface ShortKey {
	name: string;
	keySize: number;
	ivSize: number;
}

interface CryptoError {
	filename: string;
	err: string;
}

const streamCiphers = [
	{ value: 'stream:a5/1', label: 'A5/1' }];

const blockCiphers: CipherTag[] = [
	{ value: 'block:xtea', label: 'XTEA' },
	{ value: 'block:aes256', label: 'AES-256' }
];

const blockModes: CipherTag[] = [
	{ value: 'mode:ofb', label: 'OFB' }];

const hashModes: CipherTag[] = [
	{ value: 'blake256', label: 'BLAKE-256' }];

export type { Key, CipherTag, ShortKey, CryptoError };
export { streamCiphers, blockCiphers, blockModes, hashModes };
