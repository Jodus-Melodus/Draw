export function percentToGain(p: number) {
    const minGain = 0.001; // effectively silence
    const maxGain = 1.0;   // full volume

    // map percent to logarithmic gain
    return minGain * Math.pow(maxGain / minGain, p / 100);
}

export function percentToDb(p: number) {
    const minDb = -60; // -60 dB = silent
    const maxDb = 0;   // 0 dB = full volume
    return minDb + (maxDb - minDb) * (p / 100);
}

export function isValidTrackName(str: string): boolean {
    return /^[A-Za-z0-9_-]+$/.test(str);
}

export function replaceHyphensWithSpaces(str: string): string {
    return str.replace("-", " ");
}

export function replaceSpacesWithHyphens(str: string): string {
    return str.replace(" ", "-");
}