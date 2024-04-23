<script>
// var map = L.map('map').setView([51.505, -0.09], 13);

// L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
//     attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
// }).addTo(map);

// L.marker([51.5, -0.09]).addTo(map)
//     .bindPopup('A pretty CSS popup.<br> Easily customizable.')
//     .openPopup();


import "leaflet/dist/leaflet.css";
import L from "leaflet";


export default {
  name: "App",
  methods: {
    setupLeafletMap: function () {
      const mapDiv = L.map("mapContainer").setView([0, 0], 2);
      L.tileLayer(
        "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
        {
          attribution:
            '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
          maxZoom: 18
        }
      ).addTo(mapDiv);

      function onMapClick(e) {
    console.log("You clicked the map at " + e.latlng);
}

mapDiv.on('click', onMapClick);
    },
  },
  mounted() {
    this.setupLeafletMap();
  },
};
</script>

<template>
  <div class="container">
    <h1>Geolocator</h1>
    <small><button>Load folder</button></small>
    <div id="container">
      <div id="mapContainer"></div>
    </div>
  </div>
</template>

<style scoped>
body {
  margin: 0;
}

#mapContainer {
  width: 100vw;
  height: 80vh;
}
</style>
