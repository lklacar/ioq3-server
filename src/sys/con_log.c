
#include "../qcommon/q_shared.h"
#include "../qcommon/qcommon.h"
#include "sys_local.h"

#define MAX_LOG 32768

static char consoleLog[MAX_LOG];
static unsigned int writePos = 0;
static unsigned int readPos = 0;

/*
==================
CON_LogSize
==================
*/
unsigned int CON_LogSize(void) {
    if (readPos <= writePos)
        return writePos - readPos;
    else
        return writePos + MAX_LOG - readPos;
}

/*
==================
CON_LogFree
==================
*/
static unsigned int CON_LogFree(void) {
    return MAX_LOG - CON_LogSize() - 1;
}

/*
==================
CON_LogWrite
==================
*/
unsigned int CON_LogWrite(const char *in) {
    unsigned int length = strlen(in);
    unsigned int firstChunk;
    unsigned int secondChunk;

    while (CON_LogFree() < length && CON_LogSize() > 0) {
        // Free enough space
        while (consoleLog[readPos] != '\n' && CON_LogSize() > 1)
            readPos = (readPos + 1) % MAX_LOG;

        // Skip past the '\n'
        readPos = (readPos + 1) % MAX_LOG;
    }

    if (CON_LogFree() < length)
        return 0;

    if (writePos + length > MAX_LOG) {
        firstChunk = MAX_LOG - writePos;
        secondChunk = length - firstChunk;
    } else {
        firstChunk = length;
        secondChunk = 0;
    }

    Com_Memcpy(consoleLog + writePos, in, firstChunk);
    Com_Memcpy(consoleLog, in + firstChunk, secondChunk);

    writePos = (writePos + length) % MAX_LOG;

    return length;
}

/*
==================
CON_LogRead
==================
*/
unsigned int CON_LogRead(char *out, unsigned int outSize) {
    unsigned int firstChunk;
    unsigned int secondChunk;

    if (CON_LogSize() < outSize)
        outSize = CON_LogSize();

    if (readPos + outSize > MAX_LOG) {
        firstChunk = MAX_LOG - readPos;
        secondChunk = outSize - firstChunk;
    } else {
        firstChunk = outSize;
        secondChunk = 0;
    }

    Com_Memcpy(out, consoleLog + readPos, firstChunk);
    Com_Memcpy(out + firstChunk, consoleLog, secondChunk);

    readPos = (readPos + outSize) % MAX_LOG;

    return outSize;
}
