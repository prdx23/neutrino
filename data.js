let data3dCube = [
    // front
    -1, -1, +1,
    +1, -1, +1,
    +1, +1, +1,

    -1, -1, +1,
    +1, +1, +1,
    -1, +1, +1,

    // right
    +1, -1, +1,
    +1, -1, -1,
    +1, +1, -1,

    +1, -1, +1,
    +1, +1, -1,
    +1, +1, +1,

    // back
    +1, -1, -1,
    -1, -1, -1,
    +1, +1, -1,

    +1, +1, -1,
    -1, -1, -1,
    -1, +1, -1,

    // left
    -1, -1, -1,
    -1, -1, +1,
    -1, +1, +1,

    -1, -1, -1,
    -1, +1, +1,
    -1, +1, -1,

    // top
    -1, +1, +1,
    +1, +1, +1,
    +1, +1, -1,

    -1, +1, +1,
    +1, +1, -1,
    -1, +1, -1,

    // bottom
    -1, -1, +1,
    +1, -1, -1,
    +1, -1, +1,

    -1, -1, +1,
    -1, -1, -1,
    +1, -1, -1,
]

let data3dCubeColor = [
    // front
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    80,  70, 120,
    80,  70, 120,
    80,  70, 120,

    // right
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    160, 160, 220,
    160, 160, 220,
    160, 160, 220,

    // back
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    80,  70, 120,
    80,  70, 120,
    80,  70, 120,

    // left
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    160, 160, 220,
    160, 160, 220,
    160, 160, 220,

    // top
    76, 170, 100,
    76, 170, 100,
    76, 170, 100,
    140, 170, 80,
    140, 170, 80,
    140, 170, 80,

    // bottom
    76, 170, 100,
    76, 170, 100,
    76, 170, 100,
    140, 170, 80,
    140, 170, 80,
    140, 170, 80,
]

let shaders = {
    main: {
        vertex: document.getElementById('vertex-shader').textContent,
        fragment: document.getElementById('fragment-shader').textContent,
    },
    cube: {
        vertex: document.getElementById('vertex-shader').textContent,
        fragment: document.getElementById('fragment-shader').textContent,
    },
}


let buffers = {
    cubeVertices: {
        data: new Float32Array(data3dCube),
        bufferType: 'STATIC_DRAW',
        size: 3,
        type: 'FLOAT',
        normalize: false,
    },

    cubeColors: {
        data: new Uint8Array(data3dCubeColor),
        bufferType: 'STATIC_DRAW',
        size: 3,
        type: 'UNSIGNED_BYTE',
        normalize: true,
    },
}


let objects = {

    cube1: {
        shader: 'main',
        count: 6 * 6,
        attributes: {
            'a_position': 'cubeVertices',
            'a_color': 'cubeColors',
        },
        uniformBlocks: {
            'objectData': ['u_matrix'],
        },
        uniforms: {
            'u_matrix': {
                update: (viewProjectionMatrix, worldMatrix) => {
                    let matrix = m4.identity()
                    matrix = m4.multiply(matrix, viewProjectionMatrix)
                    matrix = m4.scale(matrix, 50, 50, 50)
                    matrix = m4.multiply(matrix, worldMatrix)
                    return matrix
                },
            },
        },
    },

    cube2: {
        shader: 'main',
        count: 6 * 6,
        attributes: {
            'a_position': 'cubeVertices',
            'a_color': 'cubeColors',
        },

        uniformBlocks: {
            'objectData': ['u_matrix'],
        },
        uniforms: {
            'u_matrix': {
                update: (viewProjectionMatrix, worldMatrix) => {
                    let matrix = m4.identity()
                    matrix = m4.multiply(matrix, viewProjectionMatrix)
                    matrix = m4.translate(matrix, 250, 0, 0)
                    matrix = m4.scale(matrix, 50, 50, 50)
                    matrix = m4.multiply(matrix, worldMatrix)
                    return matrix
                },
            },
        },
    },

    cube3: {
        shader: 'cube',
        count: 6 * 6,
        attributes: {
            'a_position': 'cubeVertices',
            'a_color': 'cubeColors',
        },
        uniformBlocks: {
            'objectData': ['u_matrix'],
        },
        uniforms: {
            'u_matrix': {
                update: (viewProjectionMatrix, worldMatrix) => {
                    let matrix = m4.identity()
                    matrix = m4.multiply(matrix, viewProjectionMatrix)
                    matrix = m4.translate(matrix, -250, 0, 0)
                    matrix = m4.scale(matrix, 50, 50, 50)
                    matrix = m4.multiply(matrix, worldMatrix)
                    return matrix
                },
            },
        },
    },
}

let data3dF = [
    // left column front
    0,   0,  0,
    0, 150,  0,
    30,   0,  0,
    0, 150,  0,
    30, 150,  0,
    30,   0,  0,
    // top rung front
    30,   0,  0,
    30,  30,  0,
    100,   0,  0,
    30,  30,  0,
    100,  30,  0,
    100,   0,  0,

    // middle rung front
    30,  60,  0,
    30,  90,  0,
    67,  60,  0,
    30,  90,  0,
    67,  90,  0,
    67,  60,  0,

    // left column back
    0,   0,  30,
    30,   0,  30,
    0, 150,  30,
    0, 150,  30,
    30,   0,  30,
    30, 150,  30,

    // top rung back
    30,   0,  30,
    100,   0,  30,
    30,  30,  30,
    30,  30,  30,
    100,   0,  30,
    100,  30,  30,

    // middle rung back
    30,  60,  30,
    67,  60,  30,
    30,  90,  30,
    30,  90,  30,
    67,  60,  30,
    67,  90,  30,

    // top
    0,   0,   0,
    100,   0,   0,
    100,   0,  30,
    0,   0,   0,
    100,   0,  30,
    0,   0,  30,

    // top rung right
    100,   0,   0,
    100,  30,   0,
    100,  30,  30,
    100,   0,   0,
    100,  30,  30,
    100,   0,  30,

    // under top rung
    30,   30,   0,
    30,   30,  30,
    100,  30,  30,
    30,   30,   0,
    100,  30,  30,
    100,  30,   0,

    // between top rung and middle
    30,   30,   0,
    30,   60,  30,
    30,   30,  30,
    30,   30,   0,
    30,   60,   0,
    30,   60,  30,

    // top of middle rung
    30,   60,   0,
    67,   60,  30,
    30,   60,  30,
    30,   60,   0,
    67,   60,   0,
    67,   60,  30,

    // right of middle rung
    67,   60,   0,
    67,   90,  30,
    67,   60,  30,
    67,   60,   0,
    67,   90,   0,
    67,   90,  30,

    // bottom of middle rung.
    30,   90,   0,
    30,   90,  30,
    67,   90,  30,
    30,   90,   0,
    67,   90,  30,
    67,   90,   0,

    // right of bottom
    30,   90,   0,
    30,  150,  30,
    30,   90,  30,
    30,   90,   0,
    30,  150,   0,
    30,  150,  30,

    // bottom
    0,   150,   0,
    0,   150,  30,
    30,  150,  30,
    0,   150,   0,
    30,  150,  30,
    30,  150,   0,

    // left side
    0,   0,   0,
    0,   0,  30,
    0, 150,  30,
    0,   0,   0,
    0, 150,  30,
    0, 150,   0,
]

let data3dFColor = [
    // left column front
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,

    // top rung front
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,

    // middle rung front
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,
    200,  70, 120,

    // left column back
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,

    // top rung back
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,

    // middle rung back
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,
    80, 70, 200,

    // top
    70, 200, 210,
    70, 200, 210,
    70, 200, 210,
    70, 200, 210,
    70, 200, 210,
    70, 200, 210,

    // top rung right
    200, 200, 70,
    200, 200, 70,
    200, 200, 70,
    200, 200, 70,
    200, 200, 70,
    200, 200, 70,

    // under top rung
    210, 100, 70,
    210, 100, 70,
    210, 100, 70,
    210, 100, 70,
    210, 100, 70,
    210, 100, 70,

    // between top rung and middle
    210, 160, 70,
    210, 160, 70,
    210, 160, 70,
    210, 160, 70,
    210, 160, 70,
    210, 160, 70,

    // top of middle rung
    70, 180, 210,
    70, 180, 210,
    70, 180, 210,
    70, 180, 210,
    70, 180, 210,
    70, 180, 210,

    // right of middle rung
    100, 70, 210,
    100, 70, 210,
    100, 70, 210,
    100, 70, 210,
    100, 70, 210,
    100, 70, 210,

    // bottom of middle rung.
    76, 210, 100,
    76, 210, 100,
    76, 210, 100,
    76, 210, 100,
    76, 210, 100,
    76, 210, 100,

    // right of bottom
    140, 210, 80,
    140, 210, 80,
    140, 210, 80,
    140, 210, 80,
    140, 210, 80,
    140, 210, 80,

    // bottom
    90, 130, 110,
    90, 130, 110,
    90, 130, 110,
    90, 130, 110,
    90, 130, 110,
    90, 130, 110,

    // left side
    160, 160, 220,
    160, 160, 220,
    160, 160, 220,
    160, 160, 220,
    160, 160, 220,
    160, 160, 220,
]
