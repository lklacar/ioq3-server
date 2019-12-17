var Buffer = require('buffer').Buffer;
var dgram = require('dgram');
var WebSocketServer = require('ws').Server;
var WebSocket = require('ws');
var wss = new WebSocketServer({port: 27961});
var SERVER_IP = '127.0.0.1';
var SERVER_PORT = 27960;


const ws = new WebSocket('ws://localhost:27950/');

ws.on('open', function open() {
    let a = [255, 255, 255, 255];

    a = a.concat("infoResponse\n".split('').map(i => i.charCodeAt(0)));

    const b = new Uint8Array(a.length + 1);

    a.forEach((o, i) => {
        b[i] = o;
    });

    ws.send(b, {binary: true});
});


wss.on('connection', function (error) {
    console.log("error", error);
});

wss.on('connection', function (ws) {
    try {

        console.log("on connection....", ws);
        var udpClient = dgram.createSocket('udp4');

        udpClient.on('message', function (msg, rinfo) {
            try {
                ws.send(msg);
            } catch (e) {
                console.log("ws.send(msg)")
            }
        });

        ws.on('message', function (message) {
            var msgBuff = new Buffer(message);
            try {
                udpClient.send(msgBuff, 0, msgBuff.length, SERVER_PORT, SERVER_IP);
            } catch (e) {
                console.log("udpClient.send")
            }
        });
    } catch (e) {
        console.log(e);
    }
});