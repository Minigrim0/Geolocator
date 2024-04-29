
<template>
    <div class="container">
        <h1 class="title">Geolocator</h1>
        <small><button @click="load_folder()">Load folder</button></small>
        <div v-if="current_image" class="top-pic">
            <img :src="current_image.name" class="fit-content" />
        </div>
        <div class="confirm-button">
            <button @click="confirm()">Confirm</button>
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
            if (this.images.length > 0) {
                return this.images[0];
            } else {
                return null;
            }
        },
        current_image_coordinates() {
            if (this.current_image) {
                return this.current_image.coordinates;
            } else {
                return null;
            }
        }
    },
    methods: {
        load_folder: function () {
            dialog
                .open({directory: true, multiple: true})
                .then((result) => {
                    if (result == null) {
                        return;
                    }
                    for (const path of result) {
                        invoke("tauri", {cmd: "open", path: path})
                        .then((files) => {
                            files.forEach(element => {
                                element.name = convertFileSrc(element.name);
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
        setPicturePosition: function (e) {
            if (this.images.length == 0) {
                return;
            }

            this.images[0].coordinates = e;
        },
        floatToDMS: function (D) {
            return [
                0 | (D < 0 ? (D = -D) : D),
                0 | (((D += 1e-9) % 1) * 60),
                (0 | (((D * 60) % 1) * 6000)) / 100
            ];
        },
        confirm() {
            invoke("tauri", {cmd: "write", file: {
                name: this.images[0].path,
                coordinates: {
                    latitude: this.floatToDMS(this.images[0].coordinates.lat),
                    longitude: this.floatToDMS(this.images[0].coordinates.lng)
                }
            }})
            .then((result) => {
                console.log(result);
            })
            .catch((err) => {
                console.error(err);
            });
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

.confirm-button {
    position: absolute;
    bottom: 5px;
    right: 5px;
    z-index: 1000;
    background-color: rgba(128, 255, 128, 0.5);
}
</style>
