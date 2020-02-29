import * as wasm from "wasm-frames";

const constraints = {
    audio: false,
    video: {
        facingMode: ["user", "environment"],
    }
};

function init() {
    const sourceCanvas = document.getElementById('source');
    const sourceCtx = sourceCanvas.getContext('2d');

    const targetCtx = document.getElementById('target').getContext('2d');

    navigator.mediaDevices.getUserMedia(constraints).then(
        mediaStream => {

            console.log("acquired");
            // let vidEl = $('video');
            // vidEl[0].srcObject = mediaStream;

            const track = mediaStream.getVideoTracks()[0];
            const imageCapture = new ImageCapture(track);

            let interval = null;

            interval = setInterval(function() {
                if (!track.muted) {
                    imageCapture.grabFrame().then(frame => {

                        sourceCanvas.width = frame.width;
                        sourceCanvas.height = frame.height;
                        sourceCtx.drawImage(frame, 0, 0);
                        let r = wasm.add_one(3);

                        //wasm.process_image(
                        //    sourceCtx,
                        //    targetCtx,
                        //    frame.width,
                        //    frame.height
                        //);
                        //const imageData = sourceCtx.getImageData(0, 0, sourceCanvas.width, sourceCanvas.height);

                    }).catch(err => {
                        clearInterval(interval);
                        console.error('grabFrame() failed: ', err)
                    });
                }
            }, 80);

        },
        error => {
            alert('Could not acquire media: ' + error);
        }); //.catch( error => {})
}

init();
