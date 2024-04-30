
<template>
    <div class="container">
        <h1 class="title">Geolocator</h1>
        <small><button @click="load_folder()">Load folder</button></small>
        <div v-if="current_image" class="top-pic">
            <img :src="current_image.display_path" class="fit-content" />
        </div>
        <div class="control-buttons" v-if="current_image !== null">
            <button v-if="current_image_coordinates !== null" @click="confirm()">Confirm</button>
            <button @click="skip()">Next</button>
        </div>
        <div id="container" class="interactive-map">
            <Map :coords="current_image_coordinates" @map-click="(e) => setPicturePosition(e)" />
        </div>
    </div>
</template>

<script>
import { dialog, invoke } from "@tauri-apps/api";
import { convertFileSrc } from '@tauri-apps/api/tauri';

import Map from "./components/map.vue";

export default {
    name: "App",
    components: {
        Map
    },
    data() {
        return {
            images: [],
        };
    },
    computed: {
        current_image() {
            return (this.images.length > 0) ? this.images[0] : null;
        },
        current_image_coordinates() {
            return (this.current_image) ? this.current_image.coordinates: null;
        }
    },
    methods: {
        load_folder: function () {
            dialog
                .open({directory: true, multiple: true})
                .then((result) => {
                    if (result == null) return;
                    for (const path of result) {
                        invoke("tauri", {cmd: "open", path: path})
                            .then((files) => {
                                files.forEach(element => {
                                    element.display_path = convertFileSrc(element.path);
                                    if (element.coordinates) element.coordinates = this.DMStoFloat(element.coordinates);
                                });
                                this.images.push(...files);
                            })
                            .catch((err) => {
                                console.error(err);
                            });
                    }
                })
                .catch((err) => {
                    console.error(err);
                });
        },
        DMStoFloat(coords) {
            console.log("Converting DMS to decimal:", coords);
            return L.latLng(
                (coords.latitude[0] == "S" ? -1 : 1) * coords.latitude[1] + coords.latitude[2] / 60 + coords.latitude[3] / 3600,
                (coords.longitude[0] == "W" ? -1 : 1) * coords.longitude[1] + coords.longitude[2] / 60 + coords.longitude[3] / 3600
            );
        },
        FloatToDMS: function (D, long) {
            return [
                long ? (D < 0 ? 'W' : 'E') : (D < 0 ? 'S' : 'N'),
                (0 | Math.abs((D < 0 ? (D = -D) : D))),
                (0 | Math.abs((((D += 1e-9) % 1) * 60))),
                (0 | Math.abs((((D * 60) % 1) * 6000)) / 100),
            ];
        },
        setPicturePosition: function (e) {
            if (this.images.length == 0) {
                return;
            }

            this.images[0].coordinates = e;
        },
        confirm() {
            let file = {
                path: this.images[0].path,
                coordinates: {
                    latitude: this.FloatToDMS(this.images[0].coordinates.lat, false),
                    longitude: this.FloatToDMS(this.images[0].coordinates.lng, true)
                }
            }
            invoke("tauri", {cmd: "save_image", file: file})
                .then((result) => {
                    console.log(result);
                    this.images.shift();
                })
                .catch((err) => {
                    console.error(err);
                });
        },
        skip() {
            this.images.shift();
        }
    },
};
</script>

<style scoped>
body {
  margin: 0 !important;
}

#mapContainer {
  width: 100%;
  height: 100%;
}

.top-pic {
    position: absolute;
    top: 5px;
    right: 5px;
    z-index: 1000;
    max-width: 10vw;
    max-height: 10vh;
}

.fit-content {
    width: 100%;
    height: auto;
}

.title {
    font-size: 2em;
    margin-bottom: 0;
    color: black;
    position: absolute;
    top: 5px;
    left: 5px;
    z-index: 1000;
    font-family: Arial, sans-serif;
}

.interactive-map {
    position: absolute;
    top: 0;
    left: 0;
    z-index: -1;
    width: 100vw;
    height: 100vh;
}

.control-buttons {
    position: absolute;
    bottom: 5px;
    right: 5px;
    z-index: 1000;
}
</style>
