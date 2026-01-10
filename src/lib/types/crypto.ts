interface Key {
    label: string;
    key: number[];
    iv?: number[];
}

interface CipherTag {
    label: string;
    value: string;
}

export type { Key, CipherTag };