const API_KEY = import.meta.env.VITE_GOOGLE_API_KEY

export async function getCoordinates(address) {
  const url = `https://maps.googleapis.com/maps/api/geocode/json?address=${encodeURIComponent(address)}&key=${API_KEY}`

  try {
    const res = await fetch(url)
    const data = await res.json()

    if (data.status === 'OK') {
      const location = data.results[0].geometry.location
      return location // { lat: ..., lng: ... }
    } else {
      throw new Error('找不到該地點')
    }
  } catch (err) {
    console.error('地理編碼失敗：', err)
    return err
  }
}
