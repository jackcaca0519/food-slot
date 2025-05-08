<!-- src/App.vue -->
<template>
  <div class="container">
    <h1>🍽️ 今天吃什麼？</h1>
    <LocationInput @location-submitted="handleLocation" />
    <div>{{ output }}</div>
    <p>目前取得的餐廳數：{{ restaurants.length }}</p>
    <SpinWheel v-if="restaurants.length" :items="restaurants" />
  </div>
</template>

<script>
import { getCoordinates } from './api/getCoordinates.js'
import { invoke } from "@tauri-apps/api/core"
import LocationInput from './components/LocationInput.vue'
import SpinWheel from './components/SpinWheel.vue'

export default {
  data() {
        return {
            output: '',
            restaurants: [],
        }
  },
  components: {
    LocationInput,
    SpinWheel
  },
  methods: {
    async handleLocation(location) {
      this.output = '正在查詢...'
      const coords = await getCoordinates(location)
      if (coords.lng && coords.lat) {
        this.output = `位置: ${location}, 經度：${coords.lng}, 緯度：${coords.lat}`;
        try {
          const results = await invoke('get_restaurants', {
            location: coords
          })
          console.log('從 Rust 得到的餐廳：', results)
          this.restaurants = results
        } catch (err) {
          console.error('Rust 後端錯誤：', err)
          this.output = `查詢餐廳失敗：${err}`
        }
      } else if (coords) {
        this.output = coords;
      } else {
        this.output = '無法找到該地點，請再試一次';
      }
    }
  }
}
</script>

<style>
.container {
  text-align: center;
  margin-top: 100px;
}
</style>
