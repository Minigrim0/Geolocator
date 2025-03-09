<template>
    <div class="container">
        <h1 class="title">Geolocator</h1>
        <small><button @click="load_folder()">Load folder</button></small>
        <div v-if="current_image" class="top-pic">
            <img :src="current_image.display_path" class="fit-content" />
            <p>{{ this.index + 1 }} / {{ this.images.length }}</p>
        </div>
        <div class="control-buttons" v-if="current_image !== null">
            <button @click="previous()">Previous</button>
            <button
                v-if="current_image_coordinates !== null"
                @click="confirm()"
            >
                Confirm
            </button>
            <button @click="next()">Next</button>
        </div>
        <div id="container" class="interactive-map">
            <Map
                :coords="current_image_coordinates"
                @map-click="(e) => setPicturePosition(e)"
            />
        </div>
    </div>
</template>

<script>
import { invoke, convertFileSrc } from "@tauri-apps/api/core";

import Map from "./components/map.vue";
import * as dialog from "@tauri-apps/plugin-dialog";

export default {
    name: "App",
    components: {
        Map,
    },
    data() {
        return {
            images: [],
            index: 0,
        };
    },
    computed: {
        current_image() {
            return this.images.length > 0 && this.index < this.images.length
                ? this.images[this.index]
                : null;
        },
        current_image_coordinates() {
            return this.current_image ? this.current_image.coordinates : null;
        },
        images_left() {
            return this.images.length - this.index;
        },
    },
    methods: {
        load_folder: function () {
            dialog
                .open({ directory: true, multiple: true })
                .then((result) => {
                    if (result == null) return;
                    for (const path of result) {
                        invoke("tauri", { cmd: "open", path: path })
                            .then((files) => {
                                files.forEach((element) => {
                                    element.display_path = convertFileSrc(
                                        element.path,
                                    );
                                    if (element.coordinates)
                                        element.coordinates = this.DMStoFloat(
                                            element.coordinates,
                                        );
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
            return L.latLng(
                (coords.latitude[0] == "S" ? -1 : 1) * coords.latitude[1] +
                    coords.latitude[2] / 60 +
                    coords.latitude[3] / 3600,
                (coords.longitude[0] == "W" ? -1 : 1) * coords.longitude[1] +
                    coords.longitude[2] / 60 +
                    coords.longitude[3] / 3600,
            );
        },
        FloatToDMS: function (D, long) {
            return [
                long ? (D < 0 ? "W" : "E") : D < 0 ? "S" : "N",
                0 | Math.abs(D < 0 ? (D = -D) : D),
                0 | Math.abs(((D += 1e-9) % 1) * 60),
                0 | (Math.abs(((D * 60) % 1) * 6000) / 100),
            ];
        },
        setPicturePosition: function (e) {
            if (this.images.length == 0) {
                return;
            }

            this.images[this.index].coordinates = e;
        },
        updateIndex(offset) {
            this.index += offset;
            if (this.index < 0) {
                this.index = this.images.length - 1;
            } else if (this.index >= this.images.length) {
                this.index = 0;
            }
        },
        confirm() {
            let file = {
                path: this.images[this.index].path,
                coordinates: {
                    latitude: this.FloatToDMS(
                        this.images[this.index].coordinates.lat,
                        false,
                    ),
                    longitude: this.FloatToDMS(
                        this.images[this.index].coordinates.lng,
                        true,
                    ),
                },
            };
            invoke("tauri", { cmd: "save_image", file: file })
                .then((_) => {
                    this.updateIndex(1);
                })
                .catch((err) => {
                    // TODO; Handle
                    console.error(err);
                });
        },
        next() {
            this.updateIndex(1);
        },
        previous() {
            this.updateIndex(-1);
        },
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
    color: black;
    font-family: Arial, sans-serif;
    position: absolute;
    top: 5px;
    right: 5px;
    z-index: 1000;
    max-width: 15vw;
    max-height: 15vh;
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
