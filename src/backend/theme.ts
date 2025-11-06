export const THEMES = ["light", "dark", "red", "green", "blue"];

type BodyTheme = {
    background: string;
    text: string;

    tracklist: TrackListTheme;
    mixconsole: MixConsoleTheme;
    track: TrackTheme
    channel: ChannelTheme
};

type TrackListTheme = {
    background: string;
};

type MixConsoleTheme = {
    background: string;
};

type TrackTheme = {
    background: string;
    text: string;
    name: {
        text: string;
    };
    mute: {
        active: {
            background1: string;
            background2: string;
            text: string;
        };
        inactive: {
            background1: string;
            background2: string;
            text: string;
        }
    };
    solo: {
        active: {
            background1: string;
            background2: string;
            text: string;
        };
        inactive: {
            background1: string;
            background2: string;
            text: string;
        }
    };
    monitor: {
        active: {
            background1: string;
            background2: string;
            text: string;
        };
        inactive: {
            background1: string;
            background2: string;
            text: string;
        }
    };
    record: {
        active: {
            background1: string;
            background2: string;
            text: string;
        };
        inactive: {
            background1: string;
            background2: string;
            text: string;
        }
    };
};

type ChannelTheme = {
    background1: string;
    background2: string;
    text: string;
    meters: {
        gain: {
            background1: string;
            background2: string;
            background3: string;
        }
    };
    gain: {
        background: string;
        text: string;
    };
    pan: {
        background1: string;
        background2: string;
        background3: string;
        text: string;
    };
    fader: {
        background: string;
        thumb: {
            background1: string;
            background2: string;
            body: string;
            notch: string;
        };
        gain: {
            background: string;
            text: string;
        },
        slit: {
            background: string;
        }
    };
    name: {
        text: string;
    };
    mute: {
        active: {
            background1: string;
            background2: string;
            text: string;
        };
        inactive: {
            background1: string;
            background2: string;
            text: string;
        }
    };
    solo: {
        active: {
            background1: string;
            background2: string;
            text: string;
        };
        inactive: {
            background1: string;
            background2: string;
            text: string;
        }
    };
    monitor: {
        active: {
            background1: string;
            background2: string;
            text: string;
        };
        inactive: {
            background1: string;
            background2: string;
            text: string;
        }
    };
    record: {
        active: {
            background1: string;
            background2: string;
            text: string;
        };
        inactive: {
            background1: string;
            background2: string;
            text: string;
        }
    }
}


export async function loadTheme(modeIndex: number | string) {
    // Accept either a numeric index or a theme-name string. If a string is
    // provided and it's numeric ("2"), parse it. If it's a name ("dark"),
    // find its index in THEMES. Normalize to a valid index with wrap-around.
    if (THEMES.length === 0) {
        console.warn("No themes available to load");
        return;
    }

    let idx: number | null = null;

    if (typeof modeIndex === "number") {
        if (Number.isInteger(modeIndex)) idx = modeIndex;
    } else if (typeof modeIndex === "string") {
        // Try numeric string first
        const asNum = Number(modeIndex);
        if (!Number.isNaN(asNum) && Number.isInteger(asNum)) {
            idx = asNum;
        } else {
            // Treat as theme name
            const found = THEMES.indexOf(modeIndex);
            if (found !== -1) idx = found;
        }
    }

    if (idx === null) {
        console.warn("loadTheme called with invalid index", modeIndex);
        return;
    }

    const normalized = ((idx % THEMES.length) + THEMES.length) % THEMES.length;
    const mode = THEMES[normalized];
    try {
        // Use dynamic import for loading theme files
        const theme = await import(`../themes/${mode}.json`);
        applyTheme(theme.default || theme);
    } catch (err) {
        console.error("Error loading theme", mode, err);
    }
}

export function applyTheme(theme: BodyTheme) {
    const root = document.documentElement.style;

    function walk(obj: any, path: string[]) {
        for (const key in obj) {
            const value = obj[key];
            const newPath = [...path, key];

            if (typeof value === "object" && value !== null) {
                walk(value, newPath);
            } else {
                const varName = "--" + newPath.join("-");
                root.setProperty(varName, String(value));
            }
        }
    }

    walk(theme, []);
}