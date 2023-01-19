const width = 800
const height = 800


function createProgram(gl, vsSource, fsSource) {

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

    let buffers = genBuffers(gl)

    for( let [name, shader] of Object.entries(shaders) ) {
        shaders[name].program = createProgram(gl, shader.vertex, shader.fragment)
        shaders[name].objects = []
    }

    for( let [name, bufferData] of Object.entries(buffers) ) {
        let buffer = gl.createBuffer()
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer)
        gl.bufferData(gl.ARRAY_BUFFER, bufferData.data, bufferData.bufferType)
        buffers[name].buffer = buffer
        gl.bindBuffer(gl.ARRAY_BUFFER, null)
    }

    let uboIndex = 0
    for( let [name, object] of Object.entries(objects) ) {
        let program = shaders[object.shader].program

        let vao = gl.createVertexArray()
        gl.bindVertexArray(vao)
        objects[name].vao = vao

        for( let attrib of Object.keys(object.attributes) ) {
            let buffer = buffers[object.attributes[attrib]]

            let location = gl.getAttribLocation(program, attrib)
            gl.enableVertexAttribArray(location)

            gl.bindBuffer(gl.ARRAY_BUFFER, buffer.buffer)
            gl.vertexAttribPointer(
                location, buffer.size, buffer.type, buffer.normalize, 0, 0,
            )
            gl.bindBuffer(gl.ARRAY_BUFFER, null)

        }

        // for( let uniformName of Object.keys(object.uniforms) ) {
        //     object.uniforms[uniformName].location = gl.getUniformLocation(
        //         program, uniformName
        //     )
        // }

        for( let [uniformBlockName, uniformBlockData] of Object.entries(object.uniforms) ) {
            let index = gl.getUniformBlockIndex(program, uniformBlockName)
            let size = gl.getActiveUniformBlockParameter(
                program, index, gl.UNIFORM_BLOCK_DATA_SIZE
            )

            let buffer = gl.createBuffer()
            gl.bindBuffer(gl.UNIFORM_BUFFER, buffer)
            gl.bufferData(gl.UNIFORM_BUFFER, size, gl.DYNAMIC_DRAW)
            gl.bindBuffer(gl.UNIFORM_BUFFER, null)

            uboIndex += 1
            gl.bindBufferBase(gl.UNIFORM_BUFFER, uboIndex, buffer)

            let variableNames = Object.keys(uniformBlockData)
            let variableIndices = gl.getUniformIndices(program, variableNames)
            let variableOffsets = gl.getActiveUniforms(
                program, variableIndices, gl.UNIFORM_OFFSET
            )

            for( let [i, variable] of variableNames.entries() ) {
                object.uniforms[uniformBlockName][variable].index = variableIndices[i]
                object.uniforms[uniformBlockName][variable].offset = variableOffsets[i]
            }

            object.uniforms[uniformBlockName].buffer = buffer
            object.uniforms[uniformBlockName].index = index
            object.uniforms[uniformBlockName].uboIndex = uboIndex
            // gl.uniformBlockBinding(program, index, uboIndex)
        }

        shaders[object.shader].objects.push(objects[name])
        gl.bindVertexArray(null)
    }

    gl.enable(gl.CULL_FACE)
    gl.enable(gl.DEPTH_TEST)

    let x = 0

    function render() {
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height)
        gl.clearColor(0, 0, 0, 1)
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)


        let cameraMatrix = m4.identity()
        cameraMatrix = m4.translate(cameraMatrix, 0, 300, 1800)
        cameraMatrix = m4.lookAt(
            [cameraMatrix[12], cameraMatrix[13], cameraMatrix[14]],
            [0, 0, 0],
            [0, 1, 0],
        )

        let viewMatrix = m4.inverse(cameraMatrix)

        let projectionMatrix = m4.perspective(
            30 * Math.PI / 180,
            gl.canvas.clientWidth / gl.canvas.clientHeight,
            1, 2000,
        )

        let viewProjectionMatrix = m4.multiply(projectionMatrix, viewMatrix)


        let i = 0
        for( let shader of Object.values(shaders) ) {
            gl.useProgram(shader.program)

            for( let object of Object.values(shader.objects) ) {
                i += 1
                gl.bindVertexArray(object.vao)


                let objectMatrix = m4.identity()
                objectMatrix = m4.yRotate(objectMatrix, -x * 0.5 * i * Math.PI / 180)
                objectMatrix = m4.xRotate(objectMatrix, -x * 0.5 * i * Math.PI / 180)
                // object.uniforms['u_matrix'].update(
                //     gl,
                //     object.uniforms['u_matrix'].location,
                //     viewProjectionMatrix,
                //     objectMatrix,
                // )
                let matrix = object.uniforms.objectData.u_matrix.update(
                    viewProjectionMatrix, objectMatrix
                )

                gl.bindBuffer(gl.UNIFORM_BUFFER, object.uniforms.objectData.buffer)
                gl.bufferSubData(
                    gl.UNIFORM_BUFFER,
                    object.uniforms.objectData.u_matrix.offset,
                    new Float32Array(matrix),
                    0
                )
                gl.uniformBlockBinding(
                    shader.program,
                    object.uniforms.objectData.index,
                    object.uniforms.objectData.uboIndex,
                )

                gl.drawArrays(gl.TRIANGLES, 0, object.count)
                gl.bindVertexArray(null)
                gl.bindBuffer(gl.UNIFORM_BUFFER, null)
            }

        }

        x += 1
        requestAnimationFrame(render)
    }
    render()

}
