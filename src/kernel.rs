pub const AVG: [[f64; 3]; 3] = [
    [0.11111,0.11111,0.11111],
    [0.11111,0.11111,0.11111],
    [0.11111,0.11111,0.11111],
];

pub const GAUSSIAN: [[f64; 3]; 3] = [
    [0.0625,0.125,0.0625],
    [0.125,0.25,0.125],
    [0.0625,0.125,0.0625],
];

pub const PREWITT_X: [[f64; 3]; 3] = [
    [-1.0, 0.0, 1.0],
    [-1.0, 0.0, 1.0],
    [-1.0, 0.0, 1.0],
];

pub const PREWITT_Y: [[f64; 3]; 3] = [
    [-1.0, -1.0, -1.0],
    [0.0, 0.0, 0.0],
    [1.0, 1.0, 1.0],
];

pub const SEBEL_X: [[f64; 3]; 3] = [
    [-1.0, 0.0, 1.0],
    [-2.0, 0.0, 2.0],
    [-1.0, 0.0, 1.0],
];

pub const SEBEL_Y: [[f64; 3]; 3] = [
    [-1.0, -2.0, -1.0],
    [0.0, 0.0, 0.0],
    [1.0, 2.0, 1.0],
];

pub const LAPLACIAN: [[f64; 3]; 3] = [
    [-1.0, -1.0, -1.0],
    [-1.0, 8.0, -1.0],
    [-1.0, -1.0, -1.0],
];

pub const LAPLACIAN_HPF: [[f64; 3]; 3] = [
    [-1.0, -1.0, -1.0],
    [-1.0, 9.0, -1.0],
    [-1.0, -1.0, -1.0],
];
