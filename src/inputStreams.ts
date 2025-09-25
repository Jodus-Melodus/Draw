import { getInputStreamDeviceList, selectInputStreamDeviceIndex, selectInputStreamDevice } from "./backend/menus";
import { getTrackList } from "./backend/tracks";

let list = await getInputStreamDeviceList();
console.log(list);

await selectInputStreamDeviceIndex(0);
console.log("Selected stream");
await selectInputStreamDevice();
console.log("clicked ok");
let tracks = await getTrackList();
console.log(tracks.tracks);