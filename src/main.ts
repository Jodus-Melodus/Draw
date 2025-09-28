import { getTrackList, updateTrack } from "./backend/tracks";

async function init() {
  const muteButton = document.querySelector('.channel-mute');

  if (muteButton) {
    muteButton.addEventListener('click', () => {
      muteButton.classList.toggle('active');
    });
  }

  const soloButton = document.querySelector('.channel-solo');

  if (soloButton) {
    soloButton.addEventListener('click', () => {
      soloButton.classList.toggle('active');
    });
  }

  const recordButton = document.querySelector('.channel-record');

  if (recordButton) {
    recordButton.addEventListener('click', () => {
      recordButton.classList.toggle('active');
    });
  }

  const monitorButton = document.querySelector('.channel-monitor');

  if (monitorButton) {
    monitorButton.addEventListener('click', () => {
      monitorButton.classList.toggle('active');
    });
  }

  let track_list = await getTrackList();
  console.log(track_list.tracks);
  await updateTrack("master-out", { Gain: 0.0 });
  let track_list2 = await getTrackList();
  console.log(track_list2.tracks);
}

init(); 