<!-- src/components/LocationInput.vue -->
<template>
    <div>
        <input v-model="location" type="text" placeholder="請輸入地點（例如：台北車站）" />
        <button @click="submitLocation">搜尋美食</button>
    </div>
</template>

<script>
export default {
    data() {
        return {
            location: ''
        }
    },
    methods: {
        submitLocation() {
            if (this.location.trim() === '') {
                alert('請輸入地點')
                return
            }
            this.$emit('location-submitted', this.location)
        }
    }
}
</script>

<style scoped>
input {
    padding: 8px;
    width: 200px;
    margin-right: 10px;
}

button {
    padding: 8px 12px;
}
</style>