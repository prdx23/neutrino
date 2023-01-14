const width = 800
const height = 800


function createShader(gl, type, source) {
    let shader = gl.createShader(type)
    gl.shaderSource(shader, source)
    gl.compileShader(shader)

    if( !gl.getShaderParameter(shader, gl.COMPILE_STATUS) ) {
        console.log('Shader Error: ', gl.getShaderInfoLog(shader))
        gl.deleteShader(shader)
    }

    return shader
}



function createProgram(gl, vsSource, fsSource, attributes) {
    let vertexShader = createShader(gl, gl.VERTEX_SHADER, vsSource)
    let fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fsSource)

    let program = gl.createProgram()
    gl.attachShader(program, vertexShader)
    gl.attachShader(program, fragmentShader)
    gl.linkProgram(program)

    if( !gl.getProgramParameter(program, gl.LINK_STATUS) ) {
        console.log('Program Error: ', gl.getProgramInfoLog(program))
        gl.deleteProgram(program)
    }

    return program
}


function loadBuffer(gl, data, drawType) {
    let buffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer)
    gl.bufferData(gl.ARRAY_BUFFER, data, drawType)
    return buffer
}



init()


function init() {
    const canvas = document.getElementById('canvas')
    canvas.width = width
    canvas.height = height
    const gl = canvas.getContext('webgl2')

    if( !gl ) {
        alert('webgl2 not available!')
        return
    }

    let vsSource = document.getElementById('vertex-shader').textContent
    let fsSource = document.getElementById('fragment-shader').textContent
    let program = createProgram(gl, vsSource, fsSource)


    let vao = gl.createVertexArray()
    gl.bindVertexArray(vao)


    let positions = [
        200, 200,
        200, 400,
        400, 400,
    ]
    loadBuffer(gl, new Float32Array(positions), gl.STATIC_DRAW)

    let positionAttributeLocation = gl.getAttribLocation(program, 'a_position')
    gl.enableVertexAttribArray(positionAttributeLocation)
    gl.vertexAttribPointer(
        positionAttributeLocation,
        2,           // size,
        gl.FLOAT,    // type,
        false,       // normalize,
        0,           // stride,
        0,           // offset
    )

    let colors = [
        255, 0, 0,
        0, 255, 0,
        0, 0, 255,
    ]
    loadBuffer(gl, new Uint8Array(colors), gl.STATIC_DRAW)

    let colorAttributeLocation = gl.getAttribLocation(program, 'a_color')
    gl.enableVertexAttribArray(colorAttributeLocation)
    gl.vertexAttribPointer(
        colorAttributeLocation,
        3,           // size,
        gl.UNSIGNED_BYTE,    // type,
        true,       // normalize,
        0,           // stride,
        0,           // offset
    )


    let resolutionUniformLocation = gl.getUniformLocation(program, 'u_resolution')

    // render

    gl.viewport(0, 0, gl.canvas.width, gl.canvas.height)
    gl.clearColor(0, 0, 0, 1)
    gl.clear(gl.COLOR_BUFFER_BIT)

    gl.useProgram(program)
    gl.bindVertexArray(vao)

    gl.uniform2f(resolutionUniformLocation, gl.canvas.width, gl.canvas.height)

    gl.drawArrays(
        gl.TRIANGLES,  // primitive type
        0,             // offset
        3,             // count
    )
}









