function start() {
    const qualitySlider = document.getElementById('quality-slider');
    qualitySlider.addEventListener('input', () => {
        document.querySelectorAll('.quality.shown').forEach((e) => {
            e.classList.remove('shown')
        });
        document.getElementById('quality-' + qualitySlider.value).classList.add('shown');
    });
    const url = document.getElementById('the_url');
    let urlPlaceholder = url.getAttribute('placeholder')
    url.addEventListener('focus', () => {
        url.setAttribute('placeholder', '');
    });
    url.addEventListener('blur', () => {
        url.setAttribute('placeholder', urlPlaceholder);
    });
    const dlPlaylist = document.getElementById('dl-playlist');
    url.addEventListener('input', () => {
        if (url.value.match(/list=/)) {
            dlPlaylist.classList.remove('hidden');
        } else {
            dlPlaylist.classList.add('hidden');
        }
    });
}

document.addEventListener('DOMContentLoaded', start, false);