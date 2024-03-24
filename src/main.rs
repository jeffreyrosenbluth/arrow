use std::collections::HashMap;

use arrow::core::*;
use arrow::eval::*;
use arrow::march::render;

#[allow(unused_imports)]
use arrow::sdf::sd_plane;
use glam::Vec3;

const S: u32 = 1;
const WIDTH: u32 = 1024 / S;
const HEIGHT: u32 = 768 / S;

fn main() {
    use arrow::pratt::*;
    let background = 0.75;
    let examples = examples();
    let (mut input, pos) = *examples.get("else").unwrap();
    let ast = parse(&mut input);
    dbg!(&ast);
    let sdf: Sdf = Box::new(move |p| make_sdf(&ast, 0.1, 0.2, p));
    println!("sdf: {}", sdf(v3(0.0, 0.0, -50.0)));
    // let plane = sd_plane(v3(0.0, 0.85, 0.3), 10.0, I);
    // let sdf = union(sdf, plane);
    let img_data = render(
        &sdf,
        // Camera position
        pos,
        // Look at
        ZERO3,
        &vec![
            Light::new(v3(0.0, 0.0, -50.0), 1.5),
            Light::new(v3(0.0, 10.0, 40.0), 1.0),
        ],
        background,
        WIDTH,
        HEIGHT,
        1, // Anti-aliasing
    );
    image::save_buffer("hatch.png", &img_data, WIDTH, HEIGHT, image::ColorType::L8).unwrap();
}

fn examples<'a>() -> HashMap<&'a str, (&'a str, Vec3)> {
    let pos = v3(0.0, 0.0, -20.0);
    let mut examples = HashMap::new();
    examples.insert("sphere", ("b=B(y-12)-6; L(x,b,z)-6", v3(0.0, 0.0, -20.0)));
    examples.insert(
        "balls8a",
        ("@xyz{$=B($)-6,} L(x,y,z)-5", v3(10.0, 20.0, -25.0)),
    );
    examples.insert(
        "cubes8a",
        (
            "rU(bx3(x,y,z, 5), bx3(x-8, y+5,z,5), bx3(x+8,y-5,z,5), 5)",
            v3(10.0, 20.0, -25.0),
        ),
    );
    examples.insert(
        "box_of_balls",
        (
            "s=1; @5{ [x,y]=r0(x,y), [x,z]=r1(x,z), @xyz{$=B($*2)-8,} s*=.5; } (L(x,y,z)-8)*s",
            v3(0.0, 0.0, -20.0),
        ),
    );
    examples.insert(
        "sponge", 
        ("r=bx3(x,y,z,9),s=1; @3{ @xyz{$=(mod($+9,18)-9)*3,} s/=3, r=k(r,-U(@xyz{bx2($,$$,9),})*s), }r", v3(-20.0, 20.0, -5.0))
    );
    examples.insert("donut", ("don(x,y-2,z,15,2)", v3(0.0, 0.0, -20.0)));
    examples.insert(
        "donuts",
        ("don(x,y-3,mod(z,8)-4,8,1)", (v3(-50.0, 20.0, -20.0))),
    );
    examples.insert("rods", ("L(B(B(x)-3)-3,B(y)-3)-2", pos));
    examples.insert(
        "rounded_box",
        ("bx3(x,y,z,7,4,4)-5", v3(-20.0, 20.0, -20.0)),
    );
    examples.insert("elbow", ("L(k(x,y-10),z)-5", pos));
    examples.insert("fence", ("L(x,TR(y))-.5", v3(10.0, 10.0, -50.0)));
    examples.insert(
        "cross",
        (
            "U( bx3(mod(x,4)-2,y,z,6), bx3(x,y,mod(x,4)-2,6), L(TR(x),y)-1, L(x+20,y-20,z-20)-8)",
            v3(-10.0, 20.0, -10.0),
        ),
    );
    examples.insert(
        "apollonius",
        (
            "s=2.5,h=s/2,d=(s+h)/2,q=20,y-=10,[x,y]=r0(x,y),@xyz{$/=q,}c=1,t=0,@7{@xyz{$=mod($-h,s)-h,}t=d/D([x,y,z],[x,y,z]),@xyzc{$*=t,}}d=L(x,y,z)/c*2.-.025", 
            v3(0.0, 0.0, -30.0)
        ),
    );
    examples.insert("rot_cube", ("[a,b]=r0(x,y-9); bx3(a,b,z,4)-.5", pos));
    examples.insert(
        "hyperplane",
        ("a=(2*x-3*z+6*y)/7,b=(7*x-2*z+26*y)/27,c=(6*z-3*x+22*y)/23,d=0,zz=0;[x,z]=r1(x,z+8),[x,y]=r0(x,y),y-=3,zz=FR(z/26-.55)*26-13,d=SM(9,-12,y+3-z*.3),U(k(k(k(bx3(x,y-5,zz,7,14,7)-1,@abc{d-B(TR($))),}L(x+99,y+445,z+32)-434)", pos),
    );
    examples.insert(
        "desire",
        ("[x,y]=r0(x,y-1), [x,z]=r1(x,z), yb=B(y)-22.5, U(rG(32-k(0-y-13,z-15),TR(x*.25)*4-2+4*SM(0,16,x),4),rG( B(B(L(L(x,z)-16,yb-cl(yb,-8.5,8.5))-8)-4)-2, B(B(L(B(x)-15,B(B(y)-15)-15,B(z)-15)-9)-4)-2,1 ))", pos),
    );
    examples.insert(
        "singularity",
        ("[x,z]=r0(x,z),l=L(x,y,z),n=2.0 * nz(atan2(z,x),Math.acos(y/l),l,.3,0,1),d=l-20+n*5,d=B(d)-5,d=B(d)-1,b=99,@4{b=U(b,bx2(x,y-n*3+10-$*10,100,2+$*.5)),}rU(y+20-B(n)*.2,k(0-b,d)-.4,20)", pos),
    );
    examples.insert(
        "gnarl",
        ("p=B(y-18)-13,n=nz(x,y,z,.2,0,2)*2,q=mod(p,12+n*z)-1.8", pos),
    );
    examples.insert(
        "ondu",
        ("s=.5,y+=6, a=k(y+22,B(z+10*g(x*.005+.2))-16)-4,b=TR(x/40+.2)*40,c; [b,y]=r1(b,y), [y,z]=r0(y,z+15), r=rU( L(x-cl(x,-2,2),b*1.3,z)-3, U( L(x+5,b-1,z)-1.7, L(x+5,b-2,B(z)-1.5)-0.8, bx3(x-5,b-1,z,0.2,0.1,0.2)-0.5, bx3(x+5,b-1,z,1.9,.1,.1)-.5, L(B(x)-3.5,b-cl(b,-4,0),B(z)-1.5)-.8),1.5 )-nz(x,0,z,12,1)*0.15, s=(L(x>7?(mod(x,4)-2)/2:x,x<1?y:b/3+2,B(z)-1.5)-1.8)-nz(x,y,z,.5,1)*2, rG(U(a,g),-s,1)", pos),
    );
    examples.insert(
        "source",
        ("@zy{$f=$+nz(x,y,z,.03,1)*40,} zf+=nz(x,y,z,.1,2)*20, f=L(zf+10,yf)-20, w=y+nz(x,y,z+30,.4,2,1)*2.2+1, g=y+nz(x,y,z,0.1,1), p=min(max(g,-f),max(w,f))+nz(x,y,z,.02,2,2)*4, [ex,ey]=r0(x-18,y-6), [ey,ez]=r1(ey,z-11), es=2, e=bx3(ex,ey,ez,es*1.5,es,.1), sg=.5, @xyz{$g=e$+nz(ex/1.6,ey,ez,1,1,2)*2,} g=1e6, @xy{g=U(g,L(ez-.1,mod($g, sg)-sg/2)-sg*.07),} g=max(g,bx3(ex,ey,ez,es*1.5*.85,es*.85,2)), min(p, e, g, bx3(ex+3,B(ey)-.8,ez, 4,.1,.1), bx3(ex+3,ey,ez, .05, es*1.2,.5),bx3(ex-3,ey,ez, .05, es*.2,.2))", pos),
    );
    examples.insert(
        "spheres",
        ("y-=5,z-=3, r=L(x,y,z), ph=atan2(y,x), th=Math.acos(z/r), n=18,r0=r-33, cs=sin(n*ph)*cos(n*th), c0=L(r0,cs)-.1, c3=L(r0-cl(r0,-2,-1),r/n*(cs-.5))-.05, c4=L(r0-cl(r0,-1,.5),r/n*(cs-.75))-.05, c5=L(r0-cl(r0,-1,1),r/n*(cs-.95))-.025, n=4,r1=1.25*n*sin(th), x=r0, y=r/n*sin(th)*cos(n*ph)*sin(n*th), z=r/n*sin(th)*sin(n*ph)*cos(n*th), c1=L(x,y,z)-r1, r=L(x,y,z), ph=atan2(y,x), th=Math.acos(z/r), zr=r-r1-cl(r-r1,0,.5), n=12, x=r/n*(sin(th)*cos(n*ph)*sin(n*th)-.5), c2=L(x,zr)-.05, U(c0,c1,c2,c3,c4,c5)", pos),
    );
    examples.insert(
        "arctic",
        ("[x,z]=r0(x,z), x+=11, z+=15, y+=10, h=exp(-1.5*B(nz(x,0,z,.1,1))), g=y-10*h-nz(x,0,z,10,1)*0.05, b = y-12, a=rU( L(x-cl(x,-2,2),b*1.3,z)-3, U( L(x+5,b-1,z)-1.7, L(x+5,b-2,B(z)-1.5)-0.8, bx3(x-5,b-1,z,0.2,0.1,0.2)-0.5, bx3(x+5,b-1,z,1.9,.1,.1)-.5, L(B(x)-3.5,b-cl(b,-4,0),B(z)-1.5)-.8),1.5 )-nz(x,0,z,12,1)*0.15, s=(L(x>7?(mod(x,4)-2)/2:x,x<1?y:b/3+2,B(z)-1.5)-1.8)-nz(x,y,z,.5,1)*2, rG(U(a,g),-s,1)", pos),
    );
    examples.insert(
        "quanta",
        ("s=20,[x,z]=r0(x,z),[y,x]=r1(y,x),z+=17,y+=27,i=0,z+=ri(Z(x/s))*70,@xz{$-=nz(x,y,z,.1,i++)*5*i,$i=Z($/s),$=mod($,s)-s/2,}i=ri(xi,zi),j=ri(xi,floor(y/5)),d=i>.1?rU(L(x,z)-1*i-.5*(cos(y/4)+1),bx2(L(x,z)-(cos(floor(y/4))+1)*2,mod(y,4)-2,.1,.2)-.05,1):L(x,mod(y,5)-2.5,z)-G(j,0)*2", pos),
    );
    examples.insert(
        "thepath",
        (
            "@xyz{$m=mod($,20)-10,$i=Z($/20),}d=99,g=.05,y-=20,[z,x]=r0(z,x),n=nz(x,y,z,.1,1),n1=nz(x,y,z,.3,2,3),@4{x-=20,o=$*200+20,e=B(y+n1/2+sin(z*.05+o)*10)-1,e=rG(e,B(z+sin(x*.05+o)*25)-5+n1*2,.2),@xz{$1=mod($+n*10,3)-1.5,}e=rG(e,-(B(z1)-g),.25),e=rG(e,-(B(x1)-g),.25),d=U(d,e),[x,z]=r1(z,x),y+=20,}U(d,ri(xi,yi,zi)>.4&&L(xi,yi,zi)>3?L(xm,ym,zm)-2:10)",
             v3(10.0, 20.0, -25.0)
            )
    );
    examples.insert(
        "mycelia",
        (
            "@xyz{$/=20,} xm=.9,ym=.3,zm=.7, @4{ @xyz{$=B($)-$m,} s=1/scl(L(x,y,z)**3,.1,.1,1), @xyz{$=$*s-$$m,} } L(z,y)-.1",
            v3(10.0, 10.0, -15.0),
        )
    );
    examples.insert(
        "pawns", 
        (
            "i=mod(floor(x/8)+floor(z/8),2),x=mod(x,8)-4,z=mod(z,8)-4,a=L(x,y,z)-1,q=L(x,z),b=max(D([1,.3],[q,y]),-5-y),a=rU(a,b,1),y+=1,a=rU(a,L(x,y*5,z)-.8,1),y+=3,a=rU(a,L(x,y*2,z)-1,.5),y+=1,a=rU(a,L(x,y*3,z)-1.7,0.1),min(a,y+.5*i*nz(x,y,z,8,0)))",
            v3(0.0, 0.0, -20.0)
        )
    );
    examples.insert(
        "plato",
        (
            "d=99,l=10, x-=l*2, y-=l,z+=2.5, @3{ x+=l, a=a0*($+2),s=sin(a),c=cos(a), [x1,y1]=rot(x,y,s,c), a=a1*($+2),s=sin(a),c=cos(a), [x1,z1]=rot(x1,z,s,c), d=rU(d, bx3(x1,y1,z1,4),3), } U(d+.5, L(nz(x,y,z,.1,1,2)-.5, abs(d)-.1)-.4)",
            v3(0.0, 30.0, -10.0)
        )
    );
    examples.insert(
        "sprenkle",
        (
            "y+=7, rU( don(x,y-12,mod(z,8)-4,7+3*SM(9,15,y)+4*nz(x,y,z,.3,1),2.7-2*SM(9,15,y)+nz(x,y,z,.2,2)), L(x,y+83)-90,1)-.1*SM(0,.15,B(nz(x,y,z,2,0,3)))",
            v3(0.0, 0.0, -40.0)
        )
    );
    examples.insert(
        "target",
        (
            "n=nz(x,y,z,.4,0,2)*4; U(L(k(x,-12-z),-k(2-y,-19-x))-.5, L(TR(L(x,y)/4)*4+n,z-cl(-3,3,z))-.5-1.5*SM(3,-3,y)-3*SM(-3,-9,y) )",
            v3(-3.0, 5.0, -29.0)
        )
    );
    examples.insert(
        "toy", 
        (
            "s=1,x1=x-.25,y1=y-4.6,z1=z; x=x1,y=y1, @4{ [x,y]=r0(x,y), [x,z]=r1(x,z), @xyz{$=sB($*2,.1)-4,} s*=.4, } rU( rU( bx3(x,y,z,4)*s-0.01, rG( L(x,y)-1.75, bx3(x,y,z,4)*0.2), 0.75), don(x1,y1,z1,9,.75),2.75)", 
            v3(5.0, -5.0, -15.0)
        )
    );
    examples.insert(
        "shai_hulud",
        (
            "[x,z]=r0(x,z),m=nz(x,y,z,.4,1),r=30,t=.2,v=atan2(z,y-7),u=L(z,y-7), min( min(max(don(x+5,y+5.5,z+10,.5,.05),y+5.5), L(x*3+15,y/4+2.4,z*3+30)-1), rU(y-20+nz(x/3,y/3,z/2,1,1)+SM(30,0,15-B(x+sin(z/2)+sin(z/6)+sin(z/8)-4))*15+SM(30,0,15-B(mod(z+x+sin(x/2)+sin(x/6)+sin(x/8),60)-35))*15, max( -L(y-7,z,x+3)+4+sin(v*3-2), min( don(x-90,y+23,z,20,5+m), max(abs(don(x,y+r-7,z,r,5+m))-t,-x), max(abs(L(x,y-7,z)-5-m)-t,x), L(u/3-1,B(B(B(x*2-4)-2)-2)-1,mod(x/2+v*9,1.6)-.8)-.34) ),2) )",
            v3(-8.0, 5.0, 40.0)
        )
    );
    examples.insert(
        "else",
        (
           "y-=1, r=bx3(x,y,z,9)-2,s=1,ti=U(L(x,y)-.6, L(y,z)-.6,L(z,x)-.6); @4{ @xyz{$=(mod($+9,18)-9)*3,} s/=3, r=k(r+s,-U(@xyz{L($,$$)-12,})*s)-s, } U(r, ti)",
            v3(0.0, 20.0, -20.0) 
        )
    );
    examples.insert(
        "temple",
        (
            "d=99, [y,z]=r1(y,z), f=y+B(nz(x,z,1,.0,3))*5, @5{ [x,y,z]=[y,z,x], [x,z]=r0(x,z), [x,z]=r1(x,z), @xyz{$=sB($,2)-3,} d=rU(d, don(y,z,x,5,.5+$*.2), 1), } rU(f, L(d,nz(x,y,z,.5,1))-.1, .5)",
            v3(0.0, 0.0, -30.0)
        )
    );
    examples
}
