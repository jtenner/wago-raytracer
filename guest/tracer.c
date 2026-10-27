#include "tracer.h"

// A small deterministic Whitted tracer. All intersections and lighting run in Wasm.
typedef struct { float x, y, z; } V;
typedef struct { V center; float radius; V color; int material; } Sphere;
typedef struct { float t; V p, n, color; int material, front; } Hit;
enum { SKY, GROUND, MATTE, METAL, GLASS };
static const Sphere spheres[] = {
    {{-1.75f, .85f, 0}, .85f, {.78f, .16f, .075f}, MATTE},
    {{0, .85f, .15f}, .85f, {.97f, .99f, 1}, GLASS},
    {{1.8f, .9f, -.15f}, .9f, {.91f, .78f, .51f}, METAL}
};
float pixels[WIDTH * HEIGHT][4];
uint8_t surfaces[WIDTH * HEIGHT], reflections[WIDTH * HEIGHT];
static uint32_t rng;
static V v(float x, float y, float z) { return (V){x,y,z}; }
static V add(V a,V b) { return v(a.x+b.x,a.y+b.y,a.z+b.z); }
static V sub(V a,V b) { return v(a.x-b.x,a.y-b.y,a.z-b.z); }
static V mul(V a,float b) { return v(a.x*b,a.y*b,a.z*b); }
static V had(V a,V b) { return v(a.x*b.x,a.y*b.y,a.z*b.z); }
static float dot(V a,V b) { return a.x*b.x+a.y*b.y+a.z*b.z; }
static V cross(V a,V b) { return v(a.y*b.z-a.z*b.y,a.z*b.x-a.x*b.z,a.x*b.y-a.y*b.x); }
static float root(float x) { return __builtin_sqrtf(x); }
static V unit(V a) { return mul(a,1/root(dot(a,a))); }
static float max(float a,float b) { return a>b?a:b; }
static float min(float a,float b) { return a<b?a:b; }
static V reflect(V d,V n) { return sub(d,mul(n,2*dot(d,n))); }
static float random01(void) { rng^=rng<<13; rng^=rng>>17; rng^=rng<<5; return (rng>>8)*(1.f/16777216.f); }
static V sky(V d) { float t=.5f*(d.y+1); return add(mul(v(.91f,.94f,.98f),1-t),mul(v(.22f,.49f,.83f),t)); }

static Hit intersect(V o,V d,float limit) {
    Hit h={.t=limit,.material=SKY};
    if (d.y < -.0001f) {
        float t=-o.y/d.y;
        if(t>.001f && t<h.t) {
            h.t=t; h.material=GROUND; h.n=v(0,1,0); h.front=1;
            h.p=add(o,mul(d,t));
            // Broad, quiet tiles make the metal's reflected ground easy to see.
            int ix=(int)__builtin_floorf(h.p.x), iz=(int)__builtin_floorf(h.p.z);
            h.color=((ix+iz)&1)?v(.32f,.40f,.33f):v(.58f,.62f,.49f);
        }
    }
    for(unsigned i=0;i<sizeof(spheres)/sizeof(spheres[0]);i++) {
        Sphere s=spheres[i]; V oc=sub(o,s.center);
        float b=dot(oc,d), c=dot(oc,oc)-s.radius*s.radius, disc=b*b-c;
        if(disc<0) continue;
        float t=-b-root(disc); if(t<=.001f)t=-b+root(disc);
        if(t<=.001f || t>=h.t)continue;
        h.t=t; h.p=add(o,mul(d,t)); V outward=mul(sub(h.p,s.center),1/s.radius);
        h.front=dot(d,outward)<0; h.n=h.front?outward:mul(outward,-1);
        h.material=s.material; h.color=s.color;
    }
    return h;
}

static V radiance(V o,V d) {
    V weight=v(1,1,1);
    for(int bounce=0;bounce<8;bounce++) {
        Hit h=intersect(o,d,10000);
        if(h.material==SKY)return had(weight,sky(d));
        if(h.material==METAL) {
            weight=had(weight,h.color); d=reflect(d,h.n);
        } else if(h.material==GLASS) {
            float eta=h.front?1/1.5f:1.5f, co=min(-dot(d,h.n),1), si2=1-co*co;
            float f=1-co, fresnel=.04f+.96f*f*f*f*f*f;
            if(eta*eta*si2>1 || random01()<fresnel)d=reflect(d,h.n);
            else d=add(mul(add(d,mul(h.n,co)),eta),mul(h.n,-root(max(0,1-eta*eta*si2))));
            weight=had(weight,h.color);
        } else {
            V light=v(-3+2*random01(),5.5f,3+2*random01());
            V delta=sub(light,h.p); float distance=root(dot(delta,delta)); V ld=mul(delta,1/distance);
            Hit blocker=intersect(add(h.p,mul(h.n,.003f)),ld,distance);
            float visible=blocker.material==SKY?1:(blocker.material==GLASS?.48f:0);
            float direct=max(0,dot(h.n,ld))*visible;
            V illumination=add(mul(sky(h.n),.24f),mul(v(1.05f,.98f,.84f),direct));
            return had(weight,had(h.color,illumination));
        }
        o=add(h.p,mul(d,.003f));
    }
    return v(0,0,0);
}

void trace_scene(void) {
    V eye=v(4.4f,2.65f,7.5f), target=v(0,.85f,0);
    V forward=unit(sub(target,eye)), right=unit(cross(forward,v(0,1,0))), up=cross(right,forward);
    float scale=.36f;
    for(int y=0;y<HEIGHT;y++)for(int x=0;x<WIDTH;x++) {
        unsigned i=y*WIDTH+x; rng=0x1234567u+i*747796405u; if(!rng)rng=1;
        V sum=v(0,0,0);
        for(int s=0;s<32;s++) {
            float u=(2*(x+random01())/WIDTH-1)*scale*WIDTH/HEIGHT;
            float w=(1-2*(y+random01())/HEIGHT)*scale;
            V d=unit(add(forward,add(mul(right,u),mul(up,w))));
            sum=add(sum,radiance(eye,d));
        }
        sum=mul(sum,1/32.f);
        pixels[i][0]=sum.x; pixels[i][1]=sum.y; pixels[i][2]=sum.z; pixels[i][3]=1;
        V d=unit(add(forward,add(mul(right,(2*(x+.5f)/WIDTH-1)*scale*WIDTH/HEIGHT),mul(up,(1-2*(y+.5f)/HEIGHT)*scale))));
        Hit h=intersect(eye,d,10000); surfaces[i]=h.material;
        if(h.material==METAL) {
            V rd=reflect(d,h.n);
            reflections[i]=intersect(add(h.p,mul(rd,.003f)),rd,10000).material;
        }
    }
}
