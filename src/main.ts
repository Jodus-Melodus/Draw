import { getTrackList, updateTrack } from "./backend/tracks";

async function init() {
    let track_list = await getTrackList();
    console.log(track_list.tracks);
    await updateTrack("master-out", { Gain: 0.0 });
    let track_list2 = await getTrackList();
    console.log(track_list2.tracks);
}

init();

const muteButton = document.querySelector('.mute');

if (muteButton) {
  muteButton.addEventListener('click', () => {
    muteButton.classList.toggle('active');
  });
}