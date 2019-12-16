
#ifdef AASINTERN

extern aas_t aasworld;

//AAS error message
void QDECL AAS_Error(char *fmt, ...) __attribute__ ((format (printf, 1, 2)));
//set AAS initialized
void AAS_SetInitialized(void);
//setup AAS with the given number of entities and clients
int AAS_Setup(void);
//shutdown AAS
void AAS_Shutdown(void);
//start a new map
int AAS_LoadMap(const char *mapname);
//start a new time frame
int AAS_StartFrame(float time);
#endif //AASINTERN

//returns true if AAS is initialized
int AAS_Initialized(void);

//returns true if the AAS file is loaded
int AAS_Loaded(void);

//returns the current time
float AAS_Time(void);

//
void AAS_ProjectPointOntoVector(vec3_t point, vec3_t vStart, vec3_t vEnd, vec3_t vProj);
