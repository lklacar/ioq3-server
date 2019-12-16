
//open a log file
void Log_Open(char *filename);

//close the current log file
void Log_Close(void);

//close log file if present
void Log_Shutdown(void);

//write to the current opened log file
void QDECL Log_Write(char *fmt, ...) __attribute__ ((format (printf, 1, 2)));

//write to the current opened log file with a time stamp
void QDECL Log_WriteTimeStamped(char *fmt, ...) __attribute__ ((format (printf, 1, 2)));

//returns a pointer to the log file
FILE *Log_FilePointer(void);

//flush log file
void Log_Flush(void);

