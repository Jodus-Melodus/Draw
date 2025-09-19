const { invoke } = window.__TAURI__.core;

export async function callRust() {
    try {
        const message = await invoke("add_track", {});
        const div = document.createElement("div");
        div.textContent = message;
        document.body.append(div);
    } catch (error) {
        console.log("Error calling Rust:", error);
    }
}