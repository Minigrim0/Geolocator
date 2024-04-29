<template>
    <div id="mapContainer"></div>
</template>

<script>
import "leaflet/dist/leaflet.css";
import L from "leaflet";

export default {
    name: "Map",
    props: {
        coords: {
            type: Object,
            required: false,
        },
    },
    data() {
        return {
            map: null,
            marker: null
        };
    },
    watch: {
        coords(new_coords, _) {
            if(this.marker != null) {
                this.map.removeLayer(this.marker);
            }

            if (new_coords) {
                this.marker = new L.marker(new_coords)
                this.marker.addTo(this.map);
            }
        }
    },
    methods: {
        init: function () {
            this.map = L.map(
                "mapContainer", {
                    maxBounds: L.latLngBounds(L.latLng(-90, -180), L.latLng(90, 180))
                })
                .setView([0, 0], 2);
            L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
                attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
                maxZoom: 18,
                minZoom: 2
            }).addTo(this.map);
            this.map.on('click', (e) => this.$emit('map-click', e.latlng));
        },
    },
    mounted() {
        this.init();
    }
}
</script>