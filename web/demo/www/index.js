import { polyfill } from "../../js-src/polyfills";

polyfill(["static-content"])

let sampleFileInput = document.getElementById("sample-file");
if (sampleFileInput) {
    sampleFileInput.addEventListener("change", sampleFileSelected, false);
}

let localFileInput = document.getElementById("local-file");
if (localFileInput) {
    localFileInput.addEventListener("change", localFileSelected, false);
}

if (window.location.search && window.location.search != "") {
    let urlParams = new URLSearchParams(window.location.search);
    let url = urlParams.get("file");
    if (url && url != "") {
        loadRemoteFile(url);
    }
}

function localFileSelected() {
    let file = localFileInput.files[0];
    if (file) {
        let fileReader = new FileReader();
        fileReader.onload = e => {
            playSwf(fileReader.result);
        }
        fileReader.readAsArrayBuffer(file);
    }
}

function sampleFileSelected() {
    if (sampleFileInput.selectedIndex <= 0) {
        // No SWF selected.
        return;
    }

    let file = sampleFileInput.selectedOptions[0].innerText;
    if (file) {
        loadRemoteFile(file);
    }
}

function loadRemoteFile(url) {
    fetch(url)
        .then(response => {
            response.arrayBuffer().then(data => playSwf(data))
        });
}

let timestamp = 0;
let animationHandler;

function playSwf(swfData) {
    document.getElementById("player").play_swf_data(swfData);
}