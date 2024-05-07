fn maxf(a, b) {
    if a > b {
        a
    } else {
        b
    }
}

fn minf(a, b) {
    if a < b {
        a
    } else {
        b
    }
}

fn log2(x) {
    log(x, 2.0)
}

fn fract(x) {
    fraction(x)
}

fn trunc(x) {
    floor(x)
}

fn ceil(x) {
    ceiling(x)
}

fn modulo(x, y) {
    ((x % y + y) % y)
}

fn fake_sine(x) {
    2.0 * abs(x - floor(x) - 0.5) * x * (6.0 - 4.0 * x) - 1.0
}

fn length(x, y, z) {
    sqrt(x * x + y * y + z * z)
}

fn length(x, y) {
    sqrt(x * x + y * y)
}

fn distance(x, y, z, a, b, c) {
    length(x - a, y - b, z - c)
}

fn distance(x, y, a, b) {
    length(x - a, y - b)
}

fn dot(x, y, z, a, b, c) {
    x * a + y * b + z * c
}

fn dot(x, y, a, b) {
    x * a + y * b
}

fn cross(x, y, z, a, b, c) {
    [y * c - z * b, z * a - x * c, x * b - y * a]
} 

fn normalize(x, y, z) {
    let l = length(x, y, z);
    [x / l, y / l, z / l]
}

fn union(xs) {
    let m = 1000000000.0;
    for x in xs {
        let m = minf(m, x);
    }
    m
}

fn box3(x, y, z, a, b, c) {
    let qx = abs(x) - a;
    let qy = abs(y) - b;
    let qz = abs(z) - c;
    minf(maxf(qy, maxf(qz, qx)), 0.0) + length(maxf(qx, 0.0), maxf(qy, 0.0), maxf(qz, 0.0))
}

fn box3(x, y, z, a) {
    let qx = abs(x) - a;
    let qy = abs(y) - a;
    let qz = abs(z) - a;
    minf(maxf(qy, maxf(qz, qx)), 0.0) + length(maxf(qx, 0.0), maxf(qy, 0.0), maxf(qz, 0.0))
}

fn box2(x, y, a, b) {
    let qx = abs(x) - a;
    let qy = abs(y) - b;
    minf(maxf(qy, qx), 0.0) + length(maxf(qx, 0.0), maxf(qy, 0.0))
}

fn box2(x, y, a) {
    let qx = abs(x) - a;
    let qy = abs(y) - a;
    minf(maxf(qy, qx), 0.0) + length(maxf(qx, 0.0), maxf(qy, 0.0))
}