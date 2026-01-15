interface Key {
    label: string;
    key: number[];
    iv?: number[];
}

interface CipherTag {
    label: string;
    value: string;
}

const streamCiphers = [
    { value: "stream:a5/1", label: "A5/1" }
];

const blockCiphers : CipherTag[] = [
    { value: "block:xtea", label: "XTEA" },
    { value: "block:aes256", label: "AES-256" }
];

const blockModes : CipherTag[] = [
    { value: "mode:ofb", label: "OFB" }
];

export type { Key, CipherTag };
export { streamCiphers, blockCiphers, blockModes };