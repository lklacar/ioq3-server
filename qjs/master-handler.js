const WebSocket = require('ws');

const HEARTBEAT_MESSAGE = "heartbeat QuakeArena-1";
const GET_INFO_MESSAGE = "infoResponse\n" +
    "\\game\\LK\\g_needpass\\0\\pure\\1\\gametype\\4\\sv_maxclients\\24\\g_humanplayers\\0\\clients\\0\\mapname\\q3dm17\\hostname\\^1L^2K ^3CTF ^5 ALL NEW MAPS\\protocol\\71\\gamename\\Quake3Arena\\challenge\\@{=6n!meb6";
const HEARTBEAT_DELAY = 60000;

function sendMessageToMaster(ws, message) {
    let messageData = [255, 255, 255, 255];
    messageData = messageData.concat(message.split('').map(i => i.charCodeAt(0)));
    const messageDataUint8 = new Uint8Array(messageData.length + 1);
    messageData.forEach((o, i) => {
        messageDataUint8[i] = o;
    });
    ws.send(messageDataUint8, {binary: true});
}

export default function handleMaster(masterServerUrl) {
    const ws = new WebSocket(masterServerUrl);

    ws.onopen = function () {
        (function execute() {
            sendMessageToMaster(ws, HEARTBEAT_MESSAGE);
            setTimeout(execute, HEARTBEAT_DELAY);
        })();

        ws.onmessage = function (msg) {
            sendMessageToMaster(ws, GET_INFO_MESSAGE)
        };
    };
}