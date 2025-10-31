type BodyTheme = {
    background: string;
    text: string;

    track: TrackTheme
    channel: ChannelTheme
};

type TrackTheme = {
    background: string;
    text: string;
    name: {
        background: string;
        text: string;
    };
    mute: {
        active: {
            background: string;
            text: string;
        };
        inactive: {
            background: string;
            text: string;
        }
    };
    solo: {
        active: {
            background: string;
            text: string;
        };
        inactive: {
            background: string;
            text: string;
        }
    };
    monitor: {
        active: {
            background: string;
            text: string;
        };
        inactive: {
            background: string;
            text: string;
        }
    };
    record: {
        active: {
            background: string;
            text: string;
        };
        inactive: {
            background: string;
            text: string;
        }
    };
};

type ChannelTheme = {
    background: string;
    text: string;
    meters: {
        right: {
            background: string
        };
        left: {
            background: string
        };
    };
    gain: {
        background: string;
        text: string;
    };
    pan: {
        background: string;
        text: string;
    };
    fader: {
        background: string;
        thumb: {
            background: string;
            body: string;
            notch: string;
        };
        gain: {
            background: string;
            text: string;
        }
    };
    name: {
        background: string;
        text: string;
    };
    mute: {
        active: {
            background: string;
            text: string;
        };
        inactive: {
            background: string;
            text: string;
        }
    };
    solo: {
        active: {
            background: string;
            text: string;
        };
        inactive: {
            background: string;
            text: string;
        }
    };
    monitor: {
        active: {
            background: string;
            text: string;
        };
        inactive: {
            background: string;
            text: string;
        }
    };
    record: {
        active: {
            background: string;
            text: string;
        };
        inactive: {
            background: string;
            text: string;
        }
    }
}


export async function loadTheme(mode: "light" | "dark") {
    const response = await fetch(`themes/${mode}.json`);
    const theme = await response.json();
    applyTheme(theme);
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
                const varName = "--" + newPath.join("-"); // e.g. --channel-fader-gain-text
                console.log(varName);
                
                root.setProperty(varName, String(value));
            }
        }
    }

    walk(theme, []);
}