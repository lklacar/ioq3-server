const handleMaster = require("./master-handler");

const Buffer = require('buffer').Buffer;
const dgram = require('dgram');
const WebSocketServer = require('ws').Server;

const LOCAL_SERVER_IP = '127.0.0.1';
const LOCAL_SERVER_PORT = 27960;
const MASTER_SERVER_URL = "ws://master.quakejs.com:27950/";
const wss = new WebSocketServer({port: 27961});


handleMaster(MASTER_SERVER_URL);

wss.on('connection', function (ws) {
    try {
        const udpClient = dgram.createSocket('udp4');
        udpClient.on('message', function (msg, rinfo) {
            try {
                ws.send(msg);
            } catch (e) {
                console.error("ws.send(msg)")
            }
        });

        ws.on('message', function (message) {
            const msgBuff = new Buffer(message);
            try {
                udpClient.send(msgBuff, 0, msgBuff.length, LOCAL_SERVER_PORT, LOCAL_SERVER_IP);
            } catch (e) {
                console.error("udpClient.send")
            }
        });
    } catch (e) {
        console.error(e);
    }
});
