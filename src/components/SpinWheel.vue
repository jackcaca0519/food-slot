<!-- src/components/SpinWheel.vue -->
<template>
    <div class="wheel-container">
      <LuckyWheel
        ref="myLucky"
        width="50vw"
        height="50vw"
        :prizes="prizes"
        :blocks="blocks"
        :buttons="buttons"
        @start="startCallback"
        @end="endCallback"
      />
      <p v-if="selectedItem">
        🎉 選中：
        <span
          @click="openInWebview(selectedItem)"
          style="color: #007aff; text-decoration: underline; cursor: pointer"
        >
          {{ selectedItem.name }}
        </span>
      </p>
    </div>
  </template>
  
  <script>
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
  
  export default {
    name: 'SpinWheel',
    props: ['items'],
    data() {
      return {
        selectedItem: null,
        blocks: [{ padding: '7px', background: '#617df2' }],
        prizes: [],
        buttons: [
          { radius: '25%', background: '#617df2' },
          { radius: '22%', background: '#afc8ff' },
          {
            radius: '20%',
            background: '#8a9bf3',
            pointer: true,
            fonts: [{ text: '🎲', top: '-18px', fontSize: '30px', fontColor: 'white' }]
          }
        ]
      }
    },
    watch: {
      items: {
        immediate: true,
        handler(newItems) {
          this.prizes = newItems.map((item, index) => {
            return {
              fonts: [{ text: item.name, top: '10%', fontSize: '12px', lineClamp: 6 }],
              background: index % 2 === 0 ? '#e9e8fe' : '#b8c5f2'
            }
          })
        }
      }
    },
    methods: {
      startCallback() {
        if (!this.items.length) return
        const index = Math.floor(Math.random() * this.items.length)
        this.$refs.myLucky.play(4, 'ease-in-out')
        setTimeout(() => {
          this.$refs.myLucky.stop(index)
        }, 3000)
      },
      endCallback(prize) {
        const index = this.prizes.findIndex(p => p.fonts[0].text === prize.fonts[0].text)
        this.selectedItem = this.items[index]
      },
      openInWebview(item) {
        const url = `https://www.google.com/maps/search/?api=1&query=${encodeURIComponent(item.name + ' ' + item.address)}`
        const label = `map_${Date.now()}`
        const webview = new WebviewWindow(label, {
          url,
          title: item.name,
          width: 800,
          height: 600,
          visible: true,
          resizable: true
        })
        webview.once('tauri://error', e => {
          console.error('❌ Webview failed to load:', e)
        })
      }
    }
  }
  </script>
  
  <style scoped>
  .wheel-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 30px;
  }
  </style>
  