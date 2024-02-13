use std::{f32::consts::PI, vec};

use arrow::ast::Statement;
use arrow::core::*;
use arrow::eval::*;
use arrow::march::render;
use arrow::sdf::*;
use glam::{Affine3A, Vec2, Vec3};
use std::collections::HashMap;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

#[allow(dead_code)]
fn displacement(p: Vec3) -> f32 {
    let freq = 10.0;
    (p.x * freq).sin() * (p.y * freq).sin() * (p.z * freq).sin() * 0.1
}

#[allow(dead_code)]
fn scene0() -> Sdf {
    let noise = Noise::new(1, 0.25, 0.6);
    let displacement = move |p: Vec3| noise.get(p);
    let sphere_gold = perturb(sd_sphere(1.0, v3(-1.0, 0.0, 0.0), I), displacement);
    let sphere_red = sd_sphere(0.75, v3(1.0, 0.0, 0.0), I);
    let floor = sd_plane(v3(0.05, 1.0, 0.0), 1.0, I);

    let mut tr = Affine3A::from_rotation_y(0.5);
    tr = tr * Affine3A::from_rotation_x(-0.35);
    let cube = sd_round_box(v(0.6), 0.05, v3(1.0, 0.0, 0.0), tr);

    let tr = Affine3A::from_rotation_x(-0.5);
    let torus = sd_torus(0.7, 0.15, v3(0.1, 0.0, -0.2), tr);

    let capsule = sd_capsule(
        0.15,
        v3(-0.5, 1.9, 0.0),
        v3(-1.0, 0.0, 0.0),
        v3(1.0, 0.0, 0.0),
        I,
    );

    let rounded_cube = round(sd_box(v3(0.4, 0.3, 0.0), v3(-1.9, 1.9, 0.0), I), 0.2);

    fn ninety(p: Vec3) -> Vec3 {
        let mut tr = Affine3A::from_rotation_z(-PI / 2.0);
        tr = tr * Affine3A::from_translation(v3(2.2, -1.5, -0.5));
        tr.transform_point3(p)
    }

    let hammer = map_xyz(smooth_union(rounded_cube, capsule, 0.4), ninety);

    let rep_ball = repeat_y(sd_sphere(0.07, v3(3.0, 0.0, -0.175), I), 0.25);

    let mut balls = Vec::new();
    for i in 0..10 {
        balls.push(sd_sphere(0.07, v3(-0.9 + i as f32 * 0.2, -0.9, -1.5), I));
    }

    let frame = difference(cube, sphere_red);

    let cylinder = sd_cylinder(
        0.1,
        v3(-0.2, 0.0, -0.2),
        v3(1.75, -0.25, -1.0),
        v3(1.25, 1.0, -1.5),
        I,
    );

    let inf_cylinder = mirror_x(sd_inf_cylinder(0.1, Vec2::new(2.9, 0.0), I));

    let a = unions(vec![
        sphere_gold,
        floor,
        torus,
        hammer,
        frame,
        cylinder,
        inf_cylinder,
        rep_ball,
    ]);
    let b = unions(balls);
    union(a, b)
}

#[allow(dead_code)]
fn scene1() -> Sdf {
    let plane = sd_plane(v3(0.05, 1.0, 0.0), 1.0, I);
    union(plane, sd_sphere(1.0, ZERO3, I))
}

fn main() {
    use arrow::parser::*;
    let background = 0.75;
    // let mut input = "U(L(x+28,y-10,z+8)-12, don(x-cl(x,-15,15),y-18,z-20,10,3), bx3(x-20,y-20,z+20,8)-5, L(x+3,y-16)-2)";
    // let mut half_rings = "don(x,y-3,mod(z,8)-4,8,1)";
    // let mut donut = "don(x,y-2,z,5,1)";
    // let mut rods = "L(B(B(x)-3)-3,B(y)-3)-2";
    // let mut rounded_box = "bx3(x,y-5,z-5,7,4,4)-5";
    // let mut elbow = "L(k(x,y-10),z)-5";
    // let mut fence = "L(x,TR(y))-.5";
    // let mut cross =
    //     "U( bx3(mod(x,4)-2,y,z,6), bx3(x,y,mod(x,4)-2,6), L(TR(x),y)-1, L(x+20,y-20,z-20)-8)";
    let mut input = "[x,z]=r0(x-20,z), bx3(x,mod(y,1)-.5,mod(z,1)-.5,.45)";
    // let mut input = "r=B(x,y,z,4,3)-4, s=1; @4{ @xyz{$=(mod($+9,18)-9)*3,}, s/=3, r=k(r,-U(@xyz{bx2($,$$,9),})*s),}r";
    // let mut balls8a = "@xyz{$=B($)-6,} L(x,y,z)-5";
    // let mut balls8b = "L(B(x)-6, B(y)-6, B(z)-6) - 5";
    // let mut box_of_balls = "s=1; @5{@xyz{$=B($*2)-8,} s*=.5,} (L(x,y,z)-8)*s";
    // let mut input = "s=10,[x,z]=r0(x,z),[y,z]=r1(z,y),[y,x]=r0(y,x),@xyz{$m=mod($,1)-.5,}b=bx3(xm,ym,zm,.45)-.05,t=[0,2,3,1],i=1,n=(a=i++)=>nz(z,x,y,.01,a,a==1?2:1)*t[a]*100,@yxz{$+=n(),}@xz{$b=mod($,s*2)-s,}rG(b,bx2(bx2(xb,zb,s),TR((y+2)/40)*40,1,2.2)-.2,.3)-.1";
    // let mut input= "d=99,l=10, x-=l*2, y-=l,z+=2.5, @3{ x+=l, a=a0*($+2),s=sin(a),c=cos(a), [x1,y1]=rot(x,y,s,c), a=a1*($+2),s=sin(a),c=cos(a), [x1,z1]=rot(x1,z,s,c), d=rU(d, bx3(x1,y1,z1,4),3), } U(d+.5, L(nz(x,y,z,.5,1,6)-.5, abs(d)-.1)-.4)";
    // let mut  apollonius = "s=2.5,h=s/2,d=(s+h)/2,q=20,y-=10,[x,y]=r0(x,y),@xyz{$/=q,}c=1,t=0,@7{@xyz{$=mod($-h,s)-h,}t=d/D([x,y,z],[x,y,z]),@xyzc{$*=t,}}d=L(x,y,z)/c*2.-.025";
    let mut rot_cube = "[a,b]=r0(x,y-9); bx3(a,b,z,4)-.5";
    let ast = program(&mut rot_cube).unwrap();
    dbg!(&ast);
    let sdf: Sdf = Box::new(move |p| make_sdf(&ast, 0.125 / 2.0, 0.0, p));
    println!("sdf: {}", sdf(v3(0.0, 0.0, -100.0)));
    // let plane = sd_plane(v3(0.0, 0.85, 0.3), 10.0, I);
    // let sdf = union(sdf, plane);
    let img_data = render(
        &sdf,
        v3(0.0, 0.0, -30.0),
        ZERO3,
        // v3(5.0, 15.0, -30.0),
        // v3(-5.0, -5.0, 0.0),
        &vec![
            Light::new(v3(0.0, 0.0, -26.0), 1.0),
            // Light::new(v3(-2.0, 5.0, -6.0), 0.6),
            Light::new(v3(5.0, 10.0, -6.0), 0.3),
        ],
        background,
        WIDTH,
        HEIGHT,
        1,
    );
    image::save_buffer("hatch.png", &img_data, WIDTH, HEIGHT, image::ColorType::L8).unwrap();
}
