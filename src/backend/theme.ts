const THEMES = ["dark", "light", "green"];

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


export async function loadTheme(mode: string) {
    if (THEMES.includes(mode)) {
        const response = await fetch(`themes/${mode}.json`);
        const theme = await response.json();
        applyTheme(theme);
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