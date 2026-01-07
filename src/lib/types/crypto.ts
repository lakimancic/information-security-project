interface Key {
    label: string;
    keyHex: string;
    ivHex?: string;
}

interface CipherTag {
    label: string;
    value: string;
}

export type { Key, CipherTag };