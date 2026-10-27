#ifndef TRACER_H
#define TRACER_H
#include <stdint.h>
#define WIDTH 640
#define HEIGHT 400
extern float pixels[WIDTH * HEIGHT][4];
extern uint8_t surfaces[WIDTH * HEIGHT];
extern uint8_t reflections[WIDTH * HEIGHT];
void trace_scene(void);
#endif
