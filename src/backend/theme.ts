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
        background: string;
        text: string;
    };
    solo: {
        background: string;
        text: string;
    };
    monitor: {
        background: string;
        text: string;
    };
    record: {
        background: string;
        text: string;
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
        background: string;
        text: string;
    };
    solo: {
        background: string;
        text: string;
    };
    monitor: {
        background: string;
        text: string;
    };
    record: {
        background: string;
        text: string;
    }
}


export async function loadTheme(mode: "light" | "dark") {
    const response = await fetch(`themes/${mode}.json`);
    const theme = await response.json();
    applyTheme(theme);
}

export function applyTheme(theme: BodyTheme) {
    document.body.style.backgroundColor = theme.background;
    document.body.style.color = theme.text;

    const buttons = document.querySelectorAll<HTMLButtonElement>("button");
    buttons.forEach(button => {
        button.style.color = theme.text;
    })
}