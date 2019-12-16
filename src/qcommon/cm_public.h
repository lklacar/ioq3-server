
#include "qfiles.h"


void CM_LoadMap(const char *name, qboolean clientload, int *checksum);

void CM_ClearMap(void);

clipHandle_t CM_InlineModel(int index);        // 0 = world, 1 + are bmodels
clipHandle_t CM_TempBoxModel(const vec3_t mins, const vec3_t maxs, int capsule);

void CM_ModelBounds(clipHandle_t model, vec3_t mins, vec3_t maxs);

int CM_NumClusters(void);

int CM_NumInlineModels(void);

char *CM_EntityString(void);

// returns an ORed contents mask
int CM_PointContents(const vec3_t p, clipHandle_t model);

int CM_TransformedPointContents(const vec3_t p, clipHandle_t model, const vec3_t origin, const vec3_t angles);

void CM_BoxTrace(trace_t *results, const vec3_t start, const vec3_t end,
                 vec3_t mins, vec3_t maxs,
                 clipHandle_t model, int brushmask, int capsule);

void CM_TransformedBoxTrace(trace_t *results, const vec3_t start, const vec3_t end,
                            vec3_t mins, vec3_t maxs,
                            clipHandle_t model, int brushmask,
                            const vec3_t origin, const vec3_t angles, int capsule);

byte *CM_ClusterPVS(int cluster);

int CM_PointLeafnum(const vec3_t p);

// only returns non-solid leafs
// overflow if return listsize and if *lastLeaf != list[listsize-1]
int CM_BoxLeafnums(const vec3_t mins, const vec3_t maxs, int *list,
                   int listsize, int *lastLeaf);

int CM_LeafCluster(int leafnum);

int CM_LeafArea(int leafnum);

void CM_AdjustAreaPortalState(int area1, int area2, qboolean open);

qboolean CM_AreasConnected(int area1, int area2);

int CM_WriteAreaBits(byte *buffer, int area);

// cm_patch.c
void CM_DrawDebugSurface(void (*drawPoly)(int color, int numPoints, float *points));
