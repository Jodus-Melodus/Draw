import { getTrackList, updateTrack } from "./backend/tracks";

async function init() {
  const muteButton = document.querySelector('.channel-mute, ,track-mute');
  const soloButton = document.querySelector('.channel-solo, .track-solo');
  const recordButton = document.querySelector('.channel-record, .track-record');
  const monitorButton = document.querySelector('.channel-monitor, .track-monitor');

  if (muteButton) {
    muteButton.addEventListener('click', () => {
      muteButton.classList.add('active');
      soloButton?.classList.remove('active');
      soloButton?.classList.add('inactive');

    });
  }

  if (soloButton) {
    soloButton.addEventListener('click', () => {
      soloButton.classList.add('active');
      muteButton?.classList.remove('active');
      muteButton?.classList.add('inactive');
    });
  }

  if (recordButton) {
    recordButton.addEventListener('click', () => {
      recordButton.classList.toggle('active');
    });
  }

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