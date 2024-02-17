use arrow::core::*;
use arrow::eval::*;
use arrow::march::render;

const S: u32 = 4;
const WIDTH: u32 = 1024 / S;
const HEIGHT: u32 = 768 / S;

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
    // let mut input = "[x,z]=r0(x-20,z); bx3(x,mod(y,1)-.5,mod(z,1)-.5,1.15)";
    // let mut input = "r=B(x,y,z,4,3)-4, s=1; @4{ @xyz{$=(mod($+9,18)-9)*3,}, s/=3, r=k(r,-U(@xyz{bx2($,$$,9),})*s),}r";
    // let mut balls8a = "@xyz{$=B($)-6,} L(x,y,z)-5";
    // let mut balls8b = "L(B(x)-6, B(y)-6, B(z)-6) - 5";
    // let mut box_of_balls =
    //     "s=1; @5{ [x,y]=r0(x,y), [x,z]=r1(x,z), @xyz{$=B($*2)-8,} s*=.5; } (L(x,y,z)-8)*s";
    // let mut input = "s=10,[x,z]=r0(x,z),[y,z]=r1(z,y),[y,x]=r0(y,x),@xyz{$m=mod($,1)-.5,}b=bx3(xm,ym,zm,.45)-.05,t=[0,2,3,1],i=1,n=(a=i++)=>nz(z,x,y,.01,a,a==1?2:1)*t[a]*100,@yxz{$+=n(),}@xz{$b=mod($,s*2)-s,}rG(b,bx2(bx2(xb,zb,s),TR((y+2)/40)*40,1,2.2)-.2,.3)-.1";
    // let mut input= "d=99,l=10, x-=l*2, y-=l,z+=2.5, @3{ x+=l, a=a0*($+2),s=sin(a),c=cos(a), [x1,y1]=rot(x,y,s,c), a=a1*($+2),s=sin(a),c=cos(a), [x1,z1]=rot(x1,z,s,c), d=rU(d, bx3(x1,y1,z1,4),3), } U(d+.5, L(nz(x,y,z,.5,1,6)-.5, abs(d)-.1)-.4)";
    // let mut  apollonius = "s=2.5,h=s/2,d=(s+h)/2,q=20,y-=10,[x,y]=r0(x,y),@xyz{$/=q,}c=1,t=0,@7{@xyz{$=mod($-h,s)-h,}t=d/D([x,y,z],[x,y,z]),@xyzc{$*=t,}}d=L(x,y,z)/c*2.-.025";
    // let mut rot_cube = "[a,b]=r0(x,y-9); bx3(a,b,z,4)-.5";
    // let mut hyperplane = "a=(2*x-3*z+6*y)/7,b=(7*x-2*z+26*y)/27,c=(6*z-3*x+22*y)/23,d=0,zz=0;[x,z]=r1(x,z+8),[x,y]=r0(x,y),y-=3,zz=FR(z/26-.55)*26-13,d=SM(9,-12,y+3-z*.3),U(k(k(k(bx3(x,y-5,zz,7,14,7)-1,@abc{d-B(TR($))),}L(x+99,y+445,z+32)-434)";
    // let mut desire = "[x,y]=r0(x,y-1), [x,z]=r1(x,z), yb=B(y)-22.5, U(rG(32-k(0-y-13,z-15),TR(x*.25)*4-2+4*SM(0,16,x),4),rG( B(B(L(L(x,z)-16,yb-cl(yb,-8.5,8.5))-8)-4)-2, B(B(L(B(x)-15,B(B(y)-15)-15,B(z)-15)-9)-4)-2,1 ))";
    // let mut singularity = "[x,z]=r0(x,z),l=L(x,y,z),n=2.0 * nz(atan2(z,x),Math.acos(y/l),l,.3,0,1),d=l-20+n*5,d=B(d)-5,d=B(d)-1,b=99,@4{b=U(b,bx2(x,y-n*3+10-$*10,100,2+$*.5)),}rU(y+20-B(n)*.2,k(0-b,d)-.4,20)";
    // let mut gnarl = "p=B(y-18)-13,n=nz(x,y,z,.2,0,2)*2,q=mod(p,12+n*z)-1.8";
    // let mut ondu = "s=.5,y+=6, a=k(y+22,B(z+10*g(x*.005+.2))-16)-4,b=TR(x/40+.2)*40,c; [b,y]=r1(b,y), [y,z]=r0(y,z+15), r=rU(U(@byz{bx2(B($$$)-20,bx2($,$$,23)+3,3)-.4,}),a,3), @4{ [x,y]=r1(x,y), [y,z]=r0(y,z), a=nz(x,y,z,.02/s,$+5), a=B(a)*50-3, r=rU(r,rG(r-7*s,a*s,s*2),s*2), s*=.5, } r";
    // let mut source = "@zy{$f=$+nz(x,y,z,.03,1)*40,} zf+=nz(x,y,z,.1,2)*20, f=L(zf+10,yf)-20, w=y+nz(x,y,z+30,.4,2,1)*2.2+1, g=y+nz(x,y,z,0.1,1), p=min(max(g,-f),max(w,f))+nz(x,y,z,.02,2,2)*4, [ex,ey]=r0(x-18,y-6), [ey,ez]=r1(ey,z-11), es=2, e=bx3(ex,ey,ez,es*1.5,es,.1), sg=.5, @xyz{$g=e$+nz(ex/1.6,ey,ez,1,1,2)*2,} g=1e6, @xy{g=U(g,L(ez-.1,mod($g, sg)-sg/2)-sg*.07),} g=max(g,bx3(ex,ey,ez,es*1.5*.85,es*.85,2)), min(p, e, g, bx3(ex+3,B(ey)-.8,ez, 4,.1,.1), bx3(ex+3,ey,ez, .05, es*1.2,.5),bx3(ex-3,ey,ez, .05, es*.2,.2))";
    // let mut spheres = "y-=5,z-=3, r=L(x,y,z), ph=atan2(y,x), th=Math.acos(z/r), n=18,r0=r-33, cs=sin(n*ph)*cos(n*th), c0=L(r0,cs)-.1, c3=L(r0-cl(r0,-2,-1),r/n*(cs-.5))-.05, c4=L(r0-cl(r0,-1,.5),r/n*(cs-.75))-.05, c5=L(r0-cl(r0,-1,1),r/n*(cs-.95))-.025, n=4,r1=1.25*n*sin(th), x=r0, y=r/n*sin(th)*cos(n*ph)*sin(n*th), z=r/n*sin(th)*sin(n*ph)*cos(n*th), c1=L(x,y,z)-r1, r=L(x,y,z), ph=atan2(y,x), th=Math.acos(z/r), zr=r-r1-cl(r-r1,0,.5), n=12, x=r/n*(sin(th)*cos(n*ph)*sin(n*th)-.5), c2=L(x,zr)-.05, U(c0,c1,c2,c3,c4,c5)";
    let mut arctic = "[x,z]=r0(x,z), x+=11, z+=15, y+=10, h=exp(-1.5*B(nz(x,0,z,.1,1))), g=y-10*h-nz(x,0,z,10,1)*0.05, b = y-12, a=rU( L(x-cl(x,-2,2),b*1.3,z)-3, U( L(x+5,b-1,z)-1.7, L(x+5,b-2,B(z)-1.5)-0.8, bx3(x-5,b-1,z,0.2,0.1,0.2)-0.5, bx3(x+5,b-1,z,1.9,.1,.1)-.5, L(B(x)-3.5,b-cl(b,-4,0),B(z)-1.5)-.8,),1.5 )-nz(x,0,z,12,1)*0.15, s=(L(x>7?(mod(x,4)-2)/2:x,x<1?y:b/3+2,B(z)-1.5)-1.8)-nz(x,y,z,.5,1)*2, rG(U(a,g),-s,1)";
    let ast = program(&mut arctic).unwrap();
    dbg!(&ast);
    let sdf: Sdf = Box::new(move |p| make_sdf(&ast, 0.1, 0.2, p));
    println!("sdf: {}", sdf(v3(0.0, 0.0, -100.0)));
    // let plane = sd_plane(v3(0.0, 0.85, 0.3), 10.0, I);
    // let sdf = union(sdf, plane);
    let img_data = render(
        &sdf,
        v3(0.0, -30.0, -40.0),
        // v3(0.0, -10.0, 0.0),
        ZERO3,
        // v3(5.0, 15.0, -60.0),
        // v3(-5.0, -5.0, 0.0),
        &vec![
            Light::new(v3(0.0, 0.0, -26.0), 1.0),
            Light::new(v3(0.0, 30.0, -40.0), 1.0),
            Light::new(v3(5.0, 10.0, -6.0), 0.3),
        ],
        background,
        WIDTH,
        HEIGHT,
        1,
    );
    image::save_buffer("hatch.png", &img_data, WIDTH, HEIGHT, image::ColorType::L8).unwrap();
}
