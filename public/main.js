async function init() {
    const rustApp = await import('../pkg')
    if (!rustApp) return

    const input = document.getElementById('upload')
    const reader = new FileReader()
    const imageElement = document.getElementById('new-img')

    reader.onloadend = () => {
        const base64 = reader.result
            .replace(/^data:image\/(png|jpeg|jpg);base64,/, '')
        const processedImage = rustApp.grayscale(base64)
        imageElement.setAttribute('src', processedImage)
    }

    input.addEventListener('change', () => {
        reader.readAsDataURL(input.files[0])
    })
}

init().catch(error => console.log(error))