use core::ptr::addr_of_mut;

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 400;
pub const COUNT: usize = WIDTH * HEIGHT;
pub static mut PIXELS: [[f32; 4]; COUNT] = [[0.; 4]; COUNT];
pub static mut SURFACES: [u8; COUNT] = [0; COUNT];
pub static mut REFLECTIONS: [u8; COUNT] = [0; COUNT];

#[derive(Clone, Copy, Default)]
pub struct V { pub x: f32, pub y: f32, pub z: f32 }
pub const fn v(x: f32, y: f32, z: f32) -> V { V { x, y, z } }
impl V {
    pub fn add(self, b: V) -> V { v(self.x+b.x,self.y+b.y,self.z+b.z) }
    pub fn sub(self, b: V) -> V { v(self.x-b.x,self.y-b.y,self.z-b.z) }
    pub fn mul(self, b: f32) -> V { v(self.x*b,self.y*b,self.z*b) }
    pub fn had(self, b: V) -> V { v(self.x*b.x,self.y*b.y,self.z*b.z) }
    pub fn dot(self, b: V) -> f32 { self.x*b.x+self.y*b.y+self.z*b.z }
    pub fn cross(self, b: V) -> V { v(self.y*b.z-self.z*b.y,self.z*b.x-self.x*b.z,self.x*b.y-self.y*b.x) }
    pub fn unit(self) -> V { self.mul(1./libm::sqrtf(self.dot(self))) }
    pub fn reflect(self, n: V) -> V { self.sub(n.mul(2.*self.dot(n))) }
}
pub struct Rng(pub u32);
impl Rng {
    pub fn next(&mut self) -> f32 { self.0^=self.0<<13; self.0^=self.0>>17; self.0^=self.0<<5; (self.0>>8) as f32*(1./16777216.) }
}
pub const SKY: u8 = 0;
pub const GROUND: u8 = 1;
pub const MATTE: u8 = 2;
pub const METAL: u8 = 3;
pub const GLASS: u8 = 4;
#[derive(Clone, Copy)]
struct Sphere { center: V, radius: f32, color: V, material: u8 }
const SPHERES: [Sphere; 3] = [
    Sphere { center:v(-1.75,0.85,0.),radius:0.85,color:v(0.78,0.16,0.075),material:MATTE },
    Sphere { center:v(0.,0.85,0.15),radius:0.85,color:v(0.97,0.99,1.),material:GLASS },
    Sphere { center:v(1.8,0.9,-0.15),radius:0.9,color:v(0.91,0.78,0.51),material:METAL },
];
pub struct Hit { pub t:f32, pub p:V, pub n:V, pub color:V, pub material:u8, pub front:bool }
#[inline(never)]
fn sky(d: V) -> V { let t=0.5*(d.y+1.); v(0.91,0.94,0.98).mul(1.-t).add(v(0.22,0.49,0.83).mul(t)) }
#[inline(never)]
fn intersect(o: V, d: V, limit: f32) -> Hit {
    let mut h=Hit{t:limit,material:SKY,p:V::default(),n:V::default(),color:V::default(),front:true};
    if d.y < -0.0001 {
        let t=-o.y/d.y;
        if t>0.001 && t<h.t {
            h.t=t; h.material=GROUND; h.n=v(0.,1.,0.); h.p=o.add(d.mul(t));
            let ix=libm::floorf(h.p.x) as i32; let iz=libm::floorf(h.p.z) as i32;
            h.color=if (ix+iz)&1 !=0 {v(0.32,0.40,0.33)}else{v(0.58,0.62,0.49)};
        }
    }
    for s in SPHERES {
        let oc=o.sub(s.center); let b=oc.dot(d); let disc=b*b-oc.dot(oc)+s.radius*s.radius;
        if disc<0. {continue;}
        let mut t=-b-libm::sqrtf(disc); if t<=0.001 { t=-b+libm::sqrtf(disc); }
        if t<=0.001 || t>=h.t {continue;}
        h.t=t; h.p=o.add(d.mul(t)); let outward=h.p.sub(s.center).mul(1./s.radius);
        h.front=d.dot(outward)<0.; h.n=if h.front {outward}else{outward.mul(-1.)};
        h.material=s.material; h.color=s.color;
    }
    h
}
#[inline(never)]
fn radiance(mut o: V, mut d: V, rng: &mut Rng) -> V {
    let mut weight=v(1.,1.,1.);
    for _ in 0..8 {
        let h=intersect(o,d,10000.);
        match h.material {
            SKY => return weight.had(sky(d)),
            METAL => { weight=weight.had(h.color); d=d.reflect(h.n); },
            GLASS => {
                let eta=if h.front {1./1.5}else{1.5}; let co=(-d.dot(h.n)).min(1.); let si2=1.-co*co;
                let f=1.-co; let fresnel=0.04+0.96*f*f*f*f*f;
                d=if eta*eta*si2>1. || rng.next()<fresnel {d.reflect(h.n)}else{
                    d.add(h.n.mul(co)).mul(eta).add(h.n.mul(-libm::sqrtf((1.-eta*eta*si2).max(0.))))
                };
                weight=weight.had(h.color);
            },
            _ => {
                let light=v(-3.+2.*rng.next(),5.5,3.+2.*rng.next());
                let delta=light.sub(h.p); let distance=libm::sqrtf(delta.dot(delta)); let ld=delta.mul(1./distance);
                let blocker=intersect(h.p.add(h.n.mul(0.003)),ld,distance);
                let visible=match blocker.material {SKY=>1.,GLASS=>0.48,_=>0.};
                let direct=h.n.dot(ld).max(0.)*visible;
                let illumination=sky(h.n).mul(0.24).add(v(1.05,0.98,0.84).mul(direct));
                return weight.had(h.color.had(illumination));
            }
        }
        o=h.p.add(d.mul(0.003));
    }
    v(0.,0.,0.)
}

pub unsafe fn postcard() {
    let eye=v(4.4,2.65,7.5); let target=v(0.,0.85,0.);
    let forward=target.sub(eye).unit(); let right=forward.cross(v(0.,1.,0.)).unit(); let up=right.cross(forward);
    let ray=|x:f32,y:f32| forward.add(right.mul((2.*x/WIDTH as f32-1.)*0.36*WIDTH as f32/HEIGHT as f32))
        .add(up.mul((1.-2.*y/HEIGHT as f32)*0.36)).unit();
    let pixels=&mut *addr_of_mut!(PIXELS); let surfaces=&mut *addr_of_mut!(SURFACES); let reflections=&mut *addr_of_mut!(REFLECTIONS);
    for y in 0..HEIGHT { for x in 0..WIDTH {
        let i=y*WIDTH+x; let seed=0x1234567u32.wrapping_add((i as u32).wrapping_mul(747796405));
        let mut rng=Rng(seed.max(1)); let mut sum=V::default();
        for _ in 0..32 { let d=ray(x as f32+rng.next(),y as f32+rng.next()); sum=sum.add(radiance(eye,d,&mut rng)); }
        sum=sum.mul(1./32.); pixels[i]=[sum.x,sum.y,sum.z,1.];
        let d=ray(x as f32+0.5,y as f32+0.5); let h=intersect(eye,d,10000.); surfaces[i]=h.material;
        if h.material==METAL { let rd=d.reflect(h.n); reflections[i]=intersect(h.p.add(rd.mul(0.003)),rd,10000.).material; }
    } }
}
