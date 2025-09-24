import { selectInputStreamDevice, selectInputStreamDeviceIndex } from "./menus";
import { getTrackList } from "./tracks";


await selectInputStreamDeviceIndex(0);
console.log("Selected stream");
await selectInputStreamDevice();
console.log("clicked ok");
let tracks = await getTrackList();
console.log(tracks.tracks);