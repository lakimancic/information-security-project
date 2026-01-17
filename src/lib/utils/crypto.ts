const sizesToAlgorithm = (keySize: number, ivSize: number) => {
    if (keySize === 32 && ivSize === 32) return ["AES256"];
    else if (keySize === 16 && ivSize === 8) return ["XTEA"];
    else if (keySize === 8 && ivSize === 0) return ["A5/1"];
    return []
};

export { sizesToAlgorithm };