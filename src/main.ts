import { getTrackList, updateTrack } from "./backend/tracks";

let track_list = await getTrackList();
console.log(track_list.tracks);
await updateTrack("master-out", { Volume: 0.0 });
let track_list2 = await getTrackList();
console.log(track_list2.tracks);